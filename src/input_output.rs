use crate::memory::RamMem;
use std::io::{self, Read, Write};
pub fn input(ram: &mut RamMem, reg_c: usize, counter: u32) -> u32 {
    let mut buf = [0u8; 1];
    ram.reg_list[reg_c] = match io::stdin().read(&mut buf) {
        Ok(1) => buf[0] as u32,
        _ => 0xFFFFFFFF,
    };
    counter + 1
}

pub fn output(ram: &mut RamMem, reg_c: usize, counter: u32) -> u32 {
    let byte = ram.reg_list[reg_c] as u8;
    io::stdout().write_all(&[byte]).unwrap();
    counter + 1
}
