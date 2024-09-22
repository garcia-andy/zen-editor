use std::path::PathBuf;
use iced::{highlighter, widget::{pane_grid, text_editor}};


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
    
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Close(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Split(pane_grid::Axis, pane_grid::Pane),
    
    ThemeChanged(highlighter::Theme),
}
