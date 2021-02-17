use crate::lib::{
    config::loader::load_cfg,
    fsutil::{
        copy::copy,
        paths::{get_root, get_to_make},
    },
    structs::config::KelpDotConfig,
    util::{exec::get_root_exec_program, os::get_host_os},
};
use kelpdot_macros::*;
use std::path::Path;
use std::process::Command;
/// Backup dotfiles
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan!("[INFO] Saving dotfiles {}...", root);
    debug_print!("Building OS list...");
    let os = get_host_os()?; // Here we get guest os; If OS is unreconized, return a generic GNU / Linux System
    cyan!("[INFO] Found Os {}", os.prettyname);
    let config: KelpDotConfig = load_cfg(root.clone())?; // Load a KelpConfig struct wich is basically $DOTFILES_ROOT/kelp.yaml

    if let Some(files) = config.homefiles {
        // If config has "homefiles" keys, copy each $HOME/$FILE
        let home = std::env::var("HOME")?; // Get $HOME path or crash
        debug_print!("Home: {}", home);

        // Make sur that $DOTFILES_ROOT/home doesn't exist
        // or doesn't contain files
        if Path::new(&format!("{}/home", root)).exists() {
            std::fs::remove_dir_all(&format!("{}/home", root))?;
        }
        std::fs::create_dir(format!("{}/home", root))?;
        for f in files {
            green!("[SAVE] Copying file {}...", f);
            let path = format!("{}/{}", home, f.path);
            let file = Path::new(&path);
            // Make sur that file exists
            if !file.exists() {
                magenta!("[WARNING] Skipping {} because it doesn't exist!", f);
                break;
            }
            // Get path to make
            // Example:
            // home/**.config/i3** directory
            let tomake = get_to_make(f.path)?;
            // Create the file
            std::fs::create_dir_all(format!("{}/home/{}", root, tomake))?;
            copy(
                path.clone(),
                format!(
                    "{}/home/{}/{}",
                    root,
                    tomake,
                    file.file_name().unwrap().to_str().unwrap().to_owned()
                ),
            )?;
        }
        cyan!("[OK] Homefiles saved!");
    }
    // If config has "rootfiles" key, backup every file
    if let Some(files) = config.rootfiles {
        for f in files {
            green!("[SAVE] Copying file {}", f);
            // Get path to make:
            // Example:
            // $DOTFILES_ROOT/etc/portage/repos.conf
            let path = f.path.to_owned();
            let tomake = get_to_make(f.path)?;
            let file = Path::new(&path);
            let file_name = file.file_name().unwrap().to_str().unwrap().to_owned();
            let dest = format!("{}/{}/{}", root, tomake, &file_name);
            if Path::new(&dest).exists() {
                if Path::new(&dest).is_file() {
                    std::fs::remove_file(dest)?;
                } else {
                    std::fs::remove_dir_all(dest)?;
                }
            }
            std::fs::create_dir_all(format!("{}/{}", root, tomake))?;
            copy(
                path.clone(),
                format!(
                    "{}/{}/{}",
                    root,
                    tomake,
                    file_name
                ),
            )?;
        }
        cyan!("[OK] Rootfiles saved!");
    }
    if let Some(scripts) = config.postsave {
        for script in scripts {
            if let Some(run) = script.elevated {
                if run {
                    debug_print!("Getting elevator for script {}", script);
                    let elevator = get_root_exec_program()?;
                    cyan!(
                        "[POSTSAVE] Running script {}/{} with {}",
                        root,
                        script.path,
                        elevator
                    );
                    Command::new(&elevator) // Use SH because some systems symlinks it to bash / zsh / ash
                        .arg("sh")
                        .arg(&format!("{}/{}", root, script.path))
                        .status()?;
                }
            } else {
                cyan!("[POSTSAVE] Running script {}/{}", root, script.path);
                Command::new("sh") // Use SH because some systems symlinks it to bash / zsh / ash
                    .arg(&format!("{}/{}", root, script.path))
                    .status()?;
            }
        }
    }
    magenta!("[OK] All dotfiles saved!");
    Ok(())
}