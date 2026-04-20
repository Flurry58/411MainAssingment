use rum::instruction_process;
use rum::rumload;
use std::env;
fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    instruction_process::run_program(instructions);
}
