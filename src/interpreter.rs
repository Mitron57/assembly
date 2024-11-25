use bitvec::field::BitField;
use bitvec::vec::BitVec;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub struct Interpreter {
    value: BitVec<u8>,
    memory: HashMap<u32, BitVec<u8>>,
    commands: BitVec<u8>,
}

impl Interpreter {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            value: BitVec::repeat(false, 14),
            memory: HashMap::with_capacity(capacity),
            commands: BitVec::new(),
        }
    }

    pub fn read_commands(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let mut input = File::open(input)?;
        std::io::copy(&mut input, &mut self.commands)?;
        Ok(())
    }

    pub fn execute(mut self, output: &str) -> Result<(), Box<dyn Error>> {
        let commands = self.commands.len() / 40;
        let mut register = 0;
        for cmd in 0..commands {
            let commandline = &self.commands[cmd * 40..(cmd + 1) * 40];
            match commandline[..8].load_le::<u8>() {
                118 => {
                    self.value = commandline[8..22].to_bitvec();
                    register += 1;
                }
                231 => {
                    let region = commandline[8..24].load_le::<u32>();
                    if let Some(chunk) = self.memory.get(&region) {
                        self.value = chunk.to_bitvec();
                    } else {
                        self.value = BitVec::repeat(false, 14);
                    }
                }
                238 => {
                    let region = commandline[8..24].load_le::<u32>();
                    self.memory.insert(region, self.value.clone());
                }
                132 => {
                    let region_result = commandline[8..24].load_le::<u32>();
                    let region_value = commandline[24..].load::<u32>();
                    let value = self
                        .memory
                        .get(&region_value)
                        .map_or(BitVec::repeat(false, 14), BitVec::clone);
                    let ones = value.count_ones();
                    let mut popcnt = BitVec::repeat(false, 14);
                    popcnt.store(ones);
                    self.memory.insert(region_result, popcnt);
                }
                _ => unreachable!(),
            }
        }
        std::fs::write(output, serde_yaml::to_string(&self.memory)?)?;
        Ok(())
    }
}
