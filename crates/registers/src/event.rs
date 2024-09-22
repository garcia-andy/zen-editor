use std::path::PathBuf;

use iced::{highlighter, widget::text_editor};


#[derive(Debug, Clone)]
pub enum Event {
    None,
    Save,
    Saved,
    OpenFile,
    Opened(Option<(PathBuf, String)>),
    Quit(Option<usize>),
    Quited(PathBuf),
    RefreshEditorContent,
    ScanAllFiles,
    ScanFile(Option<PathBuf>),
    EditorAction(text_editor::Action),
    
    TabSelected(usize),
    TabClosed(usize),
    NewTab,
    
    ThemeChanged(highlighter::Theme),
}
