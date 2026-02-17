pub mod config;
pub mod paths;

pub use config::get_config_path;
pub use config::Config;
pub use config::ColorTheme;
pub use config::get_config_file;
pub use config::save_config_file;
pub use config::open_file_dialog;
pub use config::open_file_dialog_font;
pub use config::get_font;
pub use config::file_processor;

