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
        /// pass a list of opcode instructions for the cpu to execute
        opcodes: Vec<u16>,

        /// load the cpu register with data
        #[arg(short, long)]
        register: Option<Vec<u8>>,

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
        Commands::Cpu { opcodes, register } => {
            let cpu = CPU::new();
            // attempt to update the CPU register
            if let Some(reg) = register {

            }
        }
        _ => {},
    }
    exit(1);

}
