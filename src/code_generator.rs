use scanner;

pub struct CodeGenerator{
    bytes: Vec<u8>
}

impl CodeGenerator{
    pub fn new() -> CodeGenerator {
        CodeGenerator { bytes: vec![] }
    }
    
    pub fn start(&mut self, tokens: &Vec<(scanner::Token, String)>) -> &Vec<u8> {
        // zion-vm cannot handle more than 255 bytes
        self.bytes = Vec::with_capacity(255);
        
        // iterate over tokens, and generate code
        for &(ref id, ref value) in tokens.iter(){
            println!("{:?} {}",id, value);
            match *id{
                scanner::Token::InstructionId => {
                    match *value.as_str() {
                         "mov" => {

                         },
                        _ => {},
                    }
                }
                _ => {

                },
            }
        }

        &self.bytes
    }
}