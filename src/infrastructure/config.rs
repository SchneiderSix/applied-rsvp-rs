use super::paths::PathConfig;
use serde::{Serialize, Deserialize};
use rfd::AsyncFileDialog;
use pollster::FutureExt as _;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub font: String,
    pub text_size: f32,
    pub velocity: u32,
    pub theme: ColorTheme,
    pub text_history: std::collections::HashMap<String, usize>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTheme {
    pub background: String,
    pub text: String,
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub danger: String
}

pub fn get_config_path() -> PathConfig  {
    let mut result: PathConfig = Default::default();
    match std::env::consts::OS {
        "linux" => {
            let root = std::env::var("HOME").expect("Environment variable missing");
            result.set_config(format!("{}/.config/rsvp/", root));
            result.set_data(format!("{}/.local/share/rsvp/", root));
            result.set_cache(format!("{}/.cache/rsvp/", root));
            result
        },
        "macos" => {
            let root = std::env::var("HOME").expect("Environment variable missing");
            result.set_config(format!("{}/Library/Application Support/rsvp/", root));
            result.set_data(format!("{}/Library/Application Support/rsvp/", root));
            result.set_cache(format!("{}/Library/Caches/rsvp/", root));
            result
        },
        "windows" => {
            let root_app = std::env::var("APPDATA").expect("Environment variable missing");
            let root_local = std::env::var("LOCALAPPDATA").expect("Environment variable missing");
            result.set_config(format!("{}\\rsvp\\", root_app));
            result.set_data(format!("{}\\rsvp\\", root_local));
            result.set_cache(format!("{}\\rsvp\\cache\\", root_local));
            result
        },
        _ => panic!("OS isn't linux, macos, neither windows") // Error, exit
    }
}

pub fn get_config_file(path: PathConfig) -> Config {
        // Get string from path: config
        match std::fs::read_to_string(format!("{}config.toml", path.get_config())) {
            Ok(content) => {
                // Deserialize into struct
                toml::from_str(&content)
                .expect("Invalid TOML format")
            },
            Err(_) => {
                // Create object
                let config = Config {
                    font: String::from("default"),
                    velocity: 2000,
                    text_size: 20.0,
                    theme: ColorTheme {
                        background: String::from("#FFFEF9"),
                        text: String::from("#15161B"),
                        primary: String::from("#C17F5A"),
                        success: String::from("#809C6C"),
                        warning: String::from("#AE363F"),
                        danger: String::from("#DF3535"),
                    },
                    text_history: std::collections::HashMap::new(),
                };
                // Serialize into toml string 
                let toml_string = toml::to_string(&config)
                .expect("Invalid TOML format");
                
                // Create folders
                std::fs::create_dir_all(path.get_config())
                .expect("Could not create config folder");
                std::fs::create_dir_all(path.get_data())
                .expect("Could not create data folder");
                std::fs::create_dir_all(path.get_cache())
                .expect("Could not create cache folder");
                std::fs::create_dir_all(format!("{}fonts/", path.get_data()))
                .expect("Could not create data fonts folder");
                std::fs::create_dir_all(format!("{}texts/", path.get_data()))
                .expect("Could not create data texts folder");
                
                // Write the string to a file 
                std::fs::write(format!("{}config.toml", path.get_config()), toml_string)
                .expect("Could not create config file");
                config
            }
        }
}

pub fn save_config_file(path: PathConfig, config_data: &Config) {
    // Serialize into toml string
    let toml_string = toml::to_string(config_data)
    .expect("Invalid TOML format");

    // Write the string to a file 
    std::fs::write(format!("{}config.toml", path.get_config()), toml_string)
    .expect("Could not create config file");
}

pub fn open_file_dialog() -> Option<(Vec<u8>, String)> {
    let future = async {
        let file = AsyncFileDialog::new()
        .set_title("Choose any text file to being read")
        .add_filter("text", &["txt", "csv", "md", "html", "pdf"])
        .set_directory("/")
        .pick_file()
        .await;

        if let Some(file) = file {
            let data = file.read().await;

            let filename = file.file_name();

            Some((data, filename))
        } else {
            None
        }

    };
    future.block_on()
}

pub async fn file_processor(text_history: std::collections::HashMap<String, usize>, path: PathConfig, data: Vec<u8>, filename: String) -> Option<(String, String, usize)> {
    let filename_collection = filename.split(".").collect::<Vec<&str>>();
    let filename_ext = filename_collection.last().copied();
    let clean_filename = std::path::Path::new(&filename).file_stem()
    .and_then(|s| s.to_str())
    .unwrap().to_string();

    // If txt already exists then return txt content, else save it
    match std::fs::read_to_string(format!("{}texts/{}.txt", path.get_data(), clean_filename)) {
        // Send saved index
        Ok(content) => {
            Some((content, clean_filename.clone(), text_history.get(&clean_filename).copied().unwrap_or(0)))
        },
        Err(_) => {
            match filename_ext {
                Some("txt") | Some("csv") | Some("md") => {
                    let s = String::from_utf8_lossy(&data).to_string();
                    let _ = std::fs::write(format!("{}texts/{}.txt", path.get_data(), clean_filename), &s);
                    Some((s, clean_filename, 0))
                },
                Some("pdf") => {
                    let cursor = std::io::Cursor::new(data);
                    let reader = oxidize_pdf::parser::PdfReader::new(cursor).ok()?;
                    let doc = oxidize_pdf::parser::PdfDocument::new(reader);
                    let pages = doc.extract_text().ok()?;

                    let text_page = pages.into_iter().map(|x| x.text);
                    let s = text_page.collect::<Vec<String>>().join("");
                    
                    if s.is_empty() {
                        panic!("Error: result string is empty");
                    } else {
                        let _ = std::fs::write(format!("{}texts/{}.txt", path.get_data(), clean_filename), &s);
                        Some((s, clean_filename, 0))
                    }
                },
                Some("html") => {
                    let s = html2text::from_read(&data[..], 80).ok();
                    let _ = std::fs::write(format!("{}texts/{}.txt", path.get_data(), clean_filename), s.clone().unwrap());
                    Some((s?, clean_filename, 0))
                },
                _ => {panic!("Unsupported file type")}
            }
        }
    }
}

pub fn open_file_dialog_font(path: PathConfig) -> Option<(String, Vec<u8>)> {
    let future = async {
        let file = AsyncFileDialog::new()
        .set_title("Choose any font file to be used")
        .add_filter("font", &["ttf", "otf", "woff", "woff2"])
        .set_directory("/")
        .pick_file()
        .await;

        if file.as_ref().is_some() {
            let data = file.unwrap().read().await;

            let parsed_data = ttf_parser::Face::parse(&data, 0).unwrap();

            let name = parsed_data.names()
            .into_iter()
            .find_map(|label| {
                if label.name_id == ttf_parser::name_id::FAMILY {
                    label.to_string()
                } else {
                    None
                }
            });

            if let Some(family_name) = name {
                // Save file if doesn't already exists
                let new_file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}fonts/{}", path.get_data(), family_name));

                match new_file {
                    Ok(mut f) => {
                        f.write_all(&data).expect("Error writing file");
                        Some((family_name, data))
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                        Some((family_name, data))
                    },
                    Err(e) => {
                        panic!("Error creating file: {}", e)
                    }
                }
            } else {
                panic!("Error getting font family name")
            }
        } else {
            None
        }

    };
    future.block_on()
}

pub fn get_font(path: PathConfig, config: &Config) -> Option<(String, Vec<u8>)> {
    let font_name = config.font.clone();

    if font_name == "default" {
        return None;
    }

    Some((font_name.clone(), std::fs::read(format!("{}/fonts/{}", path.get_data(), font_name)).unwrap()))
}
