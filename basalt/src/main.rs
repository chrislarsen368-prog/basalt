use std::{env, io, path::PathBuf};

use basalt_core::obsidian::{self, Vault};
use basalt_tui::app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    // Check for vault path argument first
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
            // Leak the vault so it lives for the lifetime needed
            let vaults: Vec<&Vault> = vec![Box::leak(Box::new(vault))];
            terminal.show_cursor()?;
            App::start(terminal, vaults)?;
            ratatui::restore();
            return Ok(());
        }
    }

    // No valid vault path, try obsidian config
    let vaults: Vec<&Vault> = match obsidian::config::load() {
        Ok(config) => {
            // Convert owned vaults to leaked references
            config
                .vaults()
                .into_iter()
                .map(|v| {
                    let leaked: &Vault = Box::leak(Box::new((*v).clone()));
                    leaked
                })
                .collect()
        }
        Err(_) => Vec::new(),
    };

    terminal.show_cursor()?;

    App::start(terminal, vaults)?;

    ratatui::restore();

    Ok(())
}
