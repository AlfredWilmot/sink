#![allow(unused_variables)]
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

fn main() {
    let args = Args::parse();

    // get number from user input
    let float: f32 = args.number;

    //
    if args.debug {
        println!("{}", float);
    }
}
