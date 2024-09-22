use iced::widget::{button, container, text_editor};
use iced::{Border, Theme};

pub fn tooltip_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.strong.color.into()),
        border: Border::default()
            .color(palette.background.strong.color)
            .rounded(3.14)
            .width(0.5),
        ..Default::default()
    }
}

pub fn tooltip_danger_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.danger.strong.color.into()),
        border: Border::default()
            .color(palette.background.strong.color)
            .rounded(3.14)
            .width(1.0),
        ..Default::default()
    }
}

pub fn editor_style(theme: &Theme, _st: text_editor::Status) -> text_editor::Style {
    let palette = theme.extended_palette();
    let basic = theme.palette();
    text_editor::Style {
        background: basic.background.into(),
        border: Border::default()
            .rounded(3.14)
            .color(palette.secondary.weak.color)
            .width(0.5),
        icon: palette.primary.weak.color.into(),
        placeholder: palette.secondary.weak.color.into(),
        value: basic.text,
        selection: palette.secondary.weak.color.into(),
    }
}

pub fn button_danger_style(theme: &Theme) -> button::Style {
    let palette = theme.extended_palette();
    let style = button::Style {
        background: Some(palette.danger.weak.color.into()),
        border: Border::default()
            .color(palette.danger.strong.color)
            .rounded(2.0 * 3.14)
            .width(1.0),
        ..Default::default()
    };
    style
}

pub fn button_styles(theme: &Theme, st: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style {
        background: Some(palette.background.strong.color.into()),
        border: Border::default()
            .color(palette.background.strong.color)
            .rounded(2.0 * 3.14)
            .width(1.0),
        ..Default::default()
    };
    match st{
        button::Status::Pressed => {
            style.background = Some(palette.primary.weak.color.into());
        },
        button::Status::Hovered => {
            style.background = Some(palette.primary.strong.color.into());
        },
        button::Status::Active=> {},
        button::Status::Disabled => {   
            style.background = Some(palette.secondary.weak.color.into());
        }
    };
    style
}


pub fn title_bar_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.background.strong.text),
        background: Some(palette.background.strong.color.into()),
        ..Default::default()
    }
}

pub fn title_bar_focused(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.primary.strong.text),
        background: Some(palette.primary.strong.color.into()),
        ..Default::default()
    }
}

pub fn pane_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    let basic = theme.palette();
    container::Style {
        background: Some(basic.background.into()),
        border: Border {
            width: 1.0,
            color: palette.secondary.weak.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

