use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::config::load_config;
use crate::jobs::JobManager;
use crate::summarizer::SummarizerEngine;

/// Manually trigger summarization of a transcript
pub async fn run(
    transcript: PathBuf,
    task_name: Option<String>,
    foreground: bool,
    job_id: Option<String>,
) -> Result<()> {
    let config = load_config()?;

    // Generate task name if not provided
    let task_name = task_name.unwrap_or_else(|| {
        let timestamp = chrono::Local::now().format("%H%M%S");
        format!("session-{}", timestamp)
    });

    // Get working directory from transcript path or current dir
    let cwd = transcript
        .parent()
        .and_then(|p| p.parent()) // Go up from transcript to project
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string())
        });

    if !foreground {
        // Background mode: spawn detached process
        eprintln!(
            "[daily] Starting background summarization for: {}",
            task_name
        );

        // Re-invoke ourselves in foreground mode as a detached process
        let exe = std::env::current_exe().context("Failed to get current executable")?;

        let transcript_str = transcript.to_string_lossy().to_string();

        // Spawn detached background process
        #[cfg(unix)]
        {
            // Use nohup-style spawning on Unix
            Command::new(&exe)
                .args([
                    "summarize",
                    "--transcript",
                    &transcript_str,
                    "--task-name",
                    &task_name,
                    "--foreground",
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Failed to spawn background process")?;
        }

        #[cfg(windows)]
        {
            Command::new(&exe)
                .args([
                    "summarize",
                    "--transcript",
                    &transcript_str,
                    "--task-name",
                    &task_name,
                    "--foreground",
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Failed to spawn background process")?;
        }

        eprintln!("[daily] Background summarization started");
        return Ok(());
    }

    // Foreground mode: do the actual summarization
    eprintln!("[daily] Summarizing session: {}", task_name);

    // Initialize job manager for status updates
    let job_manager = JobManager::new(&config).ok();

    // Run summarization with job status tracking
    let result = run_summarization(&config, &transcript, &task_name, &cwd).await;

    // Update job status based on result
    if let (Some(ref manager), Some(ref id)) = (&job_manager, &job_id) {
        match &result {
            Ok(_) => {
                if let Err(e) = manager.mark_completed(id) {
                    eprintln!("[daily] Warning: Failed to update job status: {}", e);
                }
            }
            Err(e) => {
                if let Err(update_err) = manager.mark_failed(id, &e.to_string()) {
                    eprintln!(
                        "[daily] Warning: Failed to update job status: {}",
                        update_err
                    );
                }
            }
        }

        // Truncate log if needed
        let _ = manager.truncate_log_if_needed(id);
    }

    result
}

/// Run the actual summarization logic
async fn run_summarization(
    config: &crate::config::Config,
    transcript: &PathBuf,
    task_name: &str,
    cwd: &str,
) -> Result<()> {
    let engine = SummarizerEngine::new(config.clone());

    // Summarize the session
    let archive = engine
        .summarize_session(transcript, task_name, cwd)
        .await
        .context("Failed to summarize session")?;

    // Save the archive
    let archive_path = archive.save(config)?;
    eprintln!("[daily] Session archived: {}", archive_path.display());

    // Note: Daily summary is now generated via `daily digest` command
    // either manually or auto-triggered on session start

    eprintln!("[daily] Summarization complete!");

    Ok(())
}
