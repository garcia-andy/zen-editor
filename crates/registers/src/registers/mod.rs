
use iced::{widget::Column, Element, Subscription, Task, Theme};
use crate::Event;

pub trait Register {
    fn update(&mut self, _: Event) -> Task<Event> {
        Task::none()
    }
    
    fn view(&self) -> Element<'_, Event> {
        Column::new().into()
    }
    
    fn subscription(&self) -> Subscription<Event> {
        Subscription::none()
    }
    
    fn title(&self) -> String {
        String::from("Register")
    }
    
    fn theme(&self) -> Theme {
        Theme::Dracula
    }

}
