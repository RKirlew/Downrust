use crate::parser::Wrapper;
use crate::parser::MarkdownType;
use std::io;
use std::env;
#[path = "parser/parser.rs"] mod parser;
fn main() {
    let mut input: String = String::new(); 

    let args: Vec<String> = env::args().collect();

    let filePath = &args[1];
    io::stdin() 
        .read_line(&mut input) 
        .expect("Unable to read Stdin");
    let input=input.trim();
    if input.starts_with("##"){
       
        let userInput=Wrapper{input:input[2..].to_string(),md_type:MarkdownType::H2TYPE};
        println!("{}",{userInput.wrap()});
        println!("h2 parsed");

    }else if input.starts_with("#") {
        let userInput=Wrapper{input:input[1..].to_string(),md_type:MarkdownType::H1TYPE};
        println!("{}",{userInput.wrap()});
        println!("h2 parsed");
    } else if input.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) {
        let userInput=Wrapper{input:input[0..].to_string(),md_type:MarkdownType::PARAGRAPH};
        println!("{}",{userInput.wrap()});
        println!("p parsed");
    } else {
        println!("Unrecognized Markdown type");
        return;
    };
    
    
}
