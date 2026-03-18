use serde::Serialize;

const COLOR_RED: u32 = 0xFF0000;
const COLOR_ORANGE: u32 = 0xFFA500;
const COLOR_PURPLE: u32 = 0xAF00AF;
const COLOR_GRAY: u32 = 0xC0C0C0;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TagDefinition {
    pub name: &'static str,
    pub display_name: &'static str,
    pub icon: &'static str,
    pub emote: &'static str,
    pub color: u32,
    pub score: f64,
    pub priority: u8,
}

pub static SNIPER: TagDefinition = TagDefinition {
    name: "sniper",
    display_name: "Sniper",
    icon: "mdi-alert-octagram",
    emote: "<:sniper:1459106167270932618>",
    color: COLOR_RED,
    score: 10.0,
    priority: 1,
};

pub static BLATANT_CHEATER: TagDefinition = TagDefinition {
    name: "blatant_cheater",
    display_name: "Blatant Cheater",
    icon: "mdi-alert-octagram",
    emote: "<:blatantcheater:1459106183196577812>",
    color: COLOR_ORANGE,
    score: 5.0,
    priority: 1,
};

pub static CLOSET_CHEATER: TagDefinition = TagDefinition {
    name: "closet_cheater",
    display_name: "Closet Cheater",
    icon: "mdi-alert-octagram-outline",
    emote: "<:closetcheater:1459106337039323136>",
    color: COLOR_ORANGE,
    score: 1.5,
    priority: 1,
};

pub static REPLAYS_NEEDED: TagDefinition = TagDefinition {
    name: "replays_needed",
    display_name: "Replays Needed",
    icon: "mdi-archive-alert-outline",
    emote: "<:replaysneeded:1482502914835615745>",
    color: COLOR_GRAY,
    score: 0.0,
    priority: 2,
};

pub static CAUTION: TagDefinition = TagDefinition {
    name: "caution",
    display_name: "Caution",
    icon: "mdi-alert-outline",
    emote: "<:caution:1459106358098923583>",
    color: COLOR_GRAY,
    score: 0.0,
    priority: 3,
};

pub static CONFIRMED_CHEATER: TagDefinition = TagDefinition {
    name: "confirmed_cheater",
    display_name: "Confirmed Cheater",
    icon: "mdi-alert-octagram-outline",
    emote: "<:confirmedcheater:1459106129765204049>",
    color: COLOR_PURPLE,
    score: 5.0,
    priority: 1,
};

static ALL_TAGS: &[&TagDefinition] = &[
    &SNIPER,
    &BLATANT_CHEATER,
    &CLOSET_CHEATER,
    &REPLAYS_NEEDED,
    &CAUTION,
    &CONFIRMED_CHEATER,
];

pub const EMOTE_TAG: &str = "<:tag:1459106270207545417>";
pub const EMOTE_ADDTAG: &str = "<:addtag:1459106318387249289>";
pub const EMOTE_EDITTAG: &str = "<:edittag:1459106301929062430>";
pub const EMOTE_REMOVETAG: &str = "<:removetag:1459161936355786752>";

pub fn lookup(name: &str) -> Option<&'static TagDefinition> {
    ALL_TAGS.iter().find(|t| t.name == name).copied()
}

pub fn all() -> &'static [&'static TagDefinition] {
    ALL_TAGS
}

#[derive(Debug, Clone)]
pub struct Replay {
    pub id: String,
    pub timestamp: Option<String>,
}

impl Replay {
    pub fn format_command(&self) -> String {
        match &self.timestamp {
            Some(ts) => format!("/replay {} {}", self.id, ts),
            None => format!("/replay {}", self.id),
        }
    }
}

pub fn parse_replay(input: &str) -> Option<Replay> {
    let input = input.trim();

    let id = input
        .split_whitespace()
        .find(|word| is_dashed_uuid(word))?
        .to_string();

    let timestamp = input
        .split_whitespace()
        .find(|word| word.starts_with('#') && word.len() > 1)
        .map(|s| s.to_string());

    Some(Replay { id, timestamp })
}

fn is_dashed_uuid(s: &str) -> bool {
    if s.len() != 36 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let expected_lens = [8, 4, 4, 4, 12];
    parts
        .iter()
        .zip(expected_lens)
        .all(|(part, len)| part.len() == len && part.chars().all(|c| c.is_ascii_hexdigit()))
}
