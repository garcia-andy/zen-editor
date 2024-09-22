use iced::{alignment::Vertical, highlighter, widget::{horizontal_space, pick_list, row}, Element, Length, Task};
use registers::{Event, Register};
use ui::{button_with_icon, icon_code, Icon};

pub struct TopMenu {
    pub theme: highlighter::Theme,
}

impl TopMenu {
    pub fn new() -> Self {
        Self {
            theme: highlighter::Theme::Base16Ocean,
        }
    }
}

impl Register for TopMenu {
    fn update(&mut self, _event: Event) -> Task<Event> {
        match _event {
            Event::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            _ => Task::none(),
        }
    }
    fn view(&self) -> Element<'_, Event> {
        row![
            button_with_icon(icon_code(Icon::File.into()), "Open", Event::OpenFile),
            button_with_icon(icon_code(Icon::Save.into()), "Save", Event::Save),
            button_with_icon(icon_code(Icon::Refresh.into()), "Reload", Event::ScanAllFiles),
            horizontal_space(),
            pick_list(highlighter::Theme::ALL, Some(self.theme), Event::ThemeChanged),
        ].spacing(5).align_y(Vertical::Center).padding(2).width(Length::Fill)
        .into()
    }
}
