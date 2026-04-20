use crate::memory::RamMem;

// Only put mathematical Operations here no operation here will edit the program
// counter in ANY WAY

pub fn Add(ram: &RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
}

pub fn Mult(ram: &RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
}

pub fn Div(ram: &RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
}

// Bitwise And
pub fn Btand(ram: &RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
}
