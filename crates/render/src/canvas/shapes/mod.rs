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


pub(crate) fn is_outside_rounded_rect(px: u32, py: u32, w: u32, h: u32, r: u32) -> bool {
    if r == 0 {
        return false;
    }
    let check = |in_corner: bool, dx: u32, dy: u32| -> Option<bool> {
        in_corner.then(|| dx * dx + dy * dy > r * r)
    };
    check(px < r && py < r, r - px, r - py)
        .or_else(|| check(px >= w - r && py < r, px - (w - r - 1), r - py))
        .or_else(|| check(px < r && py >= h - r, r - px, py - (h - r - 1)))
        .or_else(|| check(px >= w - r && py >= h - r, px - (w - r - 1), py - (h - r - 1)))
        .unwrap_or(false)
}
