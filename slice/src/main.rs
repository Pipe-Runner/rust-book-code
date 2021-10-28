fn main() {
    let mut s1 = String::from("How are you doing?");
    let s2 = String::from("Beautiful");

    /**
     * A non-slice approach
     */
    // println!("First word for first sentence at -> {}", first_word(&s1));
    // println!("First word for first sentence at -> {}", first_word(&s2));

    // let index_first_word = first_word(&s1);
    // s1.clear();

    // ^ s1 is now "" but index_first_word is 3 hence have gone out of sync

    /**
     * This won't compile since we have a mixed mutable and immutable reference in the scope
     */
    let first_word_slice = first_word_slice(&s1);

    s1.clear();

    println!("{}", first_word_slice);
}

fn first_word(sentence: &String) -> usize {
    let byte_slice = sentence.as_bytes();
    let slice_iterator = byte_slice.iter();

    for (idx, &item) in slice_iterator.enumerate() {
        if item == b' ' {
            return idx;
        }
    }

    sentence.len()
}

fn first_word_slice(sentence: &String) -> &str {
    let byte_slice = sentence.as_bytes();
    let slice_iterator = byte_slice.iter();

    for (idx, &item) in slice_iterator.enumerate() {
        if item == b' ' {
            return &sentence[..idx];
        }
    }

    &sentence[..]
}
