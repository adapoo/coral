use crate::rules;

/// Calculate the total score for a set of tag names.
pub fn calculate_score(tag_names: &[&str]) -> f64 {
    tag_names
        .iter()
        .filter_map(|name| rules::lookup(name))
        .map(|def| def.score)
        .sum()
}
