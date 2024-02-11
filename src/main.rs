use clap::Parser;
use ver_cmp::{greater_ver, is_ver_greater};

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
        match is_ver_greater(&args.ver1, &args.ver2) {
            Ok(true) => println!("0"),
            Ok(false) => println!("1"),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        match greater_ver(&args.ver1, &args.ver2) {
            Ok(ver) => println!("{}", ver),
            Err(e) => println!("{}", e),
        }
    }
}