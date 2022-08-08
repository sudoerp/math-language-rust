#![allow(dead_code, unused)]
mod token;
mod parser;
use std::io;
use crate::token::{tokenize, Token, TokenType};
use crate::parser::parse;

fn main(){
   loop{
     let mut code: String = String::new();
     println!(">");
     io::stdin().read_line(&mut code).expect("Unable To Read Source Code");
     if code == " "{
       continue;
     }
     else if code.trim() == "q"{
       break;
     }
     else{
       let tokens = tokenize(&code.trim().to_string()).unwrap();
       for ast in parse(tokens){
         println!("{:?}", ast)
       }
     }
   }
   
}
