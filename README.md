# Ver-CMP

Ver-CMP is a useful cli-tool and library for comparing semantic versions

## Cli App

### Cli Installation

To build the cli tool, run the following command:

```bash
cargo build --bin ver_cmp_cli --features build-binary --release
```

### Cli Usage

```bash
ver-cmp --ver1 0.2.3 --ver2 0.2.1
```

This script returns the following

```bash
0.2.3
```

> [!TIP]
> Wanted to pipe this into another command or use it in a script‽ well I got you; use the -c or --compare flag to return a 0 1 or 2 for greater, less, or equal too respectively

## Library

This example shows how to use the library to compare two versions and how to handle the result

```rust
use ver_cmp::*;

fn main() {
    let ver1 = "1.1.5";
    let ver2 = "1.0.3";
    let result = compare_versions(ver1, ver2);

    match result {
        Ok(Ordering::Greater) => println!("{} > {}", ver1, ver2),
        Ok(Ordering::Less) => println!("{} < {}", ver1, ver2),
        Ok(Ordering::Equal) => println!("{} == {}", ver1, ver2),
        Err(e) => println!("Error: {}", e),
    }
}
```

You can also look in the [tests](src/test.rs) for more examples of how to use the library