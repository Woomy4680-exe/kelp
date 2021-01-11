use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    /// Name used to call the file: ex: I3 configuration
    pub name: Option<String>,
    /// The path to the file / directory (relative to /home/$USER)ex: .config/i3/config
    pub path: String
}
#[derive(Debug, Serialize, Deserialize)]// Required by serde_yaml
pub struct KelpConfig {
    /// The name of the configuration
    pub name: String,
    /// Files to copy contained in /home/$USER
    pub homedir: Vec<FileInfo>
}