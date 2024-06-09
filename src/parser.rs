use crate::lexer::Lexer;
use crate::translator::Arithmetic::*;
use crate::translator::Command;
use crate::translator::Command::*;
use crate::translator::MemorySegment::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
        }
    }

    pub fn next_command(&mut self) -> Option<Command> {
        let f = self.next_token().unwrap_or(&[]);
        if f.len() == 0 {
            return None;
        }

        let command = f.iter().collect::<String>();

        match command.as_str() {
            "add" => Some(Arithmetic(ADD)),
            "sub" => Some(Arithmetic(SUB)),
            "neg" => Some(Arithmetic(NEG)),
            "eq" => Some(Arithmetic(EQ)),
            "gt" => Some(Arithmetic(GT)),
            "lt" => Some(Arithmetic(LT)),
            "and" => Some(Arithmetic(AND)),
            "or" => Some(Arithmetic(OR)),
            "not" => Some(Arithmetic(NOT)),
            "push" | "pop" => {
                let memory_segment = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                let memory_segment = match memory_segment.as_str() {
                    "local" => Local,
                    "argument" => Argument,
                    "this" => This,
                    "that" => That,
                    "constant" => Constant,
                    "static" => Static,
                    "pointer" => Pointer,
                    "temp" => Temp,
                    _ => panic!("invalid memory segment: {}", memory_segment),
                };
                let index: u16 = self
                    .next_token()
                    .expect("Syntax error: segment index expected")
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if command == "push" {
                    Some(Push(memory_segment, index))
                } else {
                    Some(Pop(memory_segment, index))
                }
            }
            "label" | "goto" => {
                let label = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                if command == "label" {
                    Some(Label(label))
                } else {
                    Some(Goto(label))
                }
            }
            "if" => {
                self.next_token().expect("syntax error near if");
                assert_eq!(
                    self.next_token()
                        .expect("if-goto expected")
                        .iter()
                        .collect::<String>(),
                    "goto".to_string()
                );
                let label = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                Some(If(label))
            }
            "function" | "call" => {
                let name = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                let arg: u16 = self
                    .next_token()
                    .expect("Syntax error: segment index expected")
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if command == "function" {
                    Some(Function(name, arg))
                } else {
                    Some(Call(name, arg))
                }
            }
            "return" => Some(Return),
            _ => panic!("invalid command {}", command),
        }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        return self.lexer.next_token();
    }
}
