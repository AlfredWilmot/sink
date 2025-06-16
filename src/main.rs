#![allow(unused_variables, dead_code)]

use colored::Colorize;
use std::{f32, process::exit};

use clap::{Parser, Subcommand};

use sink::{cpu::CPU, float::DeconstructedFloat32};

/// Let's sink down into the dingy depths of the OS!
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Emulate a cpu that's loosely based on the CHIP-8 instruction set
    Cpu{

        /// load the cpu register with data
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        reg: Option<Vec<String>>,

        /// list of system opcodes for the cpu to execute
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        sys: Vec<String>,

        /// list of program opcodes for the cpu to execute
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        prog: Vec<String>,

    },
    /// Deconstruct floats into their fixed-point binary representations
    Float{
        /// floating point number
        number: f32
    },
}


fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Float{number} => {
            // is the number within the allowed range?
            if (f32::MIN..=f32::MAX).contains(&number) {
                DeconstructedFloat32::new(&number).print();
                exit(0);
            }

            println!(
                "{}",
                format!("Must be within range: [{:?}, {:?}]", f32::MIN, f32::MAX).red(),
            );
        }
        Commands::Cpu { reg, sys, prog } => {
            let mut cpu = CPU::new();

            // attempt to update the CPU register with the provided values
            if let Some(reg) = reg {
                let result = parse_args_to_byte_array(&reg);
                for (idx, entry) in result.iter().enumerate() {
                    cpu.reg[idx] = *entry;
                }
                println!("Loaded register data:\t {:x?}", cpu.reg);
            }

            // attempt to load opcodes into memory
            let result = parse_args_to_byte_array(&sys);
            cpu.write_system_mem(&result);
            println!("Loaded system memory:\t {:x?}", result);

            let result = parse_args_to_byte_array(&prog);
            cpu.write_prog_mem(&result);
            println!("Loaded program memory:\t {:x?}", result);

            // let's go!
            cpu.run();
            println!("Computed registers:\t {:x?}", cpu.reg);
        }
    }
    exit(1);

}

/// Iteratively strip two chars from each entry in vector of Strings
/// until all String entries have been consumed into an array of bytes
fn parse_args_to_byte_array(input: &Vec<String>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for entry in input {
        let mut reversed_chars: Vec<char> = entry.chars().rev().collect();
        while reversed_chars.len() > 0 {
            let msb = reversed_chars.pop().unwrap();
            let lsb = reversed_chars.pop().unwrap();
            let val: String  = [msb, lsb].iter().collect();
            result.push(u8::from_str_radix(&val, 16).unwrap());
        }
    }
    result
}
