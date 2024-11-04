use std::{env, fs, io};
use serde_json::Value;
fn main() {
    
    // read the Args from cmd: cox <path> <query>
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let query: &String = &args[2];

    // Open file and parse it to json
    println!("Args 1: {file_path}\nArgs 2: {query}");

    let context: Result<String, io::Error> = fs::read_to_string(file_path);
    match context {
        Ok(text) => {
            // each each line and parse the data
            // println!("{}", text)
            for line in text.lines(){
                let found_or_not = look_for_expression(line, query);
                match found_or_not {
                    Some(_) => println!("{}", line),
                    None => continue                
                }
            }

        },
        Err(error) => {
            println!("{}", error)
        }
    }
}

fn look_for_expression(line: &str, query: &String) -> Option<bool> {
    // json to hashmap; 
    // break query string to key value
    // look for hashmap[key] == value ? true : false
    let (key, to_find) = query.split_once("=").unwrap();

    let data: Result<Value, serde_json::Error> = serde_json::from_str(line);
    match data {
        Ok(value) => {
            if value[key] == to_find {
                return Some(true);
            }
        },
        Err(_) => {
            println!("Invalid line No {}", line)
        }
    }
    None

}