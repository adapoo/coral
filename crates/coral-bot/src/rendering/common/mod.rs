pub mod colors;
pub mod format;
pub mod prestige;

pub use colors::*;
pub use format::*;
pub use prestige::*;

use mctext::{MCText, NamedColor};

pub fn stat_line(label: &str, value: &str, color: NamedColor) -> MCText {
    MCText::new()
        .span(label)
        .color(NamedColor::Gray)
        .then(value)
        .color(color)
        .build()
}

pub fn color_name_to_named(name: &str) -> Option<NamedColor> {
    match name.to_uppercase().as_str() {
        "DARK_GREEN" => Some(NamedColor::DarkGreen),
        "DARK_AQUA" => Some(NamedColor::DarkAqua),
        "DARK_RED" => Some(NamedColor::DarkRed),
        "DARK_PURPLE" => Some(NamedColor::DarkPurple),
        "GOLD" => Some(NamedColor::Gold),
        "GRAY" => Some(NamedColor::Gray),
        "DARK_GRAY" => Some(NamedColor::DarkGray),
        "BLUE" => Some(NamedColor::Blue),
        "GREEN" => Some(NamedColor::Green),
        "AQUA" => Some(NamedColor::Aqua),
        "RED" => Some(NamedColor::Red),
        "LIGHT_PURPLE" => Some(NamedColor::LightPurple),
        "YELLOW" => Some(NamedColor::Yellow),
        "WHITE" => Some(NamedColor::White),
        _ => None,
    }
}
