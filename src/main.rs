
pub mod op;
pub mod runtime;
pub mod prog;
pub mod mem;

pub use op::Op;
pub use runtime::Runtime;
pub use prog::Prog;
pub use mem::Mem;

use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about = "Brainfuck interpreter", long_about = None)]
struct Cli {
    path: String, // path to the brainfuck source

    #[arg(short = 'm', long = "mem", default_value_t = 30000)]
    mem_size: usize, // amount of memory to allocate
}

fn main() {
    let Cli { path, mem_size } = Cli::parse();

    let prog: Prog = fs::read_to_string(path).expect("Could not read file").into();

    let mut i = io::stdin();
    let mut o = io::stdout();
    let mut rt = Runtime::new(mem_size, &mut i, &mut o);

    rt.exec(prog);
}
