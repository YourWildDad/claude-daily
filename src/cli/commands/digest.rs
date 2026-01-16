use anyhow::{Context, Result};
use std::process::{Command, Stdio};

use crate::archive::ArchiveManager;
use crate::config::load_config;
use crate::summarizer::SummarizerEngine;

/// Run the digest command - generate daily summary from sessions
pub async fn run(date: Option<String>, foreground: bool) -> Result<()> {
    let config = load_config()?;

    // Determine target date
    let target_date = date.unwrap_or_else(|| {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    });

    let manager = ArchiveManager::new(config.clone());

    // Check if there are sessions to digest
    let sessions = manager.list_sessions(&target_date)?;
    if sessions.is_empty() {
        eprintln!("[daily] No sessions found for {}", target_date);
        return Ok(());
    }

    if !foreground {
        // Background mode: spawn detached process
        eprintln!(
            "[daily] Starting background digest for {} ({} sessions)",
            target_date,
            sessions.len()
        );

        let exe = std::env::current_exe()
            .context("Failed to get current executable")?;

        Command::new(&exe)
            .args(["digest", "--date", &target_date, "--foreground"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn background digest process")?;

        eprintln!("[daily] Background digest started");
        return Ok(());
    }

    // Foreground mode: perform the digest
    eprintln!(
        "[daily] Digesting {} sessions for {}...",
        sessions.len(),
        target_date
    );

    let engine = SummarizerEngine::new(config.clone());

    // Generate daily summary from all sessions
    match engine.update_daily_summary(&target_date).await {
        Ok(summary) => {
            let summary_path = summary.save(&config)?;
            eprintln!("[daily] Daily summary created: {}", summary_path.display());

            // Delete session files after successful digest
            match manager.delete_sessions(&target_date) {
                Ok(deleted) => {
                    eprintln!(
                        "[daily] Cleaned up {} session files",
                        deleted.len()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "[daily] Warning: Failed to cleanup session files: {}",
                        e
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("[daily] Error: Failed to create daily summary: {}", e);
            eprintln!("[daily] Session files preserved for retry");
            return Err(e);
        }
    }

    eprintln!("[daily] Digest complete!");
    Ok(())
}
