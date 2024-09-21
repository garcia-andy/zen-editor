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
                    KeyBinding::new('s', true, false, Event::Save),
                    KeyBinding::new('q', true, false, Event::Quit(None)),
                    KeyBinding::new('o', true, false, Event::OpenFile),
                    KeyBinding::new('r', true, false, Event::ScanAllFiles),
                    KeyBinding::new('r', true, true, Event::ScanFile(None)),
                ]);
                (
                    ZenCore::new_with(
                        "Zen Editor (v1)".to_string(), 
                        iced::Theme::Dracula, 
                        vec![Box::new(editor)]
                    ),
                    Task::none()
                )
            });
}
