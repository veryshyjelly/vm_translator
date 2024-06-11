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
    pub fn label(&self) -> String {
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
    Call(String, u16),
    Return,
}
