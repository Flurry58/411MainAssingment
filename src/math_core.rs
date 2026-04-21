use crate::memory::RamMem;
// logan im killing u
// Only put mathematical Operations here no operation here will edit the program
// counter in ANY WAY

pub fn Add(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    ram.reg_list[reg_a] = ram.reg_list[reg_b] + ram.reg_list[reg_c];
    counter + 1
}

pub fn Mult(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    ram.reg_list[reg_a] = ram.reg_list[reg_b] * ram.reg_list[reg_c];
    counter + 1
}

pub fn Div(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    ram.reg_list[reg_a] = ram.reg_list[reg_b] / ram.reg_list[reg_c];
    counter + 1
}

// Bitwise And
pub fn Btand(ram: &mut RamMem, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
    ram.reg_list[reg_a] = ram.reg_list[reg_b] & ram.reg_list[reg_c];
    counter + 1
}
