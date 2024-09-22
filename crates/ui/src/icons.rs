use iced::{widget::text, Element, Font};
use registers::Event;

pub const ICON: Font = Font::with_name("fontello");

pub enum Icon {
    File,
    Save,
    Refresh,
    HorizontalSplit,
    VerticalSplit,
    Close,
    Pin,
    Unpin,
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
            Icon::File => '\u{e800}',
            Icon::Save => '\u{e801}',
            Icon::Refresh => '\u{e802}',
            Icon::HorizontalSplit => '\u{e803}',
            Icon::VerticalSplit => '\u{e804}',
            Icon::Close => '\u{e805}',
            Icon::Pin => '\u{e806}',
            Icon::Unpin => '\u{e807}',
        }
    }
}