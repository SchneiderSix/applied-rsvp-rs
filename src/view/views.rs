use iced::widget::{button, container, column, text, rich_text, span, row, Space, slider};
use iced_aw::helpers::color_picker;
use iced_aw::Spinner;
use iced::Element;
use iced::animation::{Animation};
use iced::time;
use iced::Color;
use iced::never;
use iced::Fill;
use iced::Center;
use crate::app::State;
use crate::message::{Message, ColorOption};
use crate::style::hex_to_rgb;

pub fn main_view(current_state: &State) -> Element<'_, Message> {
    if current_state.text_loader {
        container(
            Spinner::new()
            .width(100)
            .height(100)
            .circle_radius(7.0)
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    } else {

        let word: [&str; 3]= current_state.get_word();
        container(
            column![
                row![
                    text(current_state.get_title()).size(10),
                    Space::new()
                    .width(Fill),
                ]
                .height(Fill),
                rich_text![
                    span(word[0])
                    .color(text_animation(&current_state.get_state_theme().text, &current_state.text_animation, current_state.text_instant))
                    .font(current_state.current_font),
                    span(word[1])
                    .color(text_animation(&current_state.get_state_theme().primary, &current_state.text_animation, current_state.text_instant))
                    .font(current_state.current_font),
                    span(word[2])
                    .color(text_animation(&current_state.get_state_theme().text, &current_state.text_animation, current_state.text_instant))
                    .font(current_state.current_font),
                ].on_link_click(never)
                .size(current_state.get_text_size()),
                Space::new()
                .height(Fill),
                row![
                    Space::new()
                    .width(Fill),
                    button("Reset position").on_press(Message::ResetIndex),
                    button("Upload file").on_press(Message::FileDialog),
                    button("Upload font").on_press(Message::FileDialogFont),
                    slider(10.0..=250.0, current_state.get_text_size(), Message::TextSizeChanged),
                    color_pick_bg_view(current_state.display.background, &current_state.get_state_theme().background, ColorOption::Background),
                    color_pick_bg_view(current_state.display.text, &current_state.get_state_theme().text, ColorOption::Text),
                    color_pick_bg_view(current_state.display.primary, &current_state.get_state_theme().primary, ColorOption::Primary)    
                ]
                .align_y(Center)
                .spacing(10)
                .padding(10)
            ]
            .align_x(Center)
            .spacing(10)
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

fn color_pick_bg_view<'a>(display: bool, color_option_hex: &'a str, color_option: ColorOption) -> Element<'a, Message> {
    let button_label = text(format!("Set {} color", color_option));
    let button = button(button_label).on_press(Message::DisplayColor(color_option.clone()));

    let color_picker = color_picker(
        display,
        hex_to_rgb(color_option_hex),
        button,
        Message::UndisplayColor(color_option.clone()),
        move |color| Message::SubmitColor(color, color_option.clone()),
    );

    container(
        color_picker
    )
    .into()
}

fn text_animation(text_color_hex: &str, text_animation: &Animation<f32>, text_instant: time::Instant) -> Color {
    let current_opacity:f32 = text_animation.interpolate_with(|x| x, text_instant);

    let color: Color = hex_to_rgb(text_color_hex);
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: current_opacity
    }
}
