use std::collections::{HashMap};

fn add_word(words: &mut HashMap<String, String>, word_english: String, word_spanish: String) {
    assert!(words.contains_key(&word_english.to_string()), "Word already exist");
    words.insert(word_english.to_lowercase(), word_spanish.to_lowercase());
}

fn print_words(words: &HashMap<String, String>) {
    for words in words.iter() {
        println!("key: {} -- value: {}", words.0, words.1);
    }
}

fn translate_word(words: &HashMap<String, String>, word: String) {
    let text = word.split(" ");
    let mut result_to_translate = String::from("");
    for (_, str)  in text.enumerate() {
        if words.contains_key(str) {
            result_to_translate += &words.get(str).unwrap().to_string();
        }
        result_to_translate += " ";
    }
    println!("{}", result_to_translate);
}

fn main() {
    let mut words: HashMap<String, String> = HashMap::new();
    add_word(&mut words, "I".to_string(), "Yo".to_string());
    add_word(&mut words, "Play".to_string(), "Juego".to_string());
    add_word(&mut words, "Play".to_string(), "Juego".to_string());
    add_word(&mut words, "With".to_string(), "con".to_string());
    add_word(&mut words, "my".to_string(), "mi".to_string());
    add_word(&mut words, "phone".to_string(), "telefono".to_string());
    print_words(&words);
    translate_word(&words, "I Play with my phone".to_lowercase().to_string());
}
