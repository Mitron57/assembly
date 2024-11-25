use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long = "inp")]
    pub input: String,

    #[arg(short = 'a', long)]
    pub output_asm: String,
    
    #[arg(short = 'i', long)]
    pub output_int: String,

    #[arg(short, long)]
    pub log: String,
    
    #[arg(short, long)]
    pub capacity: usize,
}
