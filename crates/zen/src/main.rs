use zen_core::{Register, ZenCore};

fn main() {
    let _ = iced::application("Zen editor", ZenCore::update, ZenCore::view)
        .subscription(ZenCore::subscription)
        .theme(ZenCore::theme)
        .font(include_bytes!("../fonts/fontawesome-webfont.ttf"))
        .font(iced_aw::iced_fonts::REQUIRED_FONT_BYTES)
        .run();
}
