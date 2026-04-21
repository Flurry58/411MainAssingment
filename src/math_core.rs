use crate::memory::RamMem;
// logan im killing u
// Only put mathematical Operations here no operation here will edit the program
// counter in ANY WAY

pub fn Add(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
    reg_a = reg_b + reg_c
}

pub fn Mult(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
    reg_a = reg_b * reg_c
}

pub fn Div(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
    reg_a = reg_b / reg_c
}

// Bitwise And
pub fn Btand(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    counter + 1
    reg_a = reg_b & reg_c
}
