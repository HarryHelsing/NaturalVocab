use regex::Regex;
use crate::*;

pub fn mode_add_text(re: &Regex,
        word_hashmap:&mut HashMap<String, Word>) {

    let mut written_input = String::new();
    print_adding_text();
        written_input = get_input();

    //put input for text
    let text = &written_input;
        regex_word_finder(&re, word_hashmap, text);
}
