use crate::input_output::output;
use crate::input_output::{self, input};
use crate::math_core::{Add, Btand, Div, Mult};
use crate::memory::RamMem;
use bitpack;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}
static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mult,
    Div,
    BitNAND,
    Halt,
    MapSeg,
    UnMap,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64).unwrap() as u32
}
/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32(
        bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64).unwrap() as u32,
    )
}

// Run the program
pub fn run_program(inst_list: Vec<u32>) {
    let mut ram = RamMem::setup();
    ram.generate_new_address_space(); // REQUIRED after setup
    ram.load_initial_program(inst_list); // Put instruction list into memory

    let mut counter: u32 = 0;
    while counter != u32::MAX && ram.fetch(counter).is_some() {
        let inst = ram.fetch(counter).unwrap();
        //go to the instruction and change counter depending on return value
        counter = dis(inst, counter, &mut ram);
    }
}

// Returns program counter, this may change if jumped which is why it needs to return it
// It may return U32::Max if it encountered a halt or unkown instruction which will instantly
// end the program
pub fn dis(inst: Umi, counter: u32, ram: &mut RamMem) -> u32 {
    // println!(CMov
    //     "counter: {} | inst: {:#034b} | op: {:?}",
    //     counter,
    //     inst,
    //     op(inst)
    // ); //DEBUG USAGE ONLY
    match op(inst) {
        Some(Opcode::CMov) => CMov(
            ram,
            get(&RA, inst) as usize,
            get(&RB, inst) as usize,
            get(&RC, inst) as usize,
            counter,
        ),
        Some(Opcode::Load) => ram.load(
            get(&RA, inst) as usize,
            get(&RB, inst) as usize,
            get(&RC, inst) as usize,
            counter,
        ),
        Some(Opcode::Add) => Add(
            ram,
            get(&RA, inst) as usize,
            get(&RB, inst) as usize,
            get(&RC, inst) as usize,
            counter,
        ),
        Some(Opcode::Store) => ram.store(
            get(&RA, inst) as usize,
            get(&RB, inst) as usize,
            get(&RC, inst) as usize,
            counter,
        ),
        Some(Opcode::Mult) => counter + 1,
        Some(Opcode::Div) => counter + 1,
        Some(Opcode::BitNAND) => counter + 1,
        Some(Opcode::Halt) => u32::MAX, //End
        Some(Opcode::MapSeg) => ram.map(get(&RC, inst) as usize, get(&RB, inst) as usize, counter),
        Some(Opcode::UnMap) => ram.unmap(get(&RC, inst) as usize, counter),
        Some(Opcode::Output) => output(ram, get(&RC, inst) as usize, counter),
        Some(Opcode::Input) => input(ram, get(&RC, inst) as usize, counter),
        Some(Opcode::LoadProgram) => {
            ram.load_program(get(&RB, inst) as usize, get(&RC, inst) as usize, counter)
        }
        Some(Opcode::LoadValue) => ram.load_value(get(&RL, inst) as usize, get(&VL, inst), counter),
        None => u32::MAX, //End
    }
}

fn CMov(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    if ram.reg_list[reg_c] != 0 {
        ram.reg_list[reg_a] = ram.reg_list[reg_b];
    }
    counter + 1
}
