use std::collections::HashMap;
use bitvec::vec::BitVec;
use std::error::Error;
use std::fs::{File, };
use std::io::{Read, Write};
use std::ops::Range;
use bitvec::field::BitField;
use serde::Serialize;

#[derive(Default)]
pub struct Assembler {
    commands: Vec<BitVec<u8>>,
}

impl Assembler {
    pub fn read_input(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let input = std::fs::read_to_string(input)?;
        self.commands = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            self.commands.push(self.parse_command(line)?);
        }
        Ok(())
    }

    pub fn save_asm(&mut self, output: &str) -> Result<(), Box<dyn Error>> {
        let mut asm = File::create(output)?;
        for bits in &self.commands{
            let bits = bits.as_raw_slice();
            asm.write_all(bits)?;
        }
        Ok(())
    }

    pub fn log_asm(&self, output: &str) -> Result<(), Box<dyn Error>> {
        #[derive(Serialize, Default)]
        struct Output {
            command_num: u8,
            lhs: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            rhs: Option<u32>
        }
        
        let mut log = File::create(output)?;
        let mut commands = HashMap::new();
        for bits in &self.commands {
            let mut output = Output::default();
            match bits[..8].load_le::<u8>() {
                118 => {
                    output.command_num = 118;
                    output.lhs = bits[8..22].load_le();
                    commands.insert("loadc", output);
                },
                231 => {
                    output.command_num = 231;
                    output.lhs = bits[8..24].load_le();
                    commands.insert("load", output);
                },
                238 => {
                    output.command_num = 238;
                    output.lhs = bits[8..24].load_le();
                    commands.insert("store", output);
                },
                132 => {
                    output.command_num = 132;
                    output.lhs = bits[8..24].load_le();
                    output.rhs = Some(bits[24..40].load_le());
                    commands.insert("popcnt", output);
                },
                _ => unreachable!()
            }
        }
        log.write_all(serde_yaml::to_string(&commands)?.as_bytes())?;
        Ok(())
    }

    
    fn store_bits(&self, accumulator: &mut u32, data: u32, bit_range: Range<u8>) {
        let lower_bound = bit_range.start;
        for bit in bit_range {
            *accumulator |= ((data >> bit) & 1) << (bit - lower_bound);
        }
    }

    fn parse_command(&self, command: &str) -> Result<BitVec<u8>, Box<dyn Error>> {
        let (cmd, args) = command.split_once(" ").unwrap();
        let args = args
            .split(", ")
            .collect::<Vec<&str>>();
        let mut bits = BitVec::<u8, _>::repeat(false, 40);
        match cmd {
            "loadc" => {
                bits[0..8].store(118u8.to_le());
                let arg = args[0].parse::<u32>()?;
                if arg > (1 << 14) - 1 {
                    return Err("Integer overflow".into());
                }
                bits[8..22].store(arg.to_le());
            }
            "load" => {
                bits[0..8].store(231u8.to_le());
                let arg = args[0].parse::<u32>()?;
                if arg > (1 << 16) - 1 {
                    return Err("Integer overflow".into());
                }
                bits[8..24].store(arg.to_le());
            }
            "store" => {
                bits[0..8].store(238u8.to_le());
                let arg = args[0].parse::<u32>()?;
                if arg > (1 << 16) - 1 {
                    return Err("Integer overflow".into());
                }
                bits[8..24].store(arg.to_le());
            }
            "popcnt" => {
                bits[0..8].store(132u8.to_le());
                let (lhs, rhs) = (args[0].parse::<u32>()?, args[1].parse::<u32>()?);
                let bound = (1 << 16) - 1;
                if lhs > bound || rhs > bound {
                    return Err("Integer overflow".into());
                }
                bits[8..24].store(lhs.to_le());
                bits[24..40].store(rhs.to_le());
            }
            _ => return Err(format!("Unknown command: {}", cmd).into()),
        }
        Ok(bits)
    }
}
