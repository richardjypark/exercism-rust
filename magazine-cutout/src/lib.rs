// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words_map = HashMap::new();
    for &word in magazine {
        *words_map.entry(word).or_insert(0) += 1;
    }

    let mut note_map = HashMap::new();
    for &word in note {
        *note_map.entry(word).or_insert(0) += 1;
    }
    note_map
        .iter()
        .enumerate()
        .all(|(_, (&key, value))| words_map.get(key) >= Some(value))
}
