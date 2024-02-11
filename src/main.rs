use clap::Parser;
use std::cmp::Ordering;
use ver_cmp::{greater_ver, compare_versions};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    ver1: String,

    #[arg(long)]
    ver2: String,

    // return 0 if ver1 is greater, 1 if ver2 is greater, 2 if equal
    #[arg(short, long, default_value_t = false)]
    compare: bool,
}

fn main() {
    let args = Args::parse();

    if args.compare {
        match compare_versions(&args.ver1, &args.ver2) {
            Ok(Ordering::Greater) => println!("0"),
            Ok(Ordering::Less) => println!("1"),
            Ok(Ordering::Equal) => println!("2"),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        match greater_ver(&args.ver1, &args.ver2) {
            Ok(ver) => println!("{}", ver),
            Err(e) => println!("{}", e),
        }
    }
}