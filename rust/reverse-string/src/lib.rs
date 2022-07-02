pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

// For the grapheme test. Commented out as external crates are not working with exercism's test runner
// use unicode_segmentation::UnicodeSegmentation;

// pub fn reverse(input: &str) -> String {
//     input.graphemes(true).rev().collect()
// }
