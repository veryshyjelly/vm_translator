use crate::translator::MemorySegment::{
    Argument, Constant, Local, Pointer, Static, Temp, That, This,
};

#[derive(Debug)]
pub enum Arithmetic {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

#[derive(Debug)]
pub enum MemorySegment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

#[derive(Debug)]
pub enum Command {
    Arithmetic(Arithmetic),
    Push(MemorySegment, u16),
    Pop(MemorySegment, u16),
    Label(String),
    Goto(String),
    If(String),
    Function(String, u16),
    Call(String, u16),
    Return,
}

impl Command {
    pub fn translate(self, filename: &str, index: &mut u64) -> Vec<String> {
        match self {
            Command::Arithmetic(command) => Self::translate_arithmetic(index, command),
            Command::Push(mem, idx) => Self::translate_push(filename, mem, idx),
            Command::Pop(mem, idx) => Self::translate_pop(filename, mem, idx),
            Command::Label(label) => Self::translate_label(label),
            Command::Goto(label) => Self::translate_goto(label),
            Command::If(label) => Self::translate_if(label),
            Command::Function(name, arg) => Self::translate_function(name, arg),
            Command::Call(name, arg) => Self::translate_call(name, arg),
            Command::Return => Self::translate_return(),
        }
    }

    pub fn translate_arithmetic(index: &mut u64, command: Arithmetic) -> Vec<String> {
        use Arithmetic::*;
        *index += 1;
        let label = format!("(Arithmetic${index})");
        let l = format!("@Arithmetic${index}");
        match command {
            ADD => vec!["@SP", "AM=M-1", "D=M", "A=M-1", "M=M+D"],
            SUB => vec!["@SP", "AM=M-1", "D=M", "A=M-1", "M=M-D"],
            NEG => vec!["@SP", "A=M-1", "M=-M"],
            EQ => vec![
                "@SP", "AM=M-1", "D=M", "A=A-1", "D=M-D", "M=-1", &l, "D;JEQ",
                "@SP", "A=M-1", "M=0", &label,
            ],
            GT => vec![
                "@SP", "AM=M-1", "D=M", "A=A-1", "D=M-D", "M=-1", &l, "D;JGT",
                "@SP", "A=M-1", "M=0", &label,
            ],
            LT => vec![
                "@SP", "AM=M-1", "D=M", "A=A-1", "D=M-D", "M=-1", &l, "D;JLT",
                "@SP", "A=M-1", "M=0", &label,
            ],
            AND => vec!["@SP", "AM=M-1", "D=M", "A=A-1", "M=D&M"],
            OR => vec!["@SP", "AM=M-1", "D=M", "A=A-1", "M=D|M"],
            NOT => vec!["@SP", "A=M-1", "M=!M", "D=1", "M=M&D"],
        }
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }
    pub fn translate_push(filename: &str, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{filename}.{index}");
        let temp = format!("@R{}", index + 5);
        let mut instructions = match segment {
            Local => vec!["@LCL", "D=M", &i, "A=A+D", "D=M"],
            Argument => vec!["@ARG", "D=M", &i, "A=A+D", "D=M"],
            This => vec!["@THIS", "D=M", &i, "A=A+D", "D=M"],
            That => vec!["@THAT", "D=M", &i, "A=A+D", "D=M"],
            Constant => vec![&i, "D=A"],
            Static => vec![&stat, "D=M"],
            Pointer => {
                vec![
                    if index == 0 {
                        "@THIS"
                    } else if index == 1 {
                        "@THAT"
                    } else {
                        panic!("invalid index for pointer")
                    },
                    "D=M",
                    &i,
                    "A=A+D",
                    "D=M",
                ]
            }
            Temp => {
                assert!(index < 8);
                vec![&temp, "D=M"]
            }
        };
        instructions.extend(&["@SP", "A=M", "M=D", "@SP", "M=M+1"]);
        instructions
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }
    pub fn translate_pop(filename: &str, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{filename}.{index}");
        let temp = format!("@R{}", index + 5);
        match segment {
            Local => vec![
                "@LCL", "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D",
            ],
            Argument => vec![
                "@ARG", "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D",
            ],
            This => vec![
                "@THIS", "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D",
            ],
            That => vec![
                "@THAT", "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D",
            ],
            Static => vec!["@SP", "M=M-1", "A=M", "D=M", &stat, "M=D"],
            Pointer => {
                vec![
                    if index == 0 {
                        "@THIS"
                    } else if index == 1 {
                        "@THAT"
                    } else {
                        panic!("invalid index for pointer")
                    },
                    "D=M",
                    &i,
                    "D=A+D",
                    "@R13",
                    "M=D",
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "D=M",
                    "@R13",
                    "A=M",
                    "M=D",
                ]
            }
            Temp => {
                assert!(index < 8);
                vec!["@SP", "M=M-1", "A=M", "D=M", &temp, "M=D"]
            }
            _ => panic!("invalid operation: pop {:?} {}", segment, index),
        }
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }
    pub fn translate_label(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    pub fn translate_goto(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    pub fn translate_if(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    pub fn translate_function(name: String, arg: u16) -> Vec<String> {
        todo!("not implemented")
    }
    pub fn translate_call(name: String, arg: u16) -> Vec<String> {
        todo!("not implemented")
    }
    pub fn translate_return() -> Vec<String> {
        todo!("not implemented")
    }
}
