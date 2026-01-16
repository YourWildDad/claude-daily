use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "daily")]
#[command(about = "Daily Context Archive System for Claude Code")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Config file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration
    Init {
        /// Storage path (default: ~/.claude/daily)
        #[arg(short, long)]
        storage_path: Option<PathBuf>,
    },

    /// Handle Claude Code hooks (internal use)
    Hook {
        #[command(subcommand)]
        hook_type: HookType,
    },

    /// View today's archive
    View {
        /// Date to view (default: today, format: yyyy-mm-dd)
        #[arg(short, long)]
        date: Option<String>,

        /// Show daily summary only
        #[arg(long)]
        summary_only: bool,

        /// List all sessions for the day
        #[arg(long)]
        list: bool,
    },

    /// Manually trigger summarization
    Summarize {
        /// Session transcript path
        #[arg(short, long)]
        transcript: PathBuf,

        /// Task name for the archive
        #[arg(short = 'n', long)]
        task_name: Option<String>,

        /// Run in foreground (not background)
        #[arg(long)]
        foreground: bool,
    },

    /// Extract skill from archive
    ExtractSkill {
        /// Date to search (default: today)
        #[arg(short, long)]
        date: Option<String>,

        /// Session to extract from
        #[arg(short, long)]
        session: Option<String>,

        /// Output directory for skill
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Extract command from archive
    ExtractCommand {
        /// Date to search (default: today)
        #[arg(short, long)]
        date: Option<String>,

        /// Session to extract from
        #[arg(short, long)]
        session: Option<String>,

        /// Output directory for command
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Show or update configuration
    Config {
        /// Set storage path
        #[arg(long)]
        set_storage: Option<PathBuf>,

        /// Show current config
        #[arg(long)]
        show: bool,
    },

    /// Install plugin to Claude Code
    Install {
        /// Scope: user or project
        #[arg(short, long, default_value = "user")]
        scope: String,
    },
}

#[derive(Subcommand)]
pub enum HookType {
    /// SessionStart hook handler
    SessionStart,

    /// SessionEnd hook handler
    SessionEnd,
}
