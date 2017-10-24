extern crate pest;
extern crate num_traits;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate enum_primitive_derive;


mod scanner;
mod code_generator;

fn main() {
    // generating tokens
    let mut sc = scanner::Scanner::new();
    let tokens = sc.start(
        "mov a, b            
         mov d, @11h
         mov a, #22h
         mov @33h, a
         stp",
    );
    
    // generating code    
    let mut cg = code_generator::CodeGenerator::new();
    let bytes = cg.start(tokens);
    println!("{}", bytes.len());

    // write compiled program to a file 
    use std::io::prelude::*;
    use std::fs::File;
    let mut buffer = File::create("program.bin").unwrap();
    buffer.write(&bytes).unwrap();
}