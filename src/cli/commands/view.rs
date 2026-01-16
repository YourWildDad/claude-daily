use anyhow::Result;
use colored::*;

use crate::config::load_config;
use crate::archive::ArchiveManager;

/// View archives
pub async fn run(date: Option<String>, summary_only: bool, list: bool) -> Result<()> {
    let config = load_config()?;
    let manager = ArchiveManager::new(config);

    // Determine which date to view
    let view_date = date.unwrap_or_else(|| {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    });

    if list {
        // List all sessions for the date
        return list_sessions(&manager, &view_date).await;
    }

    if summary_only {
        // Show only the daily summary
        return show_daily_summary(&manager, &view_date).await;
    }

    // Show full daily archive
    show_full_archive(&manager, &view_date).await
}

async fn list_sessions(manager: &ArchiveManager, date: &str) -> Result<()> {
    let sessions = manager.list_sessions(date)?;

    if sessions.is_empty() {
        println!("{}", format!("No sessions found for {}", date).yellow());
        return Ok(());
    }

    println!("{}", format!("Sessions for {}:", date).cyan().bold());
    println!();

    for (i, session) in sessions.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().green(), session);
    }

    println!();
    println!("Total: {} sessions", sessions.len());

    Ok(())
}

async fn show_daily_summary(manager: &ArchiveManager, date: &str) -> Result<()> {
    match manager.read_daily_summary(date) {
        Ok(content) => {
            println!("{}", format!("Daily Summary - {}", date).cyan().bold());
            println!("{}", "=".repeat(50));
            println!();
            println!("{}", content);
            Ok(())
        }
        Err(_) => {
            println!("{}", format!("No daily summary found for {}", date).yellow());
            Ok(())
        }
    }
}

async fn show_full_archive(manager: &ArchiveManager, date: &str) -> Result<()> {
    // Show daily summary first
    println!("{}", format!("Daily Archive - {}", date).cyan().bold());
    println!("{}", "=".repeat(50));
    println!();

    // Try to show daily summary
    if let Ok(content) = manager.read_daily_summary(date) {
        // Extract just the overview section for brevity
        if let Some(start) = content.find("## Overview") {
            let after_header = &content[start..];
            if let Some(end) = after_header.find("\n## Sessions") {
                println!("{}", &after_header[..end]);
            } else {
                println!("{}", after_header.lines().take(10).collect::<Vec<_>>().join("\n"));
            }
        }
    }

    println!();

    // List sessions
    let sessions = manager.list_sessions(date)?;

    if sessions.is_empty() {
        println!("{}", "No sessions archived yet.".yellow());
        return Ok(());
    }

    println!("{}", "Sessions:".green().bold());
    println!();

    for session in &sessions {
        println!("  {} {}", "●".green(), session);

        // Show brief summary if available
        if let Ok(content) = manager.read_session(date, session) {
            // Extract first line of summary
            if let Some(start) = content.find("## Summary") {
                let after_header = &content[start + 11..];
                let first_line = after_header.lines().next().unwrap_or("");
                if !first_line.is_empty() && !first_line.starts_with('#') {
                    let truncated: String = first_line.chars().take(80).collect();
                    println!("    {}", truncated.dimmed());
                }
            }
        }
    }

    println!();
    println!("Use {} to see a specific session", "daily view --date DATE".cyan());

    Ok(())
}
