use crate::*;
use regex::Regex;

#[derive(PartialEq)]
enum AddType {
    FromFile,
    Written,
    Unknown,
}

pub fn mode_add_text(
    re: &Regex,
    re_chunk: &Regex,
    text_indexmap: &mut IndexMap<usize, Text>,
    word_hashmap: &mut HashMap<String, Word>,
) {
    let mut written_input_title = String::new();
    let mut written_input = String::new();
    print_adding_text();

    //Add println!
    let mut text_add_type = AddType::Unknown;
    written_input = get_input();
    let written_output: i32 = written_input.trim().parse().expect("Please type a number");

    text_add_type = match written_output {
        1 => AddType::FromFile,
        2 => AddType::Written,
        _ => AddType::Unknown,
    };
    if text_add_type == AddType::Unknown {
        return;
    };
    //Probably just find way to read all files into
    //memory from selected folder
    //for each file in x file path
    //-bind to values, regex
    //Additional: Check if files already are included
    if text_add_type == AddType::FromFile {}

    if text_add_type == AddType::Written {
        print_writing_text();
        written_input_title = get_input();
        let title_text = &written_input_title;
        written_input = get_input();
        let text = &written_input;
        regex_word_finder(&re, word_hashmap, text);
        regex_chunk_finder(&re_chunk, text_indexmap, text, title_text);
    }
}

fn regex_chunk_finder(
    re_chunk: &Regex,
    text_indexmap: &mut IndexMap<usize, Text>,
    text: &str,
    title_text: &str,
) {
    let chunks_vec: Vec<Chunk> = re_chunk
        .split(&text)
        .map(|s| Chunk {
            body: s.trim().to_string(),
        })
        .collect();

    let text_struct = Text {
        title: title_text.to_string(),
        chunks: chunks_vec,
    };

    let next_key = text_indexmap.len();
    text_indexmap.insert(next_key, text_struct);

    println!("All chunks from all texts:");
    for (key, text) in text_indexmap.iter() {
        println!("\n[Text {}] Title: {}", key, text.title);
        for (i, chunk) in text.chunks.iter().enumerate() {
            println!("  Chunk {}: {}", i, chunk.body);
        }
    }
}

fn regex_word_finder(re: &Regex, word_hashmap: &mut HashMap<String, Word>, text: &str) {
    for cap in re.captures_iter(text) {
        let captured_word = &cap[1].to_lowercase();
        word_hashmap
            .entry(captured_word.to_string())
            .or_insert(Word {
                //fill in extra fields
                word: captured_word.to_string(),
            });
    }
}
