use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input
        .graphemes(true)
        .rev()
        .flat_map(|x| x.chars())
        .collect::<String>()
}