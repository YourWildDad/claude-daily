use anyhow::Result;
use std::process::{Command, Stdio};

use crate::config::load_config;
use crate::hooks::read_hook_input;

/// Handle SessionEnd hook from Claude Code
/// Spawns background process for summarization
pub async fn handle() -> Result<()> {
    let config = load_config()?;

    // Check if hooks are enabled
    if !config.hooks.enable_session_end {
        return Ok(());
    }

    // Read hook input from stdin
    let input = match read_hook_input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("[daily] Failed to read hook input: {}", e);
            return Ok(()); // Don't block session exit
        }
    };

    // Only archive on user_exit (normal completion)
    if input.reason.as_deref() != Some("user_exit") {
        eprintln!(
            "[daily] Session ended with {:?}, skipping archive",
            input.reason
        );
        return Ok(());
    }

    // Generate task name from working directory
    let task_name = generate_task_name(&input.cwd);

    // Spawn background process for summarization
    // This ensures Claude Code can exit immediately
    let transcript_path = input.transcript_path.to_string_lossy().to_string();

    match Command::new("daily")
        .args([
            "summarize",
            "--transcript",
            &transcript_path,
            "--task-name",
            &task_name,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => {
            eprintln!(
                "[daily] Background summarization started (PID: {})",
                child.id()
            );
        }
        Err(e) => {
            eprintln!("[daily] Failed to spawn summarization process: {}", e);
        }
    }

    Ok(())
}

/// Generate a task name from the working directory
fn generate_task_name(cwd: &std::path::Path) -> String {
    // Extract project name from path
    let name = cwd
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unnamed-session".into());

    // Add timestamp to make unique
    let timestamp = chrono::Local::now().format("%H%M%S");
    format!("{}-{}", name, timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_task_name() {
        let cwd = PathBuf::from("/home/user/my-project");
        let name = generate_task_name(&cwd);
        assert!(name.starts_with("my-project-"));
    }

    #[test]
    fn test_generate_task_name_empty() {
        let cwd = PathBuf::from("/");
        let name = generate_task_name(&cwd);
        // Root directory has no file_name, should use default
        assert!(name.contains("-"));
    }
}
