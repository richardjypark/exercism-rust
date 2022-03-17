use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_lowercased = word.to_lowercase();
    let mut input = word_lowercased.graphemes(true).collect::<Vec<&str>>();
    input.sort_unstable();

    for (_, &w) in possible_anagrams.iter().enumerate() {
        let w_lowercased = w.to_lowercase();
        let mut candidate = w_lowercased.graphemes(true).collect::<Vec<&str>>();
        if candidate.len() == input.len() {
            candidate.sort_unstable();
            let mut is_anagram = true;
            for (index, &c) in candidate.iter().enumerate() {
                if c != *input.get(index).unwrap() {
                    is_anagram = false;
                    break;
                }
            }
            if is_anagram && w_lowercased != word_lowercased {
                anagrams.insert(w);
            }
        }
    }

    anagrams
}
