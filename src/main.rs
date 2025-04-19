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

/// Extract the sign-bit from a u32 representing a float.
/// A Copy of val is bit-shifted so the sign-bit is the LSB
/// (assumes val is BigEndian).
fn get_sign(val: u32) -> u32 {
    val >> 31
}

/// Extract the exponent-byte from a u32 representing a float.
/// A Copy of val is bit-shifted so the exponent-byte is the LSB,
/// and then a mask is used to remove the sign-bit
/// that prepends the exponent-byte (assumes val is BigEndian).
fn get_exponent(val: u32) -> u32 {
    (val >> 23) & 0b_00000000_11111111
}

///  bit-layout of the three components encoded into the f32 type:
///
/// [] [<----- EXPONENT ---->] [<---- MANTISSA ---->]
/// 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 .. 0
/// ^
/// (sign)
///
fn main() {
    let args = Args::parse();

    // get number from user input
    let float: f32 = args.number;

    // convert the input to u32 for bit-manipuation
    let val: u32 = float.to_bits();

    // isolate the sign-bit
    let sign_bit: u32 = get_sign(val);

    // isolate the Exponent
    let exponent_byte: u32 = get_exponent(val);

    println!("{}", float);

    if args.debug {
        println!("| input    | {:032b} |", val);
        println!("| sign     | {:032b} |", sign_bit);
        println!("| exponent | {:032b} |", exponent_byte);
    }
}
