pub mod instructions;
pub mod assembler;
pub mod repl;
pub mod vm;

#[macro_use]
extern crate nom;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
