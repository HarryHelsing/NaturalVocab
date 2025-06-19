mod println;

use serde::{Serialize, Deserialize};
use ron::ser::*;
use ron::de::*;
use std::fs;
use std::io;
use std::collections::HashMap;
use regex::Regex;
use crate::println::*;

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
        word: String,
        //comments: String,
        //familiarity: FamiliarityLevel,
        //text_links: Vec<i32>,
    }//Links to texts: Vec of hashs?, familiarity level: int?, comments: string


#[derive(Serialize, Deserialize, Debug)]
    struct Text {
        title: String,
        body: String,
        comments: String,
    }

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut written_input = String::new();
    print_welcome_1();
        written_input = get_input();
    print_welcome_2();
        written_input = get_input();
    print_lang_select();
        written_input = get_input();

        //Loading in word list from file
    let mut word_hashmap = HashMap::<String, Word>::new(); 
        word_hashmap = deserialise_word_hash();

    //Temporary test value for the logic
    let text = "It's Italian time!";
    //put this in it's own function to call when updating text?
    let re = Regex::new(r"(\p{L}+(?:'\p{L})*)").unwrap();
        regex_word_finder(&re, &mut word_hashmap, text);

        //Temporary checker for hashmap
    for key in word_hashmap.keys() {
        println!("Key: {}", key);
        }

    let mut mode_type = ModeType::Unknown;
        loop {    
    print_mode_select();
        written_input = get_input();
        let written_output: i32 = written_input.trim().parse().expect("Please type a number");
        //program crashes if number is not typed: add error handling
        
        //Need to:
        //Create if statements for each enum type
        mode_type = match written_output  {
             1 => ModeType::Add,
             2 => ModeType::Review,
             3 => ModeType::Overview,
             4 => ModeType::Exit,
             _ => ModeType::Unknown,
        };
    if mode_type == ModeType::Exit {break};
    if mode_type == ModeType::Add {mode_add_text(&re, &mut word_hashmap)};
    if mode_type == ModeType::Review {mode_review_words(&mut word_hashmap)};
    if mode_type == ModeType::Overview {mode_overview()};
        print_continue();
        written_input = get_input();
    }
    serialise_word_hash(&word_hashmap);//add string, word?
    Ok(())
    }

fn get_input() -> String {
let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn regex_word_finder(
    re: &Regex,
    word_hashmap: &mut HashMap<String, Word>,
    text: &str,

){
    for cap in re.captures_iter(text) {
        let captured_word = &cap[1].to_lowercase();
        word_hashmap.entry(captured_word.to_string())
            .or_insert(Word {//fill in extra fields
                word: captured_word.to_string(),
            });
    }

}

fn deserialise_word_hash()-> HashMap::<String, Word> {
    let hashmap: HashMap::<String, Word> = match
        fs::read_to_string("languages/italian/word_list.ron") {
            Ok(content) => {
                match from_str(&content)
                {
            Ok(hashmap) => hashmap,
            Err(_) => {   HashMap::<String, Word>::new()
                }
        }
},
    Err(_) => {   HashMap::<String, Word>::new()
    }
};
hashmap
}

fn serialise_word_hash(word_hash:&HashMap::<String, Word>) -> Result<(), Box<dyn std::error::Error>> {
let ron_string = to_string(word_hash)?;
    fs::write("languages/italian/word_list.ron", ron_string)?;

    Ok(())
}



fn mode_add_text (re: &Regex,
        word_hashmap:&mut HashMap<String, Word>) {
//Temporary test value for the logic
    let text = "Pringles?";
    //put this in it's own function to call when updating text?
    let re = Regex::new(r"(\p{L}+(?:'\p{L})*)").unwrap();
        regex_word_finder(&re, word_hashmap, text);
}

fn mode_review_words (
word_hashmap:&mut HashMap<String, Word>
    ) {

        //Temporary checker for hashmap
    for key in word_hashmap.keys() {
        println!("Key: {}", key);
        }

}

fn mode_overview () {

}
/* to-do
 * - Create input logic to take in new text
 * - Reformat into hashmap
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


/* code for later reference
 * use hashmaps (not hashsets) and methods .entry().or_insert(x);
 * .entry() to see if there is a preexisting key and .or_insert()
 * to place new values if there are no previous ones set
*/



/*
    //----- junk code----v
let new_text = Text {
    title: "Title of note".to_string(),
    body: "Here is the body of the note".to_string(),
    comments: "Comments here".to_string(),
    //In future put in tags, links, hashmap links, and flags fields
};

let ron_string = to_string(&updated_texts)?;
    fs::write("languages/italian/texts.ron", ron_string)?;
    //--- End of junk code ---^
 */ 
