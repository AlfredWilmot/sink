#![allow(unused_variables, dead_code)]

use clap::Parser;
use colored::Colorize;

/// Convert from a floating-point to fixed-point number
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Args {
    /// specify a floating point number
    number: f32,
}

///  bit-layout of the three components encoded into the f32 type:
///
/// [] [<----- EXPONENT ---->] [<---- MANTISSA ---->]
/// 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 .. 0
/// ^
/// (sign)
///
/// (assumes val is BigEndian)
///
struct DeconstructedFloat32<'a> {
    float: &'a f32,

    // IEEE 754-XXXX standards define:
    //
    sign_bit: u8,      // single bit stored in a byte
    exponent_byte: u8, // byte is fully utilised
    mantissa_bytes: [u8; 3], // 3 bytes with an unised bit (i.e. uses 23 bits)
                       // RADIX = 2 (base)
                       // BIAS = 127 (exponent offset)
}

impl<'a> DeconstructedFloat32<'a> {
    /// create a deconstructed float from an input f32
    pub fn new(val: &'a f32) -> DeconstructedFloat32<'a> {
        // convert the input to u32 for bit-manipuation
        let val_: u32 = val.to_bits();

        // isolate the sign-bit
        let sign_bit: u8 = (val_ >> 31) as u8;

        // isolate the exponent
        let exponent_byte: u8 = (val_ >> 23) as u8;

        // isolate mantissa bytes
        let mantissa_bytes: [u8; 3] = DeconstructedFloat32::get_mantissa(val_);

        DeconstructedFloat32 {
            float: val,
            sign_bit,
            exponent_byte,
            mantissa_bytes,
        }
    }

    /// Extract the mantissa bytes from a u32 representing a float.
    fn get_mantissa(val: u32) -> [u8; 3] {
        let byte_2 = val & 0b00000000_01111111_00000000_00000000; // 16..=22
        let byte_1 = val & 0b00000000_00000000_11111111_00000000; // 8..=15
        let byte_0 = val & 0b00000000_00000000_00000000_11111111; // 0..=7

        [(byte_2 >> 16) as u8, (byte_1 >> 8) as u8, byte_0 as u8]
    }
}

fn main() {
    let args = Args::parse();

    // get number from user input
    let float: f32 = args.number;

    // deconstructs float into its components
    let float_ = DeconstructedFloat32::new(&float);

    let sign_bit_txt = format!("{:b}", float_.sign_bit).on_red();
    let exponent_txt = format!("{:08b}", float_.exponent_byte).on_red();

    let m_ = float_.mantissa_bytes;
    let mantissa_txt = format!("{:07b}{:08b}{:08b}", m_[0], m_[1], m_[2]).on_red();

    println!("\nInput: {}\n", float);
    println!("| input (bits) | {:032b} |", float as u32);
    println!("| sign         | {}{:031b} |", sign_bit_txt, 0);
    println!("| exponent     | {:01b}{}{:023b} |", 0, exponent_txt, 0);
    println!("| mantissa     | {:09b}{} |", 0, mantissa_txt);
    println!();
}
