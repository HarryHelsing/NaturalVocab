mod mode_add_text;
mod println;
use crate::mode_add_text::mode_add_text;
mod serialise;
use crate::serialise::*;
//create modules for different app modes
//Also! for de/serialisation

use crate::println::*;
use indexmap::IndexMap;
use regex::Regex;
use ron::de::*;
use ron::ser::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(PartialEq)]
enum ModeType {
    Add,
    Review,
    Overview,
    Exit,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
enum FamiliarityLevel {
    NotReviewed,
    Unfamiliar,
    Somewhat,
    Familiar,
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    // <- of this  <¬
    word: String, //data duplication?
                  //comments: String,
                  //familiarity: FamiliarityLevel,
                  //text_links: Vec<i32>,
                  //word_links: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Text {
    title: String,
    chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chunk {
    body: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //APP INTRODUCTION
    let mut written_input = String::new();
    print_welcome_1();
    written_input = get_input();
    print_welcome_2();
    written_input = get_input();
    print_lang_select();
    written_input = get_input();

    //Loading in word list from file
    let mut word_hashmap = HashMap::<String, Word>::new();
    let mut text_indexmap: IndexMap<usize, Text> = IndexMap::new();
    word_hashmap = deserialise_word_hash();
    text_indexmap = deserialise_text_index();

    //put this in it's own function to call when updating text?
    let re = Regex::new(r"(\p{L}+(?:'\p{L})*)").unwrap();
    let re_chunk = Regex::new(r"#\!").unwrap();

    //SELECT A MODE
    let mut mode_type = ModeType::Unknown;
    loop {
        print_mode_select();
        written_input = get_input();
        let written_output: i32 = written_input.trim().parse().expect("Please type a number");
        //program crashes if number is not typed: add error handling

        mode_type = match written_output {
            1 => ModeType::Add,
            2 => ModeType::Review,
            3 => ModeType::Overview,
            4 => ModeType::Exit,
            _ => ModeType::Unknown,
        };
        if mode_type == ModeType::Exit {
            break;
        };
        if mode_type == ModeType::Add {
            mode_add_text(&re, &re_chunk, &mut text_indexmap, &mut word_hashmap)
        };
        if mode_type == ModeType::Review {
            mode_review_words(&mut word_hashmap)
        };
        if mode_type == ModeType::Overview {
            mode_overview()
        };
        print_continue();
        written_input = get_input();
    }
    serialise_word_hash(&word_hashmap);
    serialise_text_index(&text_indexmap);
    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn mode_review_words(word_hashmap: &mut HashMap<String, Word>) {
    //Temporary checker for hashmap
    for key in word_hashmap.keys() {
        println!("Key: {}", key);
    }
}

fn mode_overview() {}
/* to-do
 * - Create fields for linking to other hashmaps
 * - Create logic to link words and texts
 * - Create logic to navigate words and text
 * - Create logic to show words with their associate texts
 * - Add comments feature
 * save until future:
 * - Complex searching
 * - Adding tags
 * - Adding flags
 * - Adding date uploaded and logic for adding date
 */
