use std::path::PathBuf;

use anyhow::Result;
use winreg::RegKey;
use winreg::enums::*;

pub struct ContextMenuItem {
    pub label: String,
    pub icon_path: Option<PathBuf>,
    pub executable_path: PathBuf,
    pub args: String,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<String>, executable_path: impl Into<PathBuf>) -> Self {
        Self {
            label: label.into(),
            icon_path: None,
            executable_path: executable_path.into(),
            args: "\"%1\"".into(),
        }
    }

    pub fn with_args(mut self, args: impl Into<String>) -> Self {
        self.args = args.into();
        self
    }

    pub fn with_icon_path(mut self, icon_path: impl Into<PathBuf>) -> Self {
        self.icon_path = Some(icon_path.into());
        self
    }
}
pub fn add_context_menu(app_name: &str, item: &ContextMenuItem, extensions: &[&str]) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for ext in extensions {
        let Some(progid) = get_progid(ext) else {
            continue;
        };

        let base = format!(r"Software\Classes\{progid}\shell\{app_name}");
        let (menu, _) = hkcu.create_subkey(&base)?;
        menu.set_value("", &item.label)?;

        if let Some(icon) = &item.icon_path {
            menu.set_value("Icon", &icon.as_os_str())?;
        }

        let (cmd, _) = hkcu.create_subkey(format!(r"{base}\command"))?;
        let command = format!(r#""{}" {}"#, item.executable_path.to_string_lossy(), item.args);
        cmd.set_value("", &command)?;
    }
    Ok(())
}
pub fn remove_context_menu(app_name: &str, extensions: &[&str]) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for ext in extensions {
        let Some(progid) = get_progid(ext) else {
            continue;
        };

        let path = format!(r"Software\Classes\{progid}\shell");
        if let Ok(key) = hkcu.open_subkey_with_flags(&path, KEY_WRITE) {
            let _ = key.delete_subkey_all(app_name);
        }
    }
    Ok(())
}
fn get_progid(ext: &str) -> Option<String> {
    RegKey::predef(HKEY_CLASSES_ROOT)
        .open_subkey(ext)
        .and_then(|key| key.get_value(""))
        .ok()
}
