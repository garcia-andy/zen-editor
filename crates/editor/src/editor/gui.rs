
use iced::{alignment, widget::{button, row, text}, Element, Font, Length};
use registers::Event;

/* ICONS */

pub const ICON: Font = Font::with_name("FontAwesome");

pub enum Icon {
    File,
    Save,
    Refresh,
}

pub fn icon_code(codepoint: char) -> Element<'static, Event> {
    text(codepoint).font(ICON).into()
}

impl From<Icon> for Element<'static, Event> {
    fn from(icon: Icon) -> Self {
        icon_code(icon.into())
    }
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::File => '\u{f016}',
            Icon::Save => '\u{f019}',
            Icon::Refresh => '\u{f021}',
        }
    }
}



/* BUTTONS */

pub fn base_button<'a>(
    content: impl Into<Element<'a, Event>>,
    msg: Event,
) -> button::Button<'a, Event> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

pub fn button_with_icon(
    icon: impl Into<Element<'static, Event>>,
    label: &str,
    msg: Event,
) -> button::Button<Event, iced::Theme, iced::Renderer> {
    base_button(
        row![
            icon.into(),
            text(label).align_y(alignment::Vertical::Center),
        ]
        .spacing(5).padding(4)
        .align_y(iced::Alignment::Center),
        msg,
    )
}

pub fn labeled_button(
    label: &str,
    msg: Event,
) -> button::Button<Event, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg)
}

pub fn debug_button_s(label: &str) -> button::Button<Event, iced::Theme, iced::Renderer> {
    labeled_button(label, Event::None).width(Length::Shrink)
}

