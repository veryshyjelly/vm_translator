use crate::command::{Arithmetic, Command, MemorySegment};

pub struct Translator {
    filename: String,
    index: u64,
    function_name: Option<String>,
    call_rank: u64,
}

impl Translator {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            index: 0,
            function_name: None,
            call_rank: 0,
        }
    }
    pub fn init(&mut self) -> Vec<String> {
        let mut instructions = ["@256", "D=A", "@SP", "M=D"]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let _ = self.function_name.insert("Sys.System".into());
        let mut call_sys = self.translate_call("Sys.init".into(), 0);
        self.function_name = None;
        instructions.append(&mut call_sys);
        instructions
    }

    pub fn translate_command(&mut self, command: Command) -> Vec<String> {
        use Command::*;
        match command {
            Arithmetic(operation) => self.translate_arithmetic(operation),
            Push(segment, index) => self.translate_push(segment, index),
            Pop(segment, index) => self.translate_pop(segment, index),
            Label(label) => self.translate_label(label),
            Goto(label) => self.translate_goto(label),
            IfGoto(label) => self.translate_if(label),
            Function(name, local_var) => self.translate_function(name, local_var),
            Call(name, arg_count) => self.translate_call(name, arg_count),
            Return => self.translate_return(),
        }
    }

    fn translate_arithmetic(&mut self, arithmetic: Arithmetic) -> Vec<String> {
        use Arithmetic::*;
        let label = format!("({}$Arith_True.{})", self.filename, self.index);
        let l = format!("@{}$Arith_True.{}", self.filename, self.index);
        // increase the number of calls to this function
        self.index += 1;

        // Factor out the main computation of the repeating stuff
        let d = match arithmetic {
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

        let instructions = match arithmetic {
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
    fn translate_push(&self, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{}.{index}", self.filename);
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

    fn translate_pop(&self, segment: MemorySegment, index: u16) -> Vec<String> {
        use MemorySegment::*;
        let i = format!("@{index}");
        let stat = format!("@{}.{index}", self.filename);
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

    fn translate_label(&self, label: String) -> Vec<String> {
        vec![format!(
            "({}${label})",
            self.function_name
                .as_ref()
                .expect("label should be inside a function")
        )]
    }

    fn translate_goto(&self, label: String) -> Vec<String> {
        vec![
            format!(
                "@{}${label}",
                self.function_name
                    .as_ref()
                    .expect("goto should be inside a function")
            ),
            "0;JMP".into(),
        ]
    }

    fn translate_if(&self, label: String) -> Vec<String> {
        let l = format!(
            "@{}${label}",
            self.function_name
                .as_ref()
                .expect("if-goto should be inside a function")
        );
        ["@SP", "AM=M-1", "D=M", &l, "D;JNE"]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn translate_function(&mut self, name: String, local_var_count: u16) -> Vec<String> {
        self.call_rank = 0;
        let name = self.function_name.insert(name);
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

    fn translate_call(&mut self, name: String, arg_count: u16) -> Vec<String> {
        let return_address = format!(
            "{}$ret.{}",
            self.function_name
                .as_ref()
                .expect("call should be inside a function"),
            self.call_rank
        );
        self.call_rank += 1;
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

    fn translate_return(&self) -> Vec<String> {
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
