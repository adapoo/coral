mod column;
mod image;
mod rect;
mod row;
mod spacer;
mod text;
mod text_box;

pub use column::Column;
pub use image::Image;
pub use rect::RoundedRect;
pub use row::Row;
pub use spacer::Spacer;
pub use text::{Text, TextBlock};
pub use text_box::{Align, TextBox};

pub(crate) fn is_outside_rounded_rect(
    px: u32,
    py: u32,
    width: u32,
    height: u32,
    radius: u32,
) -> bool {
    if radius == 0 {
        return false;
    }

    let in_top_left = px < radius && py < radius;
    let in_top_right = px >= width - radius && py < radius;
    let in_bottom_left = px < radius && py >= height - radius;
    let in_bottom_right = px >= width - radius && py >= height - radius;

    if in_top_left {
        let dx = radius - px;
        let dy = radius - py;
        return dx * dx + dy * dy > radius * radius;
    }

    if in_top_right {
        let dx = px - (width - radius - 1);
        let dy = radius - py;
        return dx * dx + dy * dy > radius * radius;
    }

    if in_bottom_left {
        let dx = radius - px;
        let dy = py - (height - radius - 1);
        return dx * dx + dy * dy > radius * radius;
    }

    if in_bottom_right {
        let dx = px - (width - radius - 1);
        let dy = py - (height - radius - 1);
        return dx * dx + dy * dy > radius * radius;
    }

    false
}
