use anyhow::Result;
use winreg::enums::*;
use winreg::RegKey;

pub struct ContextMenuConfig {
    pub menu_text: String,
    pub exe_path: String,
    pub command_args: String,
    pub icon_path: Option<String>,
}

pub fn get_progid(extension: &str) -> Option<String> {
    let ext = if extension.starts_with('.') {
        extension.to_string()
    } else {
        format!(".{}", extension)
    };

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    hkcr.open_subkey(&ext).ok()?.get_value("").ok()
}

impl ContextMenuConfig {
    pub fn new(menu_text: impl Into<String>, exe_path: impl Into<String>) -> Self {
        Self {
            menu_text: menu_text.into(),
            exe_path: exe_path.into(),
            command_args: "\"%1\"".to_string(),
            icon_path: None,
        }
    }

    pub fn with_args(mut self, args: impl Into<String>) -> Self {
        self.command_args = args.into();
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon_path = Some(icon.into());
        self
    }
}

pub fn add_context_menu(
    app_name: &str,
    config: &ContextMenuConfig,
    extensions: &[&str],
) -> Result<()> {
    for ext in extensions {
        if let Some(progid) = get_progid(ext) {
            add_for_extension(app_name, &progid, config)?;
        }
    }
    Ok(())
}

fn add_for_extension(app_name: &str, extension: &str, config: &ContextMenuConfig) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ext_key = format!("Software\\Classes\\{}\\shell\\{}", extension, app_name);

    let (menu_key, _) = hkcu.create_subkey(&ext_key)?;
    menu_key.set_value("", &config.menu_text)?;

    if let Some(icon) = &config.icon_path {
        menu_key.set_value("Icon", icon)?;
    }

    let (command_key, _) = hkcu.create_subkey(format!("{}\\command", ext_key))?;
    command_key.set_value(
        "",
        &format!("\"{}\" {}", config.exe_path, config.command_args),
    )?;

    Ok(())
}

pub fn remove_context_menu(app_name: &str, extensions: &[&str]) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for ext in extensions {
        let shell_path = format!("Software\\Classes\\{}\\shell", ext);
        if let Ok(shell_key) = hkcu.open_subkey_with_flags(&shell_path, KEY_WRITE) {
            let _ = shell_key.delete_subkey_all(app_name);
        }
    }
    Ok(())
}
