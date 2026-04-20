use std::collections::BTreeSet;
use std::collections::HashMap;

pub struct RamMem {
    reg_list: Vec<u32>,
    address_space: BTreeSet<u32>,
    current_address_max: u32,
    memory_store: HashMap<u32, Vec<u32>>,
}

impl RamMem {
    // Run generate_new address space after setup REQUIRED
    pub fn setup() -> Self {
        let mut regs = vec![0; 8];
        Self {
            reg_list: regs,
            address_space: BTreeSet::new(),
            current_address_max: 1,
            memory_store: HashMap::new(),
        }
    }
    pub fn load_initial_program(&mut self, inst_list: Vec<u32>) {
        self.memory_store.insert(0, inst_list);
    }

    pub fn fetch(&self, counter: u32) -> Option<u32> {
        self.memory_store.get(&0)?.get(counter as usize).copied()
    }

    pub fn load_program(&mut self, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
        let segment = self.reg_list[reg_b] as usize;
        if segment != 0 {
            let key = segment as u32;
            if let Some(dupmem) = self.memory_store.get(&key).cloned() {
                self.memory_store.insert(0, dupmem);
            } else {
                println!(
                    "ERROR: load_program tried to load unmapped segment {}",
                    segment
                );
                return u32::MAX;
            }
        }
        self.reg_list[reg_c]
    }

    pub fn load_value(&mut self, reg_a: usize, value: u32, counter: u32) -> u32 {
        self.reg_list[reg_a] = value;
        counter + 1
    }

    // Each address space is generated 100 words at a time
    // so we don't overwhelm the empty address space Set
    // Run anytime the address space is empty
    pub fn generate_new_address_space(&mut self) {
        if self.address_space.is_empty() {
            let start = self.current_address_max;
            let end = start + 100;

            self.address_space.extend(start..end);
            self.current_address_max = end;
        }
    }
    pub fn load(&mut self, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
        let segment = self.reg_list[reg_b] as u32;
        let offset = self.reg_list[reg_c] as usize;
        self.reg_list[reg_a] = self.memory_store[&segment][offset];
        counter + 1
    }

    pub fn store(&mut self, reg_a: usize, reg_b: usize, reg_c: usize, counter: u32) -> u32 {
        let segment = self.reg_list[reg_a] as u32;
        let offset = self.reg_list[reg_b] as usize;
        let value = self.reg_list[reg_c];
        self.memory_store.get_mut(&segment).unwrap()[offset] = value;
        counter + 1
    }

    pub fn map(&mut self, reg_c: usize, reg_b: usize, counter: u32) -> u32 {
        self.generate_new_address_space();

        let address: u32 = self.address_space.pop_first().unwrap();
        println!("New Address Allocated: {}", address); //DEBUG

        let words = vec![0u32; self.reg_list[reg_c] as usize];
        self.memory_store.insert(address, words);
        self.reg_list[reg_b] = address;
        counter + 1
    }

    pub fn unmap(&mut self, reg_c: usize, counter: u32) -> u32 {
        if self.reg_list[reg_c] == 0 {
            return u32::MAX;
        }
        let segment_id = &self.reg_list[reg_c];
        if let Some((key, _)) = self.memory_store.remove_entry(segment_id) {
            println!("{:?}", self.address_space); //DEBUG
            self.address_space.insert(key);
        } else {
            return u32::MAX;
        }

        return counter + 1;
    }
}
