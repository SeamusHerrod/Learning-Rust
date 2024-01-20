use std::io;

fn main() {
    let mut s = String::new();
    println!("Input a string");
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read line");

    let word_idx = first_word(&s);
    let word1 = &s[0..word_idx];
    println!("First word in '{}' is {}", s, word1);

    let word1_slice = first_word_slice(&s);
    println!("word1 returned as slice: {}", word1_slice);

    let word2_slice = second_word(&s);
    println!("word2 returned as slice: {}", word2_slice);
}

//Problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array of bytes

    // iter() creates an iterator for array of bytes, enumerate() wraps iter and returns each
    // element as part of a tuple
        // (index, element reference)
    for (i, &item) in bytes.iter().enumerate() {
        // b'' - byte literal syntax
        if item == b' ' {
            // if item matches the byte rep of 'space' then return index
            return i;
        }
    }
    s.len() // if there is no space in the string, then return the last index of array
            // entire string represents a single word

}

// rewriting first_word to return the string slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // slice of entire string, equivalent to &s[0..len]
}

fn second_word(s: &String) -> &str {
    let mut space_count = 0;
    let bytes = s.as_bytes();
    let mut second_start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && space_count == 0{
            space_count +=1;
            second_start = i+1;
        }
        else if item == b' ' && space_count == 1 {
            space_count += 1;
            return &s[second_start..i];
        }
    }
    if (space_count == 0){
        println!("only one word in string, returning whole string");
    }
    &s[second_start..]
}
