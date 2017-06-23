use std::io;
use std::collections::HashMap;

pub mod huffman;
use huffman::Node;

/// Huffman algorithm as defined here:
/// https://www.siggraph.org/education/materials/HyperGraph/video/mpeg/mpegfaq/huffman_tutorial.html

fn read_input() -> String {
    /// Read user input and remove white chars.
    println!("Type your text to encode.");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => println!("Your text: {}", buffer),
        Err(error) => println!("Error! {}", error)
    }

    buffer.trim().to_string()
}

fn encode_char(letter: &str, dictionary: &HashMap<String, Vec<i32>>) -> String {
    /// Replace a letter with the corresponding list of numbers.
    let codes = dictionary.get(letter).unwrap_or_else(||panic!(format!("Could not encode char `{}`", letter)));
    codes.iter().map(|a| a.to_string()).collect::<String>()
}

fn main() {
    let to_encode_text = read_input();
    let chars = to_encode_text.split("").collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();
    let root = Node::from_input(&to_encode_text, &chars);
    let codes = root.generate_codes();
    let dictionary = root.build_dictionary(&codes);

    let result = to_encode_text
        .split("")
        .filter(|s|s != &"")
        .map(|s| encode_char(s, &dictionary))
        .collect::<String>();
//        .collect::<Vec<_>>();

    println!("Result: {:?}", result);
}
