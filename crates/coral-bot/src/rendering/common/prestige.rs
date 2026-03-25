use mctext::{MCText, NamedColor};


pub fn prestige_colors(level: u32) -> Vec<NamedColor> {
    use NamedColor::*;
    match level {
        0..=99 => vec![Gray; 6],
        100..=199 => vec![White; 6],
        200..=299 => vec![Gold; 6],
        300..=399 => vec![Aqua; 6],
        400..=499 => vec![DarkGreen; 6],
        500..=599 => vec![DarkAqua; 6],
        600..=699 => vec![DarkRed; 6],
        700..=799 => vec![LightPurple; 6],
        800..=899 => vec![Blue; 6],
        900..=999 => vec![DarkPurple; 6],
        1000..=1099 => vec![Red, Gold, Yellow, Green, Aqua, LightPurple, DarkPurple],
        1100..=1199 => vec![Gray, White, White, White, White, Gray, Gray],
        1200..=1299 => vec![Gray, Yellow, Yellow, Yellow, Yellow, Gold, Gray],
        1300..=1399 => vec![Gray, Aqua, Aqua, Aqua, Aqua, DarkAqua, Gray],
        1400..=1499 => vec![Gray, Green, Green, Green, Green, DarkGreen, Gray],
        1500..=1599 => vec![Gray, DarkAqua, DarkAqua, DarkAqua, DarkAqua, Blue, Gray],
        1600..=1699 => vec![Gray, Red, Red, Red, Red, DarkRed, Gray],
        1700..=1799 => vec![Gray, LightPurple, LightPurple, LightPurple, LightPurple, DarkPurple, Gray],
        1800..=1899 => vec![Gray, Blue, Blue, Blue, Blue, DarkBlue, Gray],
        1900..=1999 => vec![Gray, DarkPurple, DarkPurple, DarkPurple, DarkPurple, DarkGray, Gray],
        2000..=2099 => vec![DarkGray, Gray, White, White, Gray, Gray, DarkGray],
        2100..=2199 => vec![White, White, Yellow, Yellow, Gold, Gold, Gold],
        2200..=2299 => vec![Gold, Gold, White, White, Aqua, DarkAqua, DarkAqua],
        2300..=2399 => vec![DarkPurple, DarkPurple, LightPurple, LightPurple, Gold, Yellow, Yellow],
        2400..=2499 => vec![Aqua, Aqua, White, White, Gray, Gray, DarkGray],
        2500..=2599 => vec![White, White, Green, Green, DarkGreen, DarkGreen, DarkGreen],
        2600..=2699 => vec![DarkRed, DarkRed, Red, Red, LightPurple, LightPurple, LightPurple],
        2700..=2799 => vec![Yellow, Yellow, White, White, DarkGray, DarkGray, DarkGray],
        2800..=2899 => vec![Green, Green, DarkGreen, DarkGreen, Gold, Gold, Yellow],
        2900..=2999 => vec![Aqua, Aqua, DarkAqua, DarkAqua, Blue, Blue, DarkBlue],
        3000..=3099 => vec![Yellow, Yellow, Gold, Gold, Red, Red, DarkRed],
        3100..=3199 => vec![Blue, Blue, DarkAqua, DarkAqua, Gold, Gold, Yellow],
        3200..=3299 => vec![Red, DarkRed, Gray, Gray, DarkRed, Red, Red],
        3300..=3399 => vec![Blue, Blue, Blue, LightPurple, Red, Red, DarkRed],
        3400..=3499 => vec![DarkGreen, Green, LightPurple, LightPurple, DarkPurple, DarkPurple, DarkGreen],
        3500..=3599 => vec![Red, Red, DarkRed, DarkRed, DarkGreen, Green, Green],
        3600..=3699 => vec![Green, Green, Green, Aqua, Blue, Blue, DarkBlue],
        3700..=3799 => vec![DarkRed, DarkRed, Red, Red, Aqua, DarkAqua, DarkAqua],
        3800..=3899 => vec![DarkBlue, DarkBlue, Blue, DarkPurple, DarkPurple, LightPurple, DarkBlue],
        3900..=3999 => vec![Red, Red, Green, Green, DarkAqua, Blue, Blue],
        4000..=4099 => vec![DarkPurple, DarkPurple, Red, Red, Gold, Gold, Yellow],
        4100..=4199 => vec![Yellow, Yellow, Gold, Red, LightPurple, LightPurple, DarkPurple],
        4200..=4299 => vec![DarkBlue, Blue, DarkAqua, Aqua, White, Gray, Gray],
        4300..=4399 => vec![Black, DarkPurple, DarkGray, DarkGray, DarkPurple, DarkPurple, Black],
        4400..=4499 => vec![DarkGreen, DarkGreen, Green, Yellow, Gold, DarkPurple, LightPurple],
        4500..=4599 => vec![White, White, Aqua, Aqua, DarkAqua, DarkAqua, DarkAqua],
        4600..=4699 => vec![DarkAqua, Aqua, Yellow, Yellow, Gold, LightPurple, DarkPurple],
        4700..=4799 => vec![White, DarkRed, Red, Red, Blue, DarkBlue, Blue],
        4800..=4899 => vec![DarkPurple, DarkPurple, Red, Gold, Yellow, Aqua, DarkAqua],
        4900..=4999 => vec![DarkGreen, Green, White, White, Green, Green, DarkGreen],
        _ => vec![DarkRed, DarkRed, DarkPurple, Blue, Blue, DarkBlue, Black],
    }
}


pub fn prestige_star(level: u32) -> &'static str {
    match level {
        0..=1099 => "✫",
        1100..=2099 => "✪",
        2100..=3099 => "⚝",
        _ => "✥",
    }
}


pub fn build_prestige_text(text: &str, colors: &[NamedColor]) -> MCText {
    text.chars().enumerate().fold(MCText::default(), |acc, (i, ch)| {
        let color = colors.get(i).copied().unwrap_or(NamedColor::White);
        acc + MCText::new().span(&ch.to_string()).color(color).build()
    })
}
