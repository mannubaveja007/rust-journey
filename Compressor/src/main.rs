// First take 2 number from argument and then add sum of them simp shit to move forward in order to learn  args and it's working
//
use clap::Parser;

/// Simple program to Take input of two number to perform some operation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number 1
    #[arg(short = '1', long)]
    num1: u8,

    /// Number 2
    #[arg(short = '2', long)]
    num2: u8,
}

fn main() {
    let args = Args::parse();

    let result = args.num1 + args.num2;
    println!("Your Result : {result}");
}
