mod infrastructure;
mod app;
mod message;
mod view;
mod style;

use crate::app::{new, update, State, subscription};
use view::main_view;
use style::custom_theme_from_state;

fn main() -> iced::Result {
    iced::application(new, update, main_view)
    .theme(|state: &State| custom_theme_from_state(state.get_state_theme()))
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .subscription(subscription)
    .window(iced::window::Settings {
        min_size: Some(iced::Size::new(1000.0, 500.0)),
        position: iced::window::Position::Centered,
        size: iced::Size::new(1000.0, 500.0),
        ..Default::default()
    })
    .run()
}
