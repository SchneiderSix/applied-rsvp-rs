use iced::Theme;
use iced::theme::Palette;
use iced::Color;
use crate::infrastructure::ColorTheme;

pub fn custom_theme_from_state(current_theme: &ColorTheme) -> Theme {
    let custom_palette: Palette = Palette {
        background: hex_to_rgb(&current_theme.background),
        text: hex_to_rgb(&current_theme.text),
        primary: hex_to_rgb(&current_theme.primary),
        success: hex_to_rgb(&current_theme.success),
        warning: hex_to_rgb(&current_theme.warning),
        danger: hex_to_rgb(&current_theme.danger)
    };
    Theme::custom("custom", custom_palette)
}

pub fn hex_to_rgb(s: &str) -> Color {
    if !s.starts_with("#") || s.len() != 7 { panic!("Invalid hex color format") };

    Color::from_rgb8(u8::from_str_radix(&s[1..=2], 16).expect("Invalid format for custom palette"), 
    u8::from_str_radix(&s[3..=4], 16).expect("Invalid format for custom palette"), 
    u8::from_str_radix(&s[5..=6], 16).expect("Invalid format for custom palette"))
}

pub fn rgb_to_hex(c: Color) -> String {
    format!("#{:02X}{:02X}{:02X}",(c.r * 255.0) as u8, (c.g * 255.0) as u8, (c.b * 255.0) as u8)
}
