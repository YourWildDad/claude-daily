use anyhow::Result;
use std::path::PathBuf;

use crate::config::{load_config, save_config, get_config_path, Config};
use crate::archive::ArchiveManager;

/// Initialize the daily archive system
pub async fn run(storage_path: Option<PathBuf>) -> Result<()> {
    println!("[daily] Initializing Daily Context Archive System...");

    // Load or create config
    let mut config = load_config().unwrap_or_else(|_| Config::default());

    // Update storage path if provided
    if let Some(path) = storage_path {
        config.storage.path = path;
    }

    // Save config
    save_config(&config)?;
    let config_path = get_config_path()?;
    println!("[daily] Configuration saved to: {}", config_path.display());

    // Create storage directory
    let manager = ArchiveManager::new(config.clone());
    let storage_dir = manager.ensure_storage_dir()?;
    println!("[daily] Storage directory: {}", storage_dir.display());

    // Create today's directory
    let today_dir = manager.ensure_today_dir()?;
    println!("[daily] Today's archive: {}", today_dir.display());

    println!();
    println!("[daily] Initialization complete!");
    println!();
    println!("Next steps:");
    println!("  1. Install the plugin: daily install");
    println!("  2. Configure settings: daily config --show");
    println!("  3. View archives: daily view");

    Ok(())
}
