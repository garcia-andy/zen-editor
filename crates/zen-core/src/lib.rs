use iced::{widget::{container, Column}, Element, Length, Subscription, Task, Theme};
pub use registers::{Register, Event};

// registers
pub use editor::{Editor, KeyBinding};

pub struct ZenCore {
    pub title: String,
    pub theme: Theme,
    registers: Vec<Box<dyn registers::Register>>,
}

impl Default for ZenCore {
    fn default() -> Self {
        let editor: Box<dyn Register> = Box::new(Editor::new());
        
        Self::new_with("Zen".to_string(), Theme::Dark, vec![
            editor
        ])
    }
}

impl ZenCore {
    pub fn new(title: String, theme: Theme) -> Self {
        Self {
            title,
            theme,
            registers: Vec::new(),
        }
    }
    
    pub fn new_with(
        title: String, 
        theme: Theme, 
        registers: Vec<Box<dyn registers::Register>>
    ) -> Self {
        Self {
            title,
            theme,
            registers,
        }
    }
        
    pub fn register(&mut self, register: Box<dyn registers::Register>) -> &mut Self {
        self.registers.push(register);
        self
    }
    
    pub fn register_with<R: registers::Register + 'static>(&mut self, register: R) -> &mut Self {
        self.register(Box::new(register))
    }
    
    
}

impl Register for ZenCore {
    fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::None => Task::none(),
            Event::ThemeChanged(th) => {
                if th.is_dark(){
                    self.theme = Theme::Dark;
                }else{
                    self.theme = Theme::Light;
                }
                self.registers.iter_mut().fold(
                    Task::none(),
                    |t, r| 
                    t.chain(r.update(event.clone()))
                )
            },
            _ => {
                self.registers.iter_mut().fold(
                    Task::none(),
                    |t, r| 
                    t.chain(r.update(event.clone()))
                )
            }
        }
    }
    
    fn subscription(&self) -> Subscription<Event> {
        Subscription::batch(
            self.registers.iter()
                .map(|r| r.subscription())
        )
    }
    
    fn view(&self) -> Element<'_, Event> {
       let mut view = Column::new()
           .spacing(5)
           .padding(5)
           .width(Length::Fill)
           .height(Length::Fill);
       
       view = self.registers.iter().fold(
           view, 
           |v,r| 
               v.push(r.view())
       );
       
       container(view)
           .center(Length::Fill)
           .width(Length::Fill)
           .height(Length::Fill)
           .into()
   }
    
    fn title(&self) -> String {
        self.title.clone()
    }
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
