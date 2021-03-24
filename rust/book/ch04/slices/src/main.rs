fn main() {
    let words = String::from("foo bar");
    println!("first word: {}", first_word(&words));
    println!("second word: {}", second_word(&words));
    println!("third word: {}", nth_word(2, &words));
}

fn first_word(s: &str) -> &str {
    nth_word(0, s)
}

fn second_word(s: &str) -> &str {
    nth_word(1, s)
}

fn nth_word(n: u32, s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut word_count = 0;
    let mut word_start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if word_count == n {
                return &s[word_start..i];
            } else {
                word_count += 1;
                word_start = i + 1;
            }
        }
    }

    &s[..]
}
