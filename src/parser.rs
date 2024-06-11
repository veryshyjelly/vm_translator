use crate::command::Arithmetic::*;
use crate::command::Command;
use crate::command::Command::*;
use crate::command::MemorySegment::*;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
        }
    }

    pub fn next_command(&mut self) -> Result<Option<Command>, String> {
        let f = self.next_token().unwrap_or(&[]);
        if f.len() == 0 {
            return Ok(None);
        }

        let command = f.iter().collect::<String>();

        let command = match command.as_str() {
            "add" => Arithmetic(ADD),
            "sub" => Arithmetic(SUB),
            "neg" => Arithmetic(NEG),
            "eq" => Arithmetic(EQ),
            "gt" => Arithmetic(GT),
            "lt" => Arithmetic(LT),
            "and" => Arithmetic(AND),
            "or" => Arithmetic(OR),
            "not" => Arithmetic(NOT),
            "push" | "pop" => {
                let memory_segment = self
                    .next_token()
                    .ok_or("Syntax error: memory segment expected")?
                    .iter()
                    .collect::<String>();
                let segment = match memory_segment.as_str() {
                    "local" => Local,
                    "argument" => Argument,
                    "this" => This,
                    "that" => That,
                    "constant" => Constant,
                    "static" => Static,
                    "pointer" => Pointer,
                    "temp" => Temp,
                    _ => Err(format!("invalid memory segment: {}", memory_segment))?,
                };
                let index: u16 = self
                    .next_token()
                    .ok_or("Syntax error: segment index expected")?
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if command == "push" {
                    Push(segment, index)
                } else {
                    Pop(segment, index)
                }
            }
            "label" | "goto" => {
                let label = self
                    .next_token()
                    .ok_or("Syntax error: memory segment expected")?
                    .iter()
                    .collect::<String>();
                if command == "label" {
                    Label(label)
                } else {
                    Goto(label)
                }
            }
            "if" => {
                self.next_token().expect("syntax error near if");
                assert_eq!(
                    self.next_token()
                        .ok_or("if-goto expected")?
                        .iter()
                        .collect::<String>(),
                    "goto".to_string()
                );
                let label = self
                    .next_token()
                    .ok_or("Syntax error: memory segment expected")?
                    .iter()
                    .collect::<String>();
                IfGoto(label)
            }
            "function" | "call" => {
                let name = self
                    .next_token()
                    .ok_or("Syntax error: memory segment expected")?
                    .iter()
                    .collect::<String>();
                let count: u16 = self
                    .next_token()
                    .ok_or("Syntax error: segment index expected")?
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if command == "function" {
                    Function(name, count)
                } else {
                    Call(name, count)
                }
            }
            "return" => Return,
            _ => Err(format!("invalid command {}", command))?,
        };

        Ok(Some(command))
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        return self.lexer.next_token();
    }
}
