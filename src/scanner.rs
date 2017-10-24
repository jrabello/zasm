#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;
use pest::Parser;

#[derive(Debug)]
pub enum Token {
    None,
    InstructionId,
    Reg,
    Imm,
    Mem,
}

#[derive(Debug)]
pub struct Scanner {
    //example: (R, "a")
    tokens: Vec<(Token, String)>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { tokens: vec![] } //(Token::None, " ")
    }

    pub fn start(&mut self, source_code: &str) -> &Vec<(Token, String)> {
        //parsing source code
        let tokens = IdentParser::parse_str(Rule::tokens, source_code)
            .unwrap_or_else(|e| panic!("{}", e));

        //creating token_list
        for token in tokens {
            match token.as_rule() {                    
                Rule::instruction_id => {
                    self.tokens.push((
                        Token::InstructionId,
                        String::from(token.as_str()),
                    ));
                }                    
                Rule::reg => {
                    self.tokens.push((Token::Reg, String::from(token.as_str())));
                }
                Rule::imm => {
                    self.tokens.push((Token::Imm, String::from(token.as_str())));
                }
                Rule::mem => {
                    self.tokens.push((Token::Mem, String::from(token.as_str())));
                }
                _ => {
                    panic!("\tunknown token: {:?}", token.as_str());
                }
            }
            // for instruction_data in token.into_inner() {
            // }
        }
        &self.tokens
    }
}

// A pair is a combination of the rule which matched and a span of input
//println!("Rule:    {:?}", instruction.as_rule());
//println!("Span:    {:?}", instruction.clone().into_span());
//println!("Text:    {}", instruction.clone().into_span().as_str());
//"mov" ~ (reg | mem) ~ "," ~ (reg | imm | mem)
//Rule::alpha => println!("Letter:  {}", inner_pair.into_span().as_str()),
//Rule::digit => println!("Digit:   {}", inner_pair.into_span().as_str()),
//     Rule::mov => {
//         println!("mov!");
//         bytes.push(0x13);
//         //getting the type of move
//         let mut pairs = instruction_data.into_inner().peekable();
//         let op1 = pairs.next().unwrap();
//         let op2 = pairs.next().unwrap();
//         match (op1.as_rule(), op2.as_rule()) {
//             (Rule::reg, Rule::reg) => {
//                 bytes.push(0x3);
//                 println!("\tr, r");
//                 bytes.push(match op1.into_span().as_str() {
//                     "a" => 0,
//                     "b" => 1,
//                     "c" => 2,
//                     "d" => 3,
//                     "e" => 4,
//                     "f" => 5,
//                     _ => panic!("failed to know the register"),
//                 });
//                 bytes.push(match op2.into_span().as_str() {
//                     "a" => 0,
//                     "b" => 1,
//                     "c" => 2,
//                     "d" => 3,
//                     "e" => 4,
//                     "f" => 5,
//                     _ => panic!("failed to know the register"),
//                 });
//                 //println!("op1 {:?}", op1.into_span().start_pos());
//                 //println!("op2 {:?}", op2.into_span().as_str());
//             }
//             (Rule::reg, Rule::imm) => {
//                 bytes.push(0x4);
//                 println!("\tr, imm");
//                 bytes.push(match op1.into_span().as_str() {
//                     "a" => 0,
//                     "b" => 1,
//                     "c" => 2,
//                     "d" => 3,
//                     "e" => 4,
//                     "f" => 5,
//                     _ => panic!("failed to know the register"),
//                 });
//                 let mut pairs = op2.into_inner().peekable();
//                 let hex_num = pairs.next().unwrap();
//                 let number = u8::from_str_radix(hex_num.as_str(), 16).unwrap();
//                 bytes.push(number);
//                 //let number_span = op2.into_span();
//                 //let number_len = number_span.as_str().len();
//                 //let number = u8::from_str_radix(&number_span.as_str()[1..number_len-1], 16).unwrap();
//             }
//             (Rule::reg, Rule::mem) => {
//                 bytes.push(0x5);
//                 println!("\tr, mem");
//             }
//             (Rule::mem, Rule::reg) => {
//                 bytes.push(0x6);
//                 println!("\tmem, r");
//             }
//             _ => (),
//         }
//         // for op in instruction_data.into_inner() {
//         //     println!("\top: {:?}", op);
//         //     // match op.as_rule() {
//         //     //     Rule::reg => {
//         //     //         println!("\t\treg: {:?}", op);
//         //     //     }
//         //     //     _ => ()
//         //     // }
//         // }
//     }
//     Rule::stp => {
//         bytes.push(0x00);
//         println!("stp:   {}", instruction_data.into_span().as_str());
//     }
//     //Rule::reg => println!("reg:   {}", inner_pair.into_span().as_str()),
//     //Rule::imm => println!("imm:   {}", inner_pair.into_span().as_str()),
//     //Rule::mem => println!("mem:   {}", inner_pair.into_span().as_str()),
//     _ => panic!("{}", instruction_data),
// };