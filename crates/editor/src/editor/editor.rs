use std::collections::HashMap;

use iced::{
    alignment::{Horizontal, Vertical},
    color,
    keyboard,
    widget::{column, container, row, text, text_editor},
    Background,
    Border,
    Color,
    Element,
    Font,
    Length,
    Subscription,
    Task,
    Theme
};
use iced_aw::{ TabBar, TabLabel};
use registers::{ Event, Register};

use crate::fileinfo::FileInfo;
use crate::services::*;
use crate::gui::*;

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

pub struct Editor {
    content: text_editor::Content,
    pub files: Vec<FileInfo>,
    pub active_file: usize,
    pub key_bindings: Vec<KeyBinding>,
    pub key_bindings_map: HashMap<char, Vec<KeyBinding>>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            files: Vec::new(),
            active_file: 0,
            key_bindings: Vec::new(),
            key_bindings_map: HashMap::new(),
        }
    }

    pub fn new_from(t: &Self) -> Self {
        Self {
            content: text_editor::Content::with_text(t.get_content().as_str()),
            files: t.files.clone(),
            active_file: t.active_file,
            key_bindings: t.key_bindings.clone(),
            key_bindings_map: t.key_bindings_map.clone(),
        }
    }

    pub fn get_content(&self) -> String {
        self.content.text()
    }
    
    pub fn get_key_bindings(&self) -> Vec<KeyBinding> {
        self.key_bindings.clone()
    }

    pub fn get_key_bindings_map(&self) -> HashMap<char, Vec<KeyBinding>> {
        self.key_bindings_map.clone()
    }
    
    pub fn add_key_binding(&mut self, key: char, ctrl: bool, shift: bool, event: Event) {
        self.key_bindings.push(KeyBinding::new(key, ctrl, shift, event.clone()));
        self.key_bindings_map.entry(key).or_insert(Vec::new()).push(KeyBinding::new(key, ctrl, shift, event));
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
            Event::None => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Event> {
        keyboard::on_key_release(|k, modif| -> Option<Event> {
            if modif.control() {
                if modif.shift() {
                    match k.as_ref() {
                        keyboard::Key::Character("r") => {
                            Some(Event::ScanAllFiles)
                        }
                        _ => None,
                    }
                } else {
                    match k.as_ref() {
                        keyboard::Key::Character("s") => Some(Event::Save),
                        keyboard::Key::Character("q") => {
                            Some(Event::Quit(None))
                        }
                        keyboard::Key::Character("o") => Some(Event::OpenFile),
                        keyboard::Key::Character("r") => {
                            Some(Event::ScanFile(None))
                        }
                        _ => None,
                    }
                }
            } else {
                None
            }
        })
    }

    fn view(&self) -> Element<'_, Event> {
        let cursor = self.content.cursor_position();
        let num_lines = self.content.line_count();

        let buttons = row![
            button_with_icon(icon_code(Icon::File.into()), "Open", Event::OpenFile),
            button_with_icon(icon_code(Icon::Save.into()), "Save", Event::Save),
            button_with_icon(icon_code(Icon::Refresh.into()), "Reload", Event::ScanAllFiles),
        ].spacing(5).align_y(Vertical::Center).padding(2).width(Length::Fill);
        
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

        
        let mut editor = 
            text_editor(&self.content)
            .font(Font::MONOSPACE)
            .style(|th: &Theme, _st| {
                let style = text_editor::Style {
                    background: Background::Color(th.palette().background),
                    border: Border::default()
                        .color(th.palette().primary)
                        .rounded(8.0)
                        .width(1.5),
                    icon: Color::WHITE,
                    placeholder: color!(0xc2c2c2),
                    value: color!(0xefefef),
                    selection: color!(0x525282),
                };
                style
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
            column![buttons, tabs, editor, indicator]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
    }
}
