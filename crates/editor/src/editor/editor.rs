use std::{collections::HashMap, sync::{Mutex, MutexGuard, OnceLock}};

use iced::{
    alignment::Horizontal, color, highlighter, keyboard, widget::{column, container, text, text_editor}, Background, Border, Color, Element, Font, Length, Subscription, Task, Theme
};
use iced_aw::{ TabBar, TabLabel};
use registers::{ Event, Register};

use crate::fileinfo::FileInfo;
use crate::services::*;

#[derive(Debug,Clone)]
pub struct KeyBinding {
    pub key: char,
    pub ctrl: bool,
    pub shift: bool,
    pub event: Event,
}

impl KeyBinding {
    pub fn new(key: char, ctrl: bool, shift: bool, event: Event) -> Self {
        Self {
            key,
            ctrl,
            shift,
            event,
        }
    }
}

fn get_global_hashmap() -> MutexGuard<'static, HashMap<char, Vec<KeyBinding>>> {
    static MAP_KEYS: OnceLock<Mutex<HashMap<char, Vec<KeyBinding>>>> = OnceLock::new();
    MAP_KEYS.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned")
}

pub struct Editor {
    content: text_editor::Content,
    pub files: Vec<FileInfo>,
    pub active_file: usize,
    pub theme: highlighter::Theme,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            files: Vec::new(),
            active_file: 0,
            theme: highlighter::Theme::Base16Ocean,
        }
    }

    pub fn new_from(t: &Self) -> Self {
        Self {
            content: text_editor::Content::with_text(t.get_content().as_str()),
            files: t.files.clone(),
            active_file: t.active_file,
            theme: t.theme.clone(),
        }
    }

    pub fn get_content(&self) -> String {
        self.content.text()
    }

    pub fn get_key_bindings_map(&self) -> HashMap<char, Vec<KeyBinding>> {
        get_global_hashmap().clone()
    }
    
    pub fn add_key_binding(&mut self, key: char, ctrl: bool, shift: bool, event: Event) {
        let mut map = get_global_hashmap();
        map.entry(key)
            .or_insert(Vec::new())
            .push(KeyBinding::new(key, ctrl, shift, event.clone()));
    }
    
    pub fn add_ctrl_key_binding(&mut self, key: char, shift: bool, event: Event) {
        self.add_key_binding(key, true, shift, event.clone());
    }

    pub fn add_shift_key_binding(&mut self, key: char, event: Event) {
        self.add_key_binding(key, true, true, event.clone());
    }
    
    pub fn add_keys_bindings(&mut self, bindings: Vec<KeyBinding>) {
        for binding in bindings {
            self.add_key_binding(binding.key, binding.ctrl, binding.shift, binding.event);
        }
    }
    
}

impl Register for Editor {
    fn update(&mut self, _event: Event) -> Task<Event> {
        match _event {
            Event::EditorAction(action) => {
                self.content.perform(action);
                self.files[self.active_file].content = self.content.text();
                Task::none()
            }
            Event::Save => {
                Task::perform(save_content(Editor::new_from(self)), |_| {
                    Event::Saved
                })
            }
            Event::Quit(idx) => {
                let i = if let Some(index) = idx {
                    index
                } else {
                    self.active_file
                };
                Task::perform(
                    quit_file(Editor::new_from(self), i), 
                    |path| {
                    if let Some(path) = path {
                        Event::Quited(path)
                    } else {
                        Event::RefreshEditorContent
                    }
                })
            }
            Event::OpenFile => {
                Task::perform(open_file(Editor::new_from(self)), |r| {
                    Event::Opened(r)
                })
            }
            Event::Saved => Task::none(),
            Event::Quited(_path) => {
                for (i, f) in self.files.iter().enumerate() {
                    if f.path.to_str().unwrap() == _path.to_str().unwrap() {
                        self.files.remove(i);

                        if self.active_file > (self.files.len() / 2) {
                            self.active_file = self.files.len() - 1;
                        } else {
                            self.active_file = 0;
                        }
                        break;
                    }
                }
                Task::done(Event::RefreshEditorContent)
            }
            Event::Opened(Option::Some((path, content))) => {
                self.files.push(FileInfo::new(path, content));
                self.active_file = self.files.len() - 1;
                Task::done(Event::RefreshEditorContent)
            }
            Event::Opened(Option::None) => Task::none(),
            Event::RefreshEditorContent => {
                if self.files.len() > 0 {
                    self.content = text_editor::Content::with_text(
                        self.files[self.active_file].content.as_str(),
                    );
                } else {
                    self.content = text_editor::Content::new();
                }
                Task::none()
            }
            Event::ScanFile(path) => {
                let idx = 
                    scan_file(Editor::new_from(self), path);
                Task::perform(idx, |need_refresh| {
                    if need_refresh {
                        Event::RefreshEditorContent
                    } else {
                        Event::None
                    }
                })
            }
            Event::ScanAllFiles => {
                self.files.iter_mut().for_each(|f| {
                    f.content = std::fs::read_to_string(f.path.clone())
                        .expect("Unable to read file");
                });

                Task::done(Event::RefreshEditorContent)
            }
            Event::TabSelected(idx) => {
                self.active_file = idx;
                Task::done(Event::RefreshEditorContent)
            }
            Event::TabClosed(idx) => Task::done(Event::Quit(Some(idx))),
            Event::NewTab => Task::done(Event::OpenFile),
            Event::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Event::None => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Event> {
        keyboard::on_key_release(|k,m| {
            let map = get_global_hashmap();
            
            if let keyboard::Key::Character(c) = k {
                // Get the first character of the key
                let key = &c.as_str().chars().next().unwrap();
                
                if let Some(bindings) = map.get(key) {
                    for binding in bindings {
                        if binding.ctrl == m.control() && binding.shift == m.shift() {
                            return Some(binding.event.clone());
                        }
                    }
                }
                
            }
            
            None
        })
    }

    fn view(&self) -> Element<'_, Event> {
        let cursor = self.content.cursor_position();
        let num_lines = self.content.line_count();
        
        let tabs = column!(
            self.files
            .iter()
            .fold(TabBar::new(Event::TabSelected), |tab_bar, info| {
                let idx = tab_bar.size();
                let content = info.path.to_str().unwrap().split("/").last()
                    .expect("Unable to get file name").to_string();
                tab_bar.push(
                    idx,
                    TabLabel::Text(content),
                )
            })
            .set_active_tab(&self.active_file)
            .on_close(Event::TabClosed)
            .spacing(1.0)
            .padding(2.0)
            .width(Length::Fill)
            .height(Length::Shrink)
        ).width(Length::Fill)
        .padding(5);

        let languaje = 
        if let Some(info) = self.files.get(self.active_file) {
            if let Some(ext) = info.path.extension() {
                    ext.to_str().unwrap().to_lowercase()
            }else{
                    "txt".to_string()
            }
        }else{
                "txt".to_string()
        };
        let mut editor = 
            text_editor(&self.content)
            .font(Font::MONOSPACE)
            .highlight(languaje.as_str(), self.theme.clone())
            .style(|th: &Theme, _st| {
                text_editor::Style {
                    background: Background::Color(th.palette().background),
                    border: Border::default()
                        .color(th.palette().primary)
                        .rounded(8.0)
                        .width(1.5),
                    icon: Color::WHITE,
                    placeholder: color!(0xc2c2c2),
                    value: color!(0xefefef),
                    selection: color!(0x525282),
                }
            })
            .height(Length::Fill);

        if self.files.len() > 0 {
            editor = editor.on_action(Event::EditorAction);
        }
        
        let indicator = text(format!(
            "Lines {num_lines} | Cursor: {} {}",
            cursor.0, cursor.1
        ))
        .color(iced::color!(0xc2c2c2))
        .font(Font::DEFAULT)
        .line_height(text::LineHeight::Relative(2.0))
        .align_x(Horizontal::Right)
        .width(Length::Fill);

        container(
            column![tabs, editor, indicator]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
    }
}
