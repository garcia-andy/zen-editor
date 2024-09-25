use iced::Task;
use zen_core::{Editor, KeyBinding, Event, Register, ZenCore};

fn main() {
    let _ = iced::application("Zen editor", ZenCore::update, ZenCore::view)
        .subscription(ZenCore::subscription)
        .theme(ZenCore::theme)
        .font(include_bytes!("../fonts/fontello/fontello.ttf"))
        .font(iced_aw::iced_fonts::REQUIRED_FONT_BYTES)
        .run_with(
            || -> (ZenCore, Task<Event>) { 
                let mut editor = Editor::new();
                editor.add_keys_bindings(vec![
                    KeyBinding::new('s', true, false, false, Event::Save),
                    KeyBinding::new('q', true, false, false, Event::Quit(None)),
                    KeyBinding::new('o', true, false, false, Event::OpenFile),
                    KeyBinding::new('r', true, false, false, Event::ScanAllFiles),
                    KeyBinding::new('r', true, true,  false, Event::ScanFile(None)),
                ]);
                
                let top_menu = Box::new(zen_core::TopMenu::new());
                
                (
                    ZenCore::new_with(
                        "Zen Editor (v1)".to_string(), 
                        iced::Theme::Dark, 
                        vec![top_menu, Box::new(editor)]
                    ),
                    Task::none()
                )
            });
}
