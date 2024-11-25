use clap::Parser;
use crate::args::Args;
use crate::assembler::Assembler;
use crate::interpreter::Interpreter;

mod args;
mod assembler;
mod interpreter;
fn main() {
    let args = Args::parse();
    
    let mut asm = Assembler::default();
    asm.read_input(&args.input).unwrap();
    asm.save_asm(&args.output_asm).unwrap();
    asm.log_asm(&args.log).unwrap();
    
    let mut interpreter = Interpreter::with_capacity(args.capacity);
    interpreter.read_commands(&args.output_asm).unwrap();
    interpreter.execute(&args.output_int).unwrap()
    
}
