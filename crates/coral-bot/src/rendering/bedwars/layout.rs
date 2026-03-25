pub const CANVAS_WIDTH: u32 = 800;
pub const CANVAS_HEIGHT: u32 = 600;
pub const COL_WIDTH: u32 = 256;

pub const HEADER_Y: u32 = 0;
pub const HEADER_HEIGHT: u32 = 100;
pub const LEVEL_Y: u32 = 57;

pub const MAIN_ROW_Y: u32 = 116;
pub const MAIN_BOX_HEIGHT: u32 = 176;
pub const SKIN_BOX_HEIGHT: u32 = 368;
pub const SECOND_ROW_Y: u32 = 308;

pub const BOTTOM_ROW_Y: u32 = 500;
pub const BOTTOM_BOX_HEIGHT: u32 = 100;


pub fn col_x(col: u32) -> u32 {
    match col {
        0 => 0,
        1 => 272,
        2 => 544,
        _ => 0,
    }
}
