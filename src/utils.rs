

use regex::Regex;

pub fn get_time_to_read(text: &str, words_per_minute: usize) -> usize {
    let trimmed_text = text.trim();
    let re = Regex::new(r"\s+").unwrap();
    let words: usize = re.split(trimmed_text).count();
    let min_to_read = words as f64 / words_per_minute as f64;
    let sec_to_read = (min_to_read * 60.0).ceil() as usize;
    sec_to_read
}

#[test]
fn test_get_time_to_read() {
    let text = "This is a sample text to calculate the time to read.";
    let words_per_minute = 200; // average reading speed
    let time_to_read = get_time_to_read(text, words_per_minute);
    println!("Time to read: {} seconds", time_to_read);
}
