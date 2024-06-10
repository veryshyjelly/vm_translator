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

impl MemorySegment {
    fn label(&self) -> String {
        match self {
            Local => "@LCL",
            Argument => "@ARG",
            This => "@THIS",
            That => "@THAT",
            _ => panic!("label not available for {:?}", self),
        }
        .to_string()
    }
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
        use Command::*;
        match self {
            Arithmetic(command) => Self::translate_arithmetic(index, command),
            Push(mem, idx) => Self::translate_push(filename, mem, idx),
            Pop(mem, idx) => Self::translate_pop(filename, mem, idx),
            Label(label) => Self::translate_label(label),
            Goto(label) => Self::translate_goto(label),
            If(label) => Self::translate_if(label),
            Function(name, arg) => Self::translate_function(name, arg),
            Call(name, arg) => Self::translate_call(name, arg),
            Return => Self::translate_return(),
        }
    }

    fn translate_arithmetic(index: &mut u64, command: Arithmetic) -> Vec<String> {
        use Arithmetic::*;
        *index += 1;
        let label = format!("(Arith_True${index})");
        let l = format!("@Arith_True${index}");
        let d = match command {
            ADD => "M=M+D",
            SUB => "M=M-D",
            NEG => "M=-M",
            EQ => "D;JEQ",
            GT => "D;JGT",
            LT => "D;JLT",
            AND => "M=M&D",
            OR => "M=M|D",
            NOT => "M=!M",
        };

        match command {
            ADD | SUB => vec!["@SP", "AM=M-1", "D=M", "A=A-1", d],
            AND | OR => vec!["@SP", "AM=M-1", "D=M", "A=A-1", d],
            NEG | NOT => vec!["@SP", "A=M-1", d],
            EQ | GT | LT => vec![
                "@SP", "AM=M-1", "D=M", "A=A-1", "D=M-D", "M=-1", &l, d, "@SP", "A=M-1", "M=0",
                &label,
            ],
        }
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
    }
    fn translate_push(filename: &str, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{filename}.{index}");
        let temp = format!("@R{}", index + 5);
        let label = segment.label();

        // Translate the push command into assembly instructions
        let mut instructions = match segment {
            Local | Argument | This | That => vec![&label, "D=M", &i, "A=A+D", "D=M"],
            Constant => vec![&i, "D=A"],
            Static => vec![&stat, "D=M"],
            Pointer => {
                let alias = if index == 0 {
                    "@THIS"
                } else if index == 1 {
                    "@THAT"
                } else {
                    panic!("invalid index for pointer")
                };
                vec![alias, "D=M", &i, "A=A+D", "D=M"]
            }
            Temp => {
                assert!(index < 8);
                vec![&temp, "D=M"]
            }
        };
        instructions.extend(&["@SP", "A=M", "M=D", "@SP", "M=M+1"]);
        instructions.into_iter().map(|x| x.to_string()).collect()
    }
    fn translate_pop(filename: &str, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{filename}.{index}");
        let temp = format!("@R{}", index + 5);
        let label = segment.label();

        // Translate the pop command into the assembly instructions
        let instructions = match segment {
            Local | Argument | This | That => vec![
                &label, "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13", "A=M",
                "M=D",
            ],

            Static => vec!["@SP", "M=M-1", "A=M", "D=M", &stat, "M=D"],
            Pointer => {
                let alias = if index == 0 {
                    "@THIS"
                } else if index == 1 {
                    "@THAT"
                } else {
                    panic!("invalid index for pointer")
                };
                vec![
                    alias, "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                    "A=M", "M=D",
                ]
            }
            Temp => {
                assert!(index < 8);
                vec!["@SP", "M=M-1", "A=M", "D=M", &temp, "M=D"]
            }
            _ => panic!("invalid operation: pop {:?} {}", segment, index),
        };
        instructions.into_iter().map(|x| x.to_string()).collect()
    }

    fn translate_label(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    fn translate_goto(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    fn translate_if(label: String) -> Vec<String> {
        todo!("not implemented")
    }
    fn translate_function(name: String, local_var_count: u16) -> Vec<String> {
        todo!("not implemented")
    }
    fn translate_call(name: String, arg_count: u16) -> Vec<String> {
        todo!("not implemented")
    }
    fn translate_return() -> Vec<String> {
        todo!("not implemented")
    }
}
