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
        use MemorySegment::*;
        match self {
            Local => "@LCL",
            Argument => "@ARG",
            This => "@THIS",
            That => "@THAT",
            _ => "#INVALID_LABEL#",
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
    IfGoto(String),
    Function(String, u16),
    Call {
        callee: String,
        arg_count: u16,
        return_address: String,
    },
    Return,
}

impl Command {
    pub fn init() -> Vec<String> {
        let mut instructions = ["@256", "D=A", "@SP", "M=D"]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut call_sys = Self::translate_call("Sys.init".into(), 0, "Sys.init$ret.0".into());
        instructions.append(&mut call_sys);
        instructions
    }

    pub fn translate(self, filename: &str, index: &mut u64) -> Vec<String> {
        use Command::*;
        match self {
            Arithmetic(operation) => Self::translate_arithmetic(index, filename, operation),
            Push(segment, index) => Self::translate_push(filename, segment, index),
            Pop(segment, index) => Self::translate_pop(filename, segment, index),
            Label(label) => Self::translate_label(label),
            Goto(label) => Self::translate_goto(label),
            IfGoto(label) => Self::translate_if(label),
            Function(name, local_var) => Self::translate_function(name, local_var),
            Call {
                callee: name,
                arg_count,
                return_address,
            } => Self::translate_call(name, arg_count, return_address),
            Return => Self::translate_return(),
        }
    }

    fn translate_arithmetic(index: &mut u64, filename: &str, operation: Arithmetic) -> Vec<String> {
        use Arithmetic::*;
        *index += 1;
        let label = format!("({filename}$Arith_True.{index})");
        let l = format!("@{filename}$Arith_True.{index}");
        // Factor out the main computation of the repeating stuff
        let d = match operation {
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

        let instructions = match operation {
            ADD | SUB => vec!["@SP", "AM=M-1", "D=M", "A=A-1", d],
            AND | OR => vec!["@SP", "AM=M-1", "D=M", "A=A-1", d],
            NEG | NOT => vec!["@SP", "A=M-1", d],
            EQ | GT | LT => vec![
                "@SP", "AM=M-1", "D=M", "A=A-1", "D=M-D", "M=-1", &l, d, "@SP", "A=M-1", "M=0",
                &label,
            ],
        };
        instructions.into_iter().map(|x| x.to_string()).collect()
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
                vec![alias, "D=M"]
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
                &label, "D=M", &i, "D=A+D", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D",
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
                vec!["@SP", "M=M-1", "A=M", "D=M", &alias, "M=D"]
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
        vec![format!("({label})")]
    }

    fn translate_goto(label: String) -> Vec<String> {
        vec![format!("@{label}"), "0;JMP".into()]
    }

    fn translate_if(label: String) -> Vec<String> {
        let l = format!("@{label}");
        ["@SP", "AM=M-1", "D=M", &l, "D;JNE"]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn translate_function(name: String, local_var_count: u16) -> Vec<String> {
        vec![
            format!("({name})"),
            format!("@{local_var_count}"),
            "D=A".into(),
            format!("({name}$arg.{local_var_count})"),
            format!("@{name}$arg.{local_var_count}.end"),
            "D;JEQ".into(),
            "D=D-1".into(),
            "@SP".into(),
            "A=M".into(),
            "M=0".into(),
            "@SP".into(),
            "M=M+1".into(),
            format!("@{name}$arg.{local_var_count}"),
            "0;JMP".into(),
            format!("({name}$arg.{local_var_count}.end)"),
        ]
    }
    fn translate_call(name: String, arg_count: u16, return_address: String) -> Vec<String> {
        let c = format!("@{name}");
        let r = format!("@{return_address}");
        let l = format!("({return_address})");
        let arg = format!("@{arg_count}");

        let push = ["@SP", "A=M", "M=D", "@SP", "M=M+1"];

        let mut instructions = vec![&r, "D=A"];
        instructions.extend(&push);

        ["@LCL", "@ARG", "@THIS", "@THAT"]
            .into_iter()
            .for_each(|x| {
                instructions.extend([x, "D=M"]);
                instructions.extend(&push)
            });

        instructions.extend(["@SP", "D=M", "@5", "D=D-A", &arg, "D=D-A", "@ARG", "M=D"]);
        instructions.extend(["@SP", "D=M", "@LCL", "M=D"]);
        instructions.extend([&c, "0;JMP", &l]);

        instructions.into_iter().map(|x| x.to_string()).collect()
    }
    fn translate_return() -> Vec<String> {
        let mut instructions = vec![
            "@LCL", "D=M", "@R13", "M=D", "@5", "A=D-A", "D=M", "@R14", "M=D", "@SP", "M=M-1",
            "A=M", "D=M", "@ARG", "A=M", "M=D", "D=A", "@SP", "M=D+1",
        ];
        ["@THAT", "@THIS", "@ARG", "@LCL"]
            .into_iter()
            .for_each(|x| instructions.extend(["@R13", "AM=M-1", "D=M", x, "M=D"]));
        instructions.extend(["@R14", "A=M", "0;JMP"]);

        instructions.into_iter().map(|x| x.to_string()).collect()
    }
}
