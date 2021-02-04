use crate::lib::{
    fsutil::paths::get_root, terminal::colors::*, terminal::debug::debug_print, util::env::is_debug,
};
/// Backup dotfiles
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    debug_print("Building OS list...");
    let oses = crate::lib::util::os::build_os_list()?; // Get a Vec<Os> wich is a list of reconisables oses
    if is_debug() {
        debug_print("Oses:");
        for os in oses {
            debug_print(&format!("Name: {} | File: {}", os.name, os.file));
        }
    }
    Ok(())
}
