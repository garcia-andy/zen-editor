use iced::{
    alignment::{self, Horizontal}, color, keyboard, widget::{button, column, container, row, text, text_editor}, Background, Border, Color, Element, Font, Length, Subscription, Task, Theme
};
use iced_aw::{ menu::{self, Item, Menu}, menu_bar, menu_items, TabBar, TabLabel};
use registers::{ Event, Register};
use std::path::PathBuf;

pub const ICON: Font = Font::with_name("FontAwesome");

enum Icon {
    File,
    Save,
    Refresh,
}

fn icon_code(codepoint: char) -> Element<'static, Event> {
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

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub content: String,
}

impl FileInfo {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }
}

pub struct Editor {
    content: text_editor::Content,
    pub files: Vec<FileInfo>,
    pub active_file: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            files: Vec::new(),
            active_file: 0,
        }
    }

    pub fn new_from(t: &Self) -> Self {
        Self {
            content: text_editor::Content::with_text(t.get_content().as_str()),
            files: t.files.clone(),
            active_file: t.active_file,
        }
    }

    pub fn copy(&mut self, other: &Self) {
        self.files = other.files.clone();
        self.active_file = other.active_file;
        self.content =
            text_editor::Content::with_text(other.get_content().as_str());
    }

    pub fn get_content(&self) -> String {
        self.content.text()
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
                Task::perform(quit_file(Editor::new_from(self), i), |path| {
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
                let path = if let Some(p) = path {
                    p
                } else {
                    self.files[self.active_file].path.clone()
                };
                let mut idx = 0;
                for (i, f) in self.files.iter().enumerate() {
                    if f.path.to_str().unwrap() == path.to_str().unwrap() {
                        self.files[i].content =
                            std::fs::read_to_string(f.path.clone())
                                .expect("Unable to read file");
                        idx = i;
                        break;
                    }
                }
                if idx == self.active_file {
                    Task::done(Event::RefreshEditorContent)
                } else {
                    Task::none()
                }
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
        let menu_tpl_1 = |items| 
            Menu::new(items).max_width(120.0).offset(15.0).spacing(5.0);
        
        let cursor = self.content.cursor_position();
        let num_lines = self.content.line_count();

        let menu = menu_bar!(
            (debug_button_s("File"), {
                menu_tpl_1(menu_items!(
                    (button_with_icon(icon_code(Icon::File.into()), "Open", Event::OpenFile))
                    (button_with_icon(icon_code(Icon::Save.into()), "Save", Event::Save))
                    (button_with_icon(icon_code(Icon::Refresh.into()), "Reload", Event::ScanAllFiles))
                ))
            })
        ).draw_path(menu::DrawPath::Backdrop);

        let tabs = column!(
            self.files
            .iter()
            .fold(TabBar::new(Event::TabSelected), |tab_bar, info| {
                let idx = tab_bar.size();
                tab_bar.push(
                    idx,
                    TabLabel::Text(info.path.to_str().unwrap().to_string().split("/").last().unwrap().to_string()),
                )
            })
            .set_active_tab(&self.active_file)
            .on_close(Event::TabClosed)
            .spacing(1.0)
            .padding(2.0)
            .icon_font(ICON)
            .width(Length::Fill)
            .height(Length::Shrink)
        ).width(Length::Fill)
        .padding(5);

        
        let mut editor = text_editor(&self.content)
            .font(Font::MONOSPACE)
            .style(|th: &Theme, _st| {
                let style = text_editor::Style {
                    background: Background::Color(th.palette().background),
                    border: Border::default()
                        .color(th.palette().success)
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
            column![menu,  tabs, editor, indicator]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
    }
}

async fn open_file(mut this: Editor) -> Option<(PathBuf, String)> {
    let file = rfd::FileDialog::new().pick_file();
    if let Some(file) = file {
        load_file(&mut this, Some(file)).await;
        this.active_file = this.files.len() - 1;
        let info = this.files[this.active_file].clone();
        Some((info.path, info.content))
    } else {
        None
    }
}

async fn load_file(this: &mut Editor, path: Option<PathBuf>) {
    let file_path = if let Some(p) = path {
        p
    } else {
        rfd::FileDialog::new().pick_file().unwrap()
    };

    let content = std::fs::read_to_string(&file_path).unwrap();
    this.files.push(FileInfo::new(file_path, content));
}

pub async fn save_file(this: &mut Editor, path: Option<PathBuf>) {
    let file_path = if let Some(p) = path {
        p
    } else {
        rfd::FileDialog::new().save_file().unwrap()
    };

    let mut idx = this.files.len() + 1;
    for (i, f) in this.files.iter().enumerate() {
        if f.path.to_str().unwrap() == file_path.to_str().unwrap() {
            idx = i;
            break;
        }
    }

    if idx < this.files.len() {
        let content = this.files[idx].content.clone();
        std::fs::write(file_path.clone(), content)
            .expect("Unable to write file changes");
    } else {
        println!("File not found {:?} on {:?}", file_path, this.files);
    }
}

pub async fn quit_file(mut this: Editor, idx: usize) -> Option<PathBuf> {
    if this.files.len() >= 1 {
        let ret = this.files.remove(idx);

        if idx == this.active_file {
            if idx > (this.files.len() / 2) {
                this.active_file = this.files.len() - 1;
            } else {
                this.active_file = 0;
            }
        }

        Some(ret.path)
    } else {
        None
    }
}

pub async fn save_content(mut this: Editor) {
    if this.files.len() >= 1 {
        let buf = this.files[this.active_file].clone();
        save_file(&mut this, Some(buf.path)).await;
    }
}


/* BUTTONS */


fn base_button<'a>(
    content: impl Into<Element<'a, Event>>,
    msg: Event,
) -> button::Button<'a, Event> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

fn button_with_icon(
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

fn labeled_button(
    label: &str,
    msg: Event,
) -> button::Button<Event, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn debug_button_s(label: &str) -> button::Button<Event, iced::Theme, iced::Renderer> {
    labeled_button(label, Event::None).width(Length::Shrink)
}

