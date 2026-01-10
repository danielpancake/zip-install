use native_dialog::{DialogBuilder, MessageLevel};

pub fn show_error_message(message: &str) {
    DialogBuilder::message()
        .set_level(MessageLevel::Warning)
        .set_text(message)
        .alert()
        .show()
        .unwrap();
}
