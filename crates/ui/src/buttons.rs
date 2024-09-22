use iced::{alignment, widget::{button, container, text, tooltip}, Element, Length};
use registers::Event;

pub fn base_button<'a>(
    content: impl Into<Element<'a, Event>>,
    msg: Event,
) -> button::Button<'a, Event> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

pub fn danger_button<'a>(
    content: impl Into<Element<'a, Event>>,
    msg: Event,
) -> button::Button<'a, Event> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::danger)
        .on_press(msg)
}

pub fn danger_button_with_icon(
    icon: impl Into<Element<'static, Event>>,
    label: &str,
    msg: Event,
) -> tooltip::Tooltip<Event> {
    tooltip(danger_button(
       icon.into(),
        msg,
    ), label, tooltip::Position::FollowCursor)
    .style( container::bordered_box )
}

pub fn button_with_icon(
    icon: impl Into<Element<'static, Event>>,
    label: &str,
    msg: Event,
) -> tooltip::Tooltip<Event> {
    tooltip(base_button(
       icon.into(),
        msg,
    ), label, tooltip::Position::FollowCursor)
    .style( container::bordered_box )
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