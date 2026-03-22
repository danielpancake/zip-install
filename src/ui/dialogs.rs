use native_dialog::{DialogBuilder, MessageLevel};

pub fn show_message(message: &str, level: MessageLevel) {
    DialogBuilder::message()
        .set_level(level)
        .set_text(message)
        .alert()
        .show()
        .unwrap_or_else(|e| eprintln!("Failed to show dialog: {}", e));
}

pub fn show_info_message(message: &str) {
    show_message(message, MessageLevel::Info);
}

pub fn show_warning_message(message: &str) {
    show_message(message, MessageLevel::Warning);
}

pub fn show_error_message(message: &str) {
    show_message(message, MessageLevel::Error);
}

pub fn show_confirm_dialog(message: &str) -> bool {
    DialogBuilder::message()
        .set_level(MessageLevel::Warning)
        .set_text(message)
        .confirm()
        .show()
        .unwrap_or(false)
}
