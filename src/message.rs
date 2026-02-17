use iced::Color;
use derive_more::Display;
use iced::time::Instant;

#[derive(Debug, Display, Clone)]
pub enum ColorOption {
    Background,
    Text,
    Primary,
    _Success,
    _Warning,
    _Danger
}

#[derive(Debug, Clone)]
pub enum FullscreenKey {
        Escape,
        F11
}

#[derive(Debug, Clone)]
pub enum Message {
    DisplayColor(ColorOption),
    UndisplayColor(ColorOption),
    SubmitColor(Color, ColorOption),
    FileDialog,
    Tick,
    ArrowLeftPressed,
    ArrowRightPressed,
    SpacePressed,
    ResetIndex,
    ToggleFullscreen(FullscreenKey),
    TextAnimation(Instant),
    TextSizeChanged(f32),
    FileDialogFont,
    FontChanged(Result<(), iced::font::Error>),
    FileLoaded(Option<(String, String, usize)>),
}
