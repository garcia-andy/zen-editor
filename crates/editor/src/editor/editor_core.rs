use iced::{
    alignment::Horizontal, color, highlighter, 
    widget::{column, container, responsive, text, text_editor}, 
    Element, Font, Length, Task
};
use iced_aw::{ TabBar, TabLabel};
use registers::{ Event, Register};
use ui::styles;

use crate::fileinfo::FileInfo;
use crate::services::*;

#[derive(Debug)]
pub struct EditorCore {
    content: text_editor::Content,
    pub files: Vec<FileInfo>,
    pub active_file: usize,
    pub theme: highlighter::Theme,
}

impl Clone for EditorCore {
    fn clone(&self) -> Self {
        Self {
            content: text_editor::Content::with_text(self.get_content().as_str()),
            files: self.files.clone(),
            active_file: self.active_file,
            theme: self.theme.clone(),
        }
    }
}

impl EditorCore {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            files: Vec::new(),
            active_file: 0,
            theme: highlighter::Theme::Base16Ocean,
        }
    }

    pub fn new_from(t: &Self) -> Self {
        let this = Self {
            content: text_editor::Content::with_text(t.get_content().as_str()),
            files: t.files.clone(),
            active_file: t.active_file,
            theme: t.theme.clone(),
        };
        this
    }

    pub fn get_content(&self) -> String {
        self.content.text()
    }
    
}

impl Register for EditorCore {
    fn update(&mut self, _event: Event) -> Task<Event> {
        match _event {
            Event::EditorAction(action) => {
                self.content.perform(action);
                self.files[self.active_file].content = self.content.text();
                Task::none()
            }
            Event::Save => {
                Task::perform(save_content(EditorCore::new_from(self)), |_| {
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
                    quit_file(EditorCore::new_from(self), i), 
                    |path| {
                    if let Some(path) = path {
                        Event::Quited(path)
                    } else {
                        Event::RefreshEditorContent
                    }
                })
            }
            Event::OpenFile => {
                Task::perform(open_file(EditorCore::new_from(self)), |r| {
                    Event::Opened(r)
                })
            }
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
                    scan_file(EditorCore::new_from(self), path);
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
            },
            _ => Task::none(),
        }
    }
    
    fn view(&self) -> Element<'_, Event> {
        let cursor = self.content.cursor_position();
        let num_lines = self.content.line_count();
        let files = &self.files;
        let active_file = self.active_file;
        let theme = self.theme.clone();
        let content = &self.content;
        
        container(
            create_editor(
                cursor, num_lines, files.clone(), 
                active_file, theme, content
            )
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
    }
}

fn create_editor( 
    cursor: (usize, usize), 
    num_lines: usize,
    files: Vec<FileInfo>, 
    active_file: usize, 
    theme: highlighter::Theme, 
    content: &text_editor::Content
) -> Element<'_, Event> {
    responsive(move |s| {
        let tabs = column!(
            files
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
            .set_active_tab(&active_file)
            .on_close(Event::TabClosed)
            .spacing(1.0)
            .padding(2.0)
            .width(Length::Fill)
            .height(Length::Shrink)
        ).width(Length::Fill)
        .padding(5);
    
        let languaje = 
        if let Some(info) = files.get(active_file) {
            if let Some(ext) = info.path.extension() {
                    ext.to_str().unwrap().to_lowercase()
            }else{
                    "txt".to_string()
            }
        }else{
                "txt".to_string()
        };
        let mut editor = 
            text_editor(&content)
            .font(Font::MONOSPACE)
            .highlight(languaje.as_str(), theme.clone())
            .style(styles::editor_style)
            .height(Length::Fill);
    
        if files.len() > 0 {
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
        
        column![tabs, editor, indicator]
            .width(Length::FillPortion(s.width as u16))
            .height(Length::FillPortion(s.height as u16))
            .align_x(Horizontal::Center)
            .padding(5)
            .into()
    })
        .into()
}

