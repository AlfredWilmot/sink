#![allow(unused_variables, dead_code)]

use clap::Parser;

/// Convert from a floating-point to fixed-point number
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct  Args {

    /// specify a floating point number
    number: f32,

    /// indicate whether to print debug info
    #[arg(short, long)]
    debug: bool
}

/// extract the sign-bit from a u32 representing a float
/// returns the sign-bit as a single byte
/// assumes BE
fn get_sign_bit(val: u32) -> u8 {
    (val>> 31) as u8
}

fn main() {
    let args = Args::parse();

    // get number from user input
    let float: f32 = args.number;

    // convert the input to u32 for bit-manipuation
    let val: u32 = float.to_bits();

    // isolate the sign-bit
    let sign_bit: u8 = get_sign_bit(val);

    println!("{}", float);

    if args.debug {
        println!("| sign | {:08b} |", sign_bit);
    }
}
