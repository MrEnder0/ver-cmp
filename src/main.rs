use clap::Parser;
use std::cmp::Ordering;
use ver_cmp::{compare_versions, greater_ver};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    ver1: String,
    #[arg(long)]
    ver2: String,
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
            Err(e) => println!("Error: {}", e),
        }
    }
}
