# VM Translator

## Introduction

This project is an implementation of the VM Translator for the Hack computer, written in Rust. The VM Translator
translates high-level virtual machine (VM) code into Hack assembly language, which can then be assembled into machine
code for the Hack computer as described in the "From NAND to Tetris" course.

## VM Language Contract

The Hack VM language consists of stack-based instructions that can be categorized into three types:

1. **Arithmetic and Logical Commands**: Perform arithmetic and logical operations.
    - `add`, `sub`, `neg`, `eq`, `gt`, `lt`, `and`, `or`, `not`
2. **Memory Access Commands**: Access different segments of the VM memory.
    - `push segment index`, `pop segment index`
    - Segments: `constant`, `local`, `argument`, `this`, `that`, `pointer`, `temp`, `static`
3. **Program Flow Commands**: Handle branching and function calls.
    - `label label`, `goto label`, `if-goto label`
    - `function functionName nVars`, `call functionName nArgs`, `return`

### VM Commands Details

- **Arithmetic Commands**:
    - `add`: Adds the top two stack values.
    - `sub`: Subtracts the top stack value from the second top stack value.
    - `neg`: Negates the top stack value.
    - `eq`: Pushes `true` if the top two stack values are equal, otherwise `false`.
    - `gt`: Pushes `true` if the second top stack value is greater than the top stack value.
    - `lt`: Pushes `true` if the second top stack value is less than the top stack value.
    - `and`: Performs bitwise AND on the top two stack values.
    - `or`: Performs bitwise OR on the top two stack values.
    - `not`: Performs bitwise NOT on the top stack value.

- **Memory Access Commands**:
    - `push segment index`: Pushes the value from the specified segment and index onto the stack.
    - `pop segment index`: Pops the top stack value into the specified segment and index.

- **Program Flow Commands**:
    - `label label`: Declares a label in the code.
    - `goto label`: Jumps to the specified label.
    - `if-goto label`: Jumps to the specified label if the top stack value is not zero.
    - `function functionName nVars`: Declares a function and initializes local variables.
    - `call functionName nArgs`: Calls a function, passing nArgs arguments.
    - `return`: Returns from the current function, restoring the caller's state.

## Features of the VM Translator

- **Complete Implementation**: Supports all VM commands including arithmetic, memory access, and program flow commands.
- **Optimized Code Generation**: Produces efficient Hack assembly code.
- **Error Handling**: Provides informative error messages for syntax errors and undefined symbols.
- **User-Friendly CLI**: Easy-to-use command-line interface for translating `.vm` files.
- **Modular Design**: Well-structured codebase, making it easy to extend and maintain.

## Installation

To use this translator, you'll need to have Rust installed on your machine. If you don't have Rust installed, you can
get it [here](https://www.rust-lang.org/tools/install).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/veryshyjelly/vm-translator.git
cd vm-translator
```

Build the project using Cargo:

```sh
cargo build --release
```

The executable will be located in the `target/release` directory.

## Usage

To translate a VM file or directory containing VM files, run the following command:

```sh
./vm-translator path/to/yourfile.vm
```

or

```sh
./vm-translator path/to/yourdirectory
```

This will generate a corresponding Hack assembly file with a `.asm` extension in the same directory as the input file(
s).

## Example

Given the following VM code in `example.vm`:

```vm
// Simple function call
function SimpleFunction 2
push constant 10
push constant 20
add
pop local 0
push constant 30
push constant 40
sub
pop local 1
return
```

Running the translator:

```sh
./vm-translator example.vm
```

Will produce the following `example.asm` file:

```asm
// Function("SimpleFunction", 2)
(SimpleFunction)
@2
D=A
(SimpleFunction$arg.2)
@SimpleFunction$arg.2.end
D;JEQ
D=D-1
@SP
A=M
M=0
@SP
M=M+1
@SimpleFunction$arg.2
0;JMP
(SimpleFunction$arg.2.end)

// Push(Constant, 10)
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// Push(Constant, 20)
@20
D=A
@SP
A=M
M=D
@SP
M=M+1

// Arithmetic(ADD)
@SP
AM=M-1
D=M
A=A-1
M=M+D

// Pop(Local, 0)
@LCL
D=M
@0
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// Push(Constant, 30)
@30
D=A
@SP
A=M
M=D
@SP
M=M+1

// Push(Constant, 40)
@40
D=A
@SP
A=M
M=D
@SP
M=M+1

// Arithmetic(SUB)
@SP
AM=M-1
D=M
A=A-1
M=M-D

// Pop(Local, 1)
@LCL
D=M
@1
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// Return
@LCL
D=M
@R13
M=D
@5
A=D-A
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
D=A
@SP
M=D+1
@R13
AM=M-1
D=M
@THAT
M=D
@R13
AM=M-1
D=M
@THIS
M=D
@R13
AM=M-1
D=M
@ARG
M=D
@R13
AM=M-1
D=M
@LCL
M=D
@R14
A=M
0;JMP
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any improvements or
bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The creators of the "From NAND to Tetris" course, Noam Nisan and Shimon Schocken, for providing the framework and
  inspiration for this project.
- The Rust community for their excellent documentation and support.
