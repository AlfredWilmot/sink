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
                for (idx, val) in reg.iter().enumerate() {
                    println!("Inserted {} into register {}", val, idx);
                    cpu.reg[idx] = *val;
                }
            }

            // attempt to load opcodes into memory
            cpu.write_system_mem(&sys);
            cpu.write_prog_mem(&prog);

            // let's go!
            cpu.run()
        }
    }
    exit(1);

}
