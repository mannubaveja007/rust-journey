// First take 2 number from argument and then add sum of them simp shit to move forward in order to learn  args and it's working
//
// use clap::Parser;

// /// Simple program to Take input of two number to perform some operation
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Number 1
//     #[arg(short = '1', long)]
//     num1: u8,

//     /// Number 2
//     #[arg(short = '2', long)]
//     num2: u8,
// }

// fn main() {
//     let args = Args::parse();

//     let result = args.num1 + args.num2;
//     println!("Your Result : {result}");
// }

// input an file to compress and --out flag to output the file which is compreseed
//
// example: ./compressor --in <file> --out <output_file>

use clap::Parser as _;
use clap_file::{Input, Output};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::{self, Write};
use std::time::Instant;
/// Simple program to Take input an file to compress and output an compressed file
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input of File
    #[arg(short = 'f', long)]
    file: Input,

    /// Output of File
    #[arg(short = 'o', long)]
    out: Output,

    /// Decompress mode
    #[arg(short = 'd', long)]
    decompress: bool,
}

fn decompress_file(input: &mut impl io::Read, output: &mut impl Write) -> io::Result<u64> {
    let mut decoder = flate2::read::GzDecoder::new(input);
    let bytes_out = io::copy(&mut decoder, output)?;
    Ok(bytes_out)
}
fn compress_file(input: &mut impl io::Read, output: &mut impl Write) -> io::Result<u64> {
    let mut encoder = GzEncoder::new(output, Compression::default());
    let bytes_in = io::copy(input, &mut encoder)?;
    encoder.finish()?;
    Ok(bytes_in)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut input = args.file.lock();
    let mut output = args.out.lock();
    // let result = args.num1 + args.num2;
    // println!("Your Result : {result}");
    // let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    // let bytes_in = io::copy(&mut input, &mut encoder)?;
    // let output = encoder.finish().unwrap();
    // println!("Source bytes: {}", bytes_in);
    // println!("Elapsed : {:?}", start.elapsed());

    if args.decompress {
        let bytes_out = decompress_file(&mut input, &mut output)?;
        println!("Decompressed: {} bytes", bytes_out);
    } else {
        let bytes_in = compress_file(&mut input, &mut output)?;
        println!("Original    : {} bytes", bytes_in);
    }

    println!("Elapsed     : {:?}", start.elapsed());

    Ok(())
}

//  fn main() -> io::Result<()> {
//      let args = Args::parse();
//      let mut input = args.file.lock();
//      let mut output = args.out.lock();
// -    // let result = args.num1 + args.num2;
// -    // println!("Your Result : {result}");
// -    let mut encoder = GzEncoder::new(output, Compression::default());
//      let start = Instant::now();
// -    let bytes_in = io::copy(&mut input, &mut encoder)?;
// -    let output = encoder.finish().unwrap();
// -    println!("Source bytes: {}", bytes_in);
// -    println!("Elapsed : {:?}", start.elapsed());
// +
