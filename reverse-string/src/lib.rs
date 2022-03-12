use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut graphemes = Vec::new();
    for c in input.graphemes(true) {
        graphemes.push(c);
    }

    graphemes.reverse();
    let mut result = String::new();
    for (_index, &c) in graphemes.iter().enumerate() {
        result.push_str(c);
    }

    result
}
