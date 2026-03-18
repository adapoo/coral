use chrono::{DateTime, Utc};
use image::Rgba;
use mctext::{MCText, NamedColor};

use crate::canvas::{DrawContext, blend};

pub mod colors {
    use mctext::NamedColor;

    pub fn wlr(value: f64) -> NamedColor {
        match value {
            v if v >= 30.0 => NamedColor::DarkPurple,
            v if v >= 15.0 => NamedColor::LightPurple,
            v if v >= 9.0 => NamedColor::DarkRed,
            v if v >= 6.0 => NamedColor::Red,
            v if v >= 3.0 => NamedColor::Gold,
            v if v >= 2.1 => NamedColor::Yellow,
            v if v >= 1.5 => NamedColor::DarkGreen,
            v if v >= 0.9 => NamedColor::Green,
            v if v >= 0.3 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn fkdr(value: f64) -> NamedColor {
        match value {
            v if v >= 100.0 => NamedColor::DarkPurple,
            v if v >= 50.0 => NamedColor::LightPurple,
            v if v >= 30.0 => NamedColor::DarkRed,
            v if v >= 20.0 => NamedColor::Red,
            v if v >= 10.0 => NamedColor::Gold,
            v if v >= 7.0 => NamedColor::Yellow,
            v if v >= 5.0 => NamedColor::DarkGreen,
            v if v >= 3.0 => NamedColor::Green,
            v if v >= 1.0 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn kdr(value: f64) -> NamedColor {
        match value {
            v if v >= 8.0 => NamedColor::DarkPurple,
            v if v >= 7.0 => NamedColor::LightPurple,
            v if v >= 6.0 => NamedColor::DarkRed,
            v if v >= 5.0 => NamedColor::Red,
            v if v >= 4.0 => NamedColor::Gold,
            v if v >= 3.0 => NamedColor::Yellow,
            v if v >= 2.0 => NamedColor::DarkGreen,
            v if v >= 1.0 => NamedColor::Green,
            v if v >= 0.5 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn bblr(value: f64) -> NamedColor {
        match value {
            v if v >= 20.0 => NamedColor::DarkPurple,
            v if v >= 10.0 => NamedColor::LightPurple,
            v if v >= 6.0 => NamedColor::DarkRed,
            v if v >= 4.0 => NamedColor::Red,
            v if v >= 2.0 => NamedColor::Gold,
            v if v >= 1.4 => NamedColor::Yellow,
            v if v >= 1.0 => NamedColor::DarkGreen,
            v if v >= 0.6 => NamedColor::Green,
            v if v >= 0.2 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn session_wlr(value: f64) -> NamedColor {
        match value {
            v if v >= 150.0 => NamedColor::DarkPurple,
            v if v >= 75.0 => NamedColor::LightPurple,
            v if v >= 45.0 => NamedColor::DarkRed,
            v if v >= 30.0 => NamedColor::Red,
            v if v >= 15.0 => NamedColor::Gold,
            v if v >= 10.5 => NamedColor::Yellow,
            v if v >= 7.5 => NamedColor::DarkGreen,
            v if v >= 4.5 => NamedColor::Green,
            v if v >= 1.5 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn session_fkdr(value: f64) -> NamedColor {
        match value {
            v if v >= 500.0 => NamedColor::DarkPurple,
            v if v >= 250.0 => NamedColor::LightPurple,
            v if v >= 150.0 => NamedColor::DarkRed,
            v if v >= 100.0 => NamedColor::Red,
            v if v >= 50.0 => NamedColor::Gold,
            v if v >= 35.0 => NamedColor::Yellow,
            v if v >= 25.0 => NamedColor::DarkGreen,
            v if v >= 15.0 => NamedColor::Green,
            v if v >= 5.0 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn session_bblr(value: f64) -> NamedColor {
        match value {
            v if v >= 100.0 => NamedColor::DarkPurple,
            v if v >= 50.0 => NamedColor::LightPurple,
            v if v >= 30.0 => NamedColor::DarkRed,
            v if v >= 20.0 => NamedColor::Red,
            v if v >= 10.0 => NamedColor::Gold,
            v if v >= 7.0 => NamedColor::Yellow,
            v if v >= 5.0 => NamedColor::DarkGreen,
            v if v >= 3.0 => NamedColor::Green,
            v if v >= 1.0 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn wins(value: u64) -> NamedColor {
        match value {
            v if v >= 30000 => NamedColor::DarkPurple,
            v if v >= 15000 => NamedColor::LightPurple,
            v if v >= 7500 => NamedColor::DarkRed,
            v if v >= 4500 => NamedColor::Red,
            v if v >= 2250 => NamedColor::Gold,
            v if v >= 1500 => NamedColor::Yellow,
            v if v >= 450 => NamedColor::DarkGreen,
            v if v >= 300 => NamedColor::Green,
            v if v >= 150 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn final_kills(value: u64) -> NamedColor {
        match value {
            v if v >= 100000 => NamedColor::DarkPurple,
            v if v >= 50000 => NamedColor::LightPurple,
            v if v >= 25000 => NamedColor::DarkRed,
            v if v >= 15000 => NamedColor::Red,
            v if v >= 7500 => NamedColor::Gold,
            v if v >= 5000 => NamedColor::Yellow,
            v if v >= 2500 => NamedColor::DarkGreen,
            v if v >= 1000 => NamedColor::Green,
            v if v >= 500 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn kills(value: u64) -> NamedColor {
        match value {
            v if v >= 75000 => NamedColor::DarkPurple,
            v if v >= 37500 => NamedColor::LightPurple,
            v if v >= 18750 => NamedColor::DarkRed,
            v if v >= 11250 => NamedColor::Red,
            v if v >= 5625 => NamedColor::Gold,
            v if v >= 3750 => NamedColor::Yellow,
            v if v >= 1875 => NamedColor::DarkGreen,
            v if v >= 750 => NamedColor::Green,
            v if v >= 375 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn beds_broken(value: u64) -> NamedColor {
        match value {
            v if v >= 50000 => NamedColor::DarkPurple,
            v if v >= 25000 => NamedColor::LightPurple,
            v if v >= 12500 => NamedColor::DarkRed,
            v if v >= 7500 => NamedColor::Red,
            v if v >= 3750 => NamedColor::Gold,
            v if v >= 2500 => NamedColor::Yellow,
            v if v >= 1250 => NamedColor::DarkGreen,
            v if v >= 500 => NamedColor::Green,
            v if v >= 250 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }

    pub fn winstreak(value: u64) -> NamedColor {
        match value {
            v if v >= 500 => NamedColor::DarkPurple,
            v if v >= 250 => NamedColor::LightPurple,
            v if v >= 100 => NamedColor::DarkRed,
            v if v >= 75 => NamedColor::Red,
            v if v >= 50 => NamedColor::Gold,
            v if v >= 40 => NamedColor::Yellow,
            v if v >= 25 => NamedColor::DarkGreen,
            v if v >= 15 => NamedColor::Green,
            v if v >= 5 => NamedColor::White,
            _ => NamedColor::Gray,
        }
    }
}

pub fn format_ratio(value: f64) -> String {
    let s = format!("{:.2}", value);
    s.strip_suffix(".00").map(String::from).unwrap_or(s)
}

pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

pub fn format_percent(value: f64) -> String {
    let s = format!("{:.1}", value);
    let s = s.strip_suffix(".0").unwrap_or(&s);
    format!("{}%", s)
}

pub fn format_timestamp(ts: i64) -> String {
    let ts_millis = if ts > 10_000_000_000 { ts } else { ts * 1000 };
    DateTime::<Utc>::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%m/%d/%y").to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

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

pub const BAR_COLOR: Rgba<u8> = Rgba([50, 50, 55, 220]);

pub fn draw_progress_bar(
    ctx: &mut DrawContext,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    radius: u32,
    fill_frac: f64,
    bg: Rgba<u8>,
    fill: Rgba<u8>,
) {
    let (cw, ch) = ctx.buffer.dimensions();
    let r = radius.min(width / 2).min(height / 2);
    let fill_w = (fill_frac.clamp(0.0, 1.0) * width as f64).round() as u32;

    for py in 0..height {
        for px in 0..width {
            let abs_x = (ctx.x + x as i32 + px as i32) as u32;
            let abs_y = (ctx.y + y as i32 + py as i32) as u32;
            if abs_x >= cw || abs_y >= ch {
                continue;
            }
            if is_outside_rounded(px, py, width, height, r) {
                continue;
            }

            let color = if px < fill_w { fill } else { bg };
            let base = *ctx.buffer.get_pixel(abs_x, abs_y);
            ctx.buffer.put_pixel(abs_x, abs_y, blend(base, color));
        }
    }
}

fn is_outside_rounded(px: u32, py: u32, w: u32, h: u32, r: u32) -> bool {
    if r == 0 {
        return false;
    }
    if px < r && py < r {
        let dx = r - px;
        let dy = r - py;
        return dx * dx + dy * dy > r * r;
    }
    if px >= w - r && py < r {
        let dx = px - (w - r - 1);
        let dy = r - py;
        return dx * dx + dy * dy > r * r;
    }
    if px < r && py >= h - r {
        let dx = r - px;
        let dy = py - (h - r - 1);
        return dx * dx + dy * dy > r * r;
    }
    if px >= w - r && py >= h - r {
        let dx = px - (w - r - 1);
        let dy = py - (h - r - 1);
        return dx * dx + dy * dy > r * r;
    }
    false
}
