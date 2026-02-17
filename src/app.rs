use crate::message::{Message, ColorOption, FullscreenKey};
use crate::infrastructure::{Config, ColorTheme, get_config_path, get_config_file, 
save_config_file, open_file_dialog, open_file_dialog_font, get_font, file_processor};
use iced::{
    keyboard::{self, Key, key::Named, Event,},
    animation::{Animation},
    window::{self, Mode::{Fullscreen, Windowed}},
    time, Task, Subscription,
};
use crate::style::rgb_to_hex;

pub struct DisplayColor {
    pub background: bool,
    pub text: bool,
    pub primary: bool,
    pub success: bool,
    pub warning: bool,
    pub danger: bool
}

// Custom state
pub struct State {
    config: Config,
    pub display: DisplayColor,
    text: Option<Vec<Box<str>>>,
    index: Option<usize>,
    pause: bool,
    pub text_animation: Animation<f32>,
    pub text_instant: time::Instant,
    pub fullscreen: bool,
    pub current_font: iced::Font,
    pub text_title: String,
    pub text_loader: bool,
}

impl State {
    pub fn get_state_theme(&self) -> &ColorTheme {
        &self.config.theme
    }
    pub fn get_text_size(&self) -> f32 {
        self.config.text_size
    }
    pub fn set_text_size(&mut self, size: f32) {
        self.config.text_size = size;
    }
    pub fn get_velocity(&self) -> u32 {
        self.config.velocity
    }
    pub fn set_velocity(&mut self, v: u32) {
        self.config.velocity = v;
    }
    pub fn get_word(&self) -> [&str; 3] {
        if let (Some(text), Some(index)) = (&self.text, &self.index) 
            && *index < text.len() {
                let middle_char_index: usize = (text[*index].chars().count()) / 2;
                let middle_char: (usize, char) = (text[*index].char_indices().nth(middle_char_index)).unwrap();
                [&text[*index][0..middle_char.0], &text[*index][middle_char.0..(middle_char.0 + middle_char.1.len_utf8())],
                 &text[*index][(middle_char.0 + middle_char.1.len_utf8())..]]
        } else {
            ["", "", ""]
        }
    }
    pub fn get_title(&self) -> &str {
        &self.text_title
    }
}

pub fn new() -> (State, Task<Message>) {
    let config_path = get_config_path();
    let config = get_config_file(config_path);
    let state = State {
        current_font: iced::Font::with_name(config.font.clone().leak()),
        config,
        display: DisplayColor {
            background: false,
            text: false,
            primary: false,
            success: false,
            warning: false,
            danger: false
        },
        text: None,
        index: None,
        pause: true,
        text_animation: Animation::new(1.0)
        .duration(std::time::Duration::from_millis(150))
        .easing(iced::animation::Easing::EaseInCirc),
        text_instant: time::Instant::now(),
        fullscreen: false,
        text_title: "".to_string(),
        text_loader: false,
    };
    let config_path = get_config_path();
    if let Some((_name, bytes)) = get_font(config_path, &state.config) {
        (state, iced::font::load(bytes).map(Message::FontChanged))
    } else {
        (state, Task::none())
    }
}

pub fn update(current_state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::DisplayColor(color_option) => match color_option {
            ColorOption::Background => {
                current_state.display.background = true;
                Task::none()
            },
            ColorOption::Text => {
                current_state.display.text = true;
                Task::none()
            },
            ColorOption::Primary => {
                current_state.display.primary = true;
                Task::none()
            },
            ColorOption::_Success => {
                current_state.display.success = true;
                Task::none()
            },
            ColorOption::_Warning => {
                current_state.display.warning = true;
                Task::none()
            },
            ColorOption::_Danger => {
                current_state.display.danger = true;
                Task::none()
            },
        },
        Message::UndisplayColor(color_option) => match color_option {
            ColorOption::Background => {
                current_state.display.background = false;
                Task::none()
            },
            ColorOption::Text => {
                current_state.display.text = false;
                Task::none()
            },
            ColorOption::Primary => {
                current_state.display.primary = false;
                Task::none()
            },
            ColorOption::_Success => {
                current_state.display.success = false;
                Task::none()
            },
            ColorOption::_Warning => {
                current_state.display.warning = false;
                Task::none()
            },
            ColorOption::_Danger => {
                current_state.display.danger = false;
                Task::none()
            },
        },
        Message::SubmitColor(color, color_option) => match color_option {
            ColorOption::Background => {
                current_state.config.theme.background = rgb_to_hex(color);
                current_state.display.background = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
            ColorOption::Text => {
                current_state.config.theme.text = rgb_to_hex(color);
                current_state.display.text = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
            ColorOption::Primary => {
                current_state.config.theme.primary = rgb_to_hex(color);
                current_state.display.primary = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
            ColorOption::_Success => {
                current_state.config.theme.success = rgb_to_hex(color);
                current_state.display.success = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
            ColorOption::_Warning => {
                current_state.config.theme.warning = rgb_to_hex(color);
                current_state.display.warning = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
            ColorOption::_Danger => {
                current_state.config.theme.danger = rgb_to_hex(color);
                current_state.display.danger = false;
                let config_path = get_config_path();
                save_config_file(config_path, &current_state.config);
                Task::none()
            },
        },
        Message::FileDialog => {
            if let Some((data, filename)) = open_file_dialog() {
            
                let config_path = get_config_path();

                // Running off the main thread
                current_state.text_loader = true;
                return Task::perform(file_processor(current_state.config.text_history.clone(), config_path, data, filename), Message::FileLoaded)
            }
            Task::none()
        },
        Message::FileLoaded(text) => {
            current_state.text_loader = false;
            if let Some((content, title, index)) = text {
                // 1. Split text string without counting white spaces
                // 2. Convert each &str slice to String, then to Box<str>
                // 3. Collects into a Vec
                current_state.text = Some(content
                    .split_whitespace()
                    .map(|s: &str| s.to_string().into_boxed_str())
                    .collect());
                
                current_state.index = Some(index);
                current_state.pause = false;
                current_state.text_title = title;
            }
            Task::none()
        },
        Message::Tick => {
            if let (Some(text), Some(index)) = (&current_state.text, &current_state.index) 
                && *index < text.len() && !current_state.pause {
                let current_instant = time::Instant::now();
                current_state.text_animation.go_mut(0.0, current_instant);
                current_state.text_instant = current_instant;
            }
            Task::none()
        },
        Message::TextAnimation(instant) => {
            current_state.text_instant = instant;
            
            if let (Some(text), Some(index)) = (&current_state.text, &current_state.index) 
            && *index < text.len() && !current_state.pause 
            && !current_state.text_animation.is_animating(instant)

            && current_state.text_animation.value() == 0.0 {
                current_state.index = Some(*index + 1);
                current_state.text_animation.go_mut(1.0, instant);
            }
            Task::none()
        },
        Message::ArrowLeftPressed => {
            current_state.set_velocity((current_state.get_velocity() + 100).clamp(200, 2000));
            let config_path = get_config_path();
            save_config_file(config_path, &current_state.config);
            Task::none()
        },
        Message::ArrowRightPressed => {
            current_state.set_velocity((current_state.get_velocity() - 100).clamp(200, 2000));
            let config_path = get_config_path();
            save_config_file(config_path, &current_state.config);
            Task::none()
        },
        Message::SpacePressed => {
                current_state.pause = !current_state.pause;
                if let Some(index) = current_state.index {
                    let config_path = get_config_path();
                    current_state.config.text_history.insert(current_state.text_title.clone(), index);
                    save_config_file(config_path, &current_state.config);
                }
                Task::none()
            },
        Message::ResetIndex => {
          current_state.index = Some(0);
          Task::none()
        },
        Message::ToggleFullscreen(key) => match key {
             FullscreenKey::Escape => {
                 if current_state.fullscreen {
                     current_state.fullscreen = false;
                     window::latest().and_then(|id| window::set_mode::<Message>(id, Windowed))
                 } else {
                     Task::none()
                 }
             },
             FullscreenKey::F11 => {
                  if current_state.fullscreen {
                     current_state.fullscreen = false;
                     window::latest().and_then(|id| window::set_mode::<Message>(id, Windowed))
                 } else if !current_state.fullscreen {
                     current_state.fullscreen = true;
                     window::latest().and_then(|id| window::set_mode::<Message>(id, Fullscreen))
                 } else {
                     Task::none()
                 }
             },
        },
        Message::TextSizeChanged(size) => {
            current_state.set_text_size(size);
            let config_path = get_config_path();
            save_config_file(config_path, &current_state.config);
            Task::none()
        },
        Message::FileDialogFont => {
            let config_path = get_config_path();
            if let Some((name, content)) = open_file_dialog_font(config_path) {
                current_state.config.font = name;
                
                iced::font::load(content)
                .map(Message::FontChanged)
                
            } else {
                Task::none()
            }
        },
        Message::FontChanged(result) => {
            match result {
                Ok(()) => {
                    let config_path = get_config_path();
                    save_config_file(config_path, &current_state.config);
                    current_state.current_font = iced::Font::with_name(current_state.config.font.clone().leak())
                },
                Err(e) => panic!("Error updating font: {:?}", e),
            }
            Task::none()
        }
    }
}

pub fn subscription(current_state: &State) -> Subscription<Message> {
    let timer_subscription = time::every(std::time::Duration::from_millis(current_state.get_velocity().into()))
    .map(|_| Message::Tick);

    let keyboard_listener = keyboard::listen()
        .filter_map(|e: Event| {
            match e {
                Event::KeyPressed {key, ..} => match key {
                    Key::Named(Named::ArrowLeft) => Some(Message::ArrowLeftPressed),
                    Key::Named(Named::ArrowRight) => Some(Message::ArrowRightPressed),
                    Key::Named(Named::Space) => Some(Message::SpacePressed),
                    Key::Named(Named::F11) => Some(Message::ToggleFullscreen(FullscreenKey::F11)),
                    Key::Named(Named::Escape) => Some(Message::ToggleFullscreen(FullscreenKey::Escape)),
                    _ => None
                },
                _ => None
            }
        });

    let frame_lsitener = window::frames()
    .map(Message::TextAnimation);
        
    Subscription::batch([
        timer_subscription,
        keyboard_listener,
        frame_lsitener
    ])
}
