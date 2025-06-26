use crate::*;

pub fn serialise_word_hash(
    word_hash: &HashMap<String, Word>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ron_string = to_string(word_hash)?;
    fs::write("languages/italian/word_list.ron", ron_string)?;

    Ok(())
}

pub fn deserialise_word_hash() -> HashMap<String, Word> {
    let hashmap: HashMap<String, Word> = match fs::read_to_string("languages/italian/word_list.ron")
    {
        Ok(content) => match from_str(&content) {
            Ok(hashmap) => hashmap,
            Err(_) => HashMap::<String, Word>::new(),
        },
        Err(_) => HashMap::<String, Word>::new(),
    };
    hashmap
}
