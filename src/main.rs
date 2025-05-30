#![allow(unused_variables, dead_code)]

use colored::Colorize;
use std::{f32, process::exit};

use clap::Parser;

use sink::float::DeconstructedFloat32;

/// Coerce floating-points into fixed-point numbers.
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Args {
    /// specify a floating point number
    number: f32,
}

fn main() {
    let args = Args::parse();

    // get number from user input
    let float: f32 = args.number;

    // is the number within the allowed range?
    if (f32::MIN..=f32::MAX).contains(&float) {
        DeconstructedFloat32::new(&float).print();
        exit(0);
    }

    println!(
        "{}",
        format!("Must be within range: [{:?}, {:?}]", f32::MIN, f32::MAX).red(),
    );
    exit(1);
}
