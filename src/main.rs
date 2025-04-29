use crate::utils::read_lines;

use crate::parser::Wrapper;
use crate::parser::MarkdownType;
use std::io;
use std::env;

#[path = "utils/file_utils.rs"] mod utils;

#[path = "parser/parser.rs"] mod parser;
fn main() {
    let mut input: String = String::new(); 

    let args: Vec<String> = env::args().collect();

    let filePath = &args[1];

    if let Ok(lines) = read_lines(filePath) {
        for line in lines.map_while(Result::ok) {
            //println!("{}", line);
        let input=line.trim().replace("\n","");
        if input.starts_with("##"){
        
            let userInput=Wrapper{input:line[2..].to_string(),md_type:MarkdownType::H2TYPE};
            println!("{}",{userInput.wrap()});
            //println!("h2 parsed");

        }else if input.starts_with("#") {
            let userInput=Wrapper{input:line[1..].to_string(),md_type:MarkdownType::H1TYPE};
            println!("{}",{userInput.wrap()});
            //println!("h2 parsed");
        } else if !input.trim().is_empty(){
            let userInput=Wrapper{input:line[0..].to_string(),md_type:MarkdownType::PARAGRAPH};
            println!("{}",{userInput.wrap()});
           
          
        };
            }
        }
       
    
    
}
