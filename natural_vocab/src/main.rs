use serde::{Serialize, Deserialize};
use ron::ser::*;
use ron::de::*;
use std::fs;
use std::io;
use std::collections::HashMap;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
    enum FamiliarityLevel  {
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
    print_mode_select();
        written_input = get_input();

        //Loading in word list from file
    let mut word_hashmap = HashMap::<String, Word>::new(); 
        word_hashmap = deserialise_word_hash();

    //Temporary test value for the logic
    let text = "Let's imagine this is some text in italian... Café? Does it do multiple lines correctly?";
    //put this in it's own function to call when updating text?
    let re = Regex::new(r"(\p{L}+(?:'\p{L})*)").unwrap();
        word_hashmap = regex_word_finder(&re, word_hashmap, text);

        //Temporary checker for hashmap
/*    for key in word_hashmap.keys() {
        println!("Key: {}", key);*/
    }
mode_select();
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
    mut word_hashmap: HashMap<String, Word>,
    text: &str,
) -> HashMap<String, Word> {
    for cap in re.captures_iter(text) {
        let captured_word = &cap[1].to_lowercase();
        word_hashmap.entry(captured_word.to_string())
            .or_insert(Word {//fill in extra fields
                word: captured_word.to_string(),
            });
    }
    word_hashmap

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


// Print functions
fn print_welcome_1() {
    println!("

=================================
    Welcome to NaturalVocab
=================================

This app is designed to help you
learn vocabulary in any language
in a natural and interesting way.

You upload content which you find
interesting and engaging, so that
you learn the words in context,
in a way that you personally
connect with.

- Continue (press ENTER to continue)");

}

fn print_welcome_2() {
    println!("
More than this, though the app
primarily tracks the amount of
words you've become comfortable
with, it is deisgned in such a
way that over time the grammar
of the language is naturally
acquired.

The app is made to be structured,
while being flexible. It
tracks your progress in a gentle,
self defined way.

- Continue (press ENTER to continue)");
}

fn print_lang_select() {
        println!("

=================================
     Language Selection
=================================

... Later add proper alternative language support
");
}

fn print_mode_select() {
        println!("

=================================
        Mode Selection
=================================
");
}

//Do I need to feed in arguments for written_input, regex (re), word_hash?
//Then probably return a bunch of those values
fn mode_select () {
    println!(
"Select a Mode
Type 1 then press enter to begin
Text Upload and Review Mode

Type 2 then press enter to begin
Word Review and Study Mode

Type 3 then press enter to begin
Overview Progress Mode

Type 4 then press enter to Exit\n\n");

    let mut written_input = String::new();
        written_input = get_input();
        let written_input: i32 = written_input.trim().parse().expect("Please type a number");
        
        match written_input  {
             1 => mode_add_text(),
             2 => println!("Two was typed"),
             3 => println!("Three was typed"),
             4 => println!("Four was typed"),
             _ => println!("Something was typed"),
        }
//match statement with function calls
}

fn mode_add_text () {

}

fn mode_review_words () {

}

fn mode_overview () {

}
/* to-do
 * - Create input logic to take in new text
 * - Reformat into hashmap
 * - Create hashmap for 'words'
 * - Slice the body of texts into words and update words hashmap
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

println!("Current texts:\n{:#?}", updated_texts);

let ron_string = to_string(&updated_texts)?;
    fs::write("languages/italian/texts.ron", ron_string)?;
    //--- End of junk code ---^
 */ 
