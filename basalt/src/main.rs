use std::{env, io, path::PathBuf};

use basalt_core::obsidian::{self, Vault};
use basalt_tui::app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let obsidian_config = obsidian::config::load().unwrap();
    let mut vaults: Vec<&Vault> = obsidian_config.vaults();

    // Check for vault path argument
    if let Some(vault_path) = env::args().nth(1) {
        let path = PathBuf::from(&vault_path);
        if path.exists() && path.is_dir() {
            let vault_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Vault")
                .to_string();
            let vault = Vault {
                name: vault_name,
                path,
                open: true,
                ts: 0,
            };
            // Can't return owned Vault from here easily, so we'll use a Box
            let owned_vault = Box::new(vault);
            vaults = vec![Box::leak(owned_vault)];
        }
    }

    terminal.show_cursor()?;

    App::start(terminal, vaults)?;

    ratatui::restore();

    Ok(())
}
