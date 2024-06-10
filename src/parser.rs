use crate::lexer::Lexer;
use crate::translator::Arithmetic::*;
use crate::translator::Command;
use crate::translator::Command::*;
use crate::translator::MemorySegment::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    function_name: Option<String>,
    call_rank: u64,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
            function_name: None,
            call_rank: 0,
        }
    }

    pub fn next_command(&mut self) -> Option<Command> {
        let f = self.next_token().unwrap_or(&[]);
        if f.len() == 0 {
            return None;
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
                    .expect("Syntax error: memory segment expected")
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
                    Push(segment, index)
                } else {
                    Pop(segment, index)
                }
            }
            "label" | "goto" => {
                let label = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                let function_name = self
                    .function_name
                    .as_ref()
                    .expect("label should be inside a function");
                if command == "label" {
                    Label(format!("{function_name}${label}"))
                } else {
                    Goto(format!("{function_name}${label}"))
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
                let function_name = self
                    .function_name
                    .as_ref()
                    .expect("label should be inside a function");
                IfGoto(format!("{function_name}${label}"))
            }
            "function" | "call" => {
                let name = self
                    .next_token()
                    .expect("Syntax error: memory segment expected")
                    .iter()
                    .collect::<String>();
                let count: u16 = self
                    .next_token()
                    .expect("Syntax error: segment index expected")
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if command == "function" {
                    let _ = self.function_name.insert(name.clone());
                    self.call_rank = 0;
                    Function(name, count)
                } else {
                    self.call_rank += 1;
                    let function_name = self
                        .function_name
                        .as_ref()
                        .expect("label should be inside a function");
                    Call {
                        callee: name,
                        arg_count: count,
                        return_address: format!("{function_name}$ret.{}", self.call_rank),
                    }
                }
            }
            "return" => {
                self.function_name.as_ref().expect("unexpected return");
                Return
            }
            _ => panic!("invalid command {}", command),
        };

        Some(command)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        return self.lexer.next_token();
    }
}
