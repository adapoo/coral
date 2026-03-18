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
