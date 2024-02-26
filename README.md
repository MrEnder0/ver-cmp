# Ver-CMP

A useful cli-tool and library for compairing semantic versions

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

Returns the following

```bash
0.2.3
```


> [!TIP]
> You can use the -c or --compare flag to return a 0 1 or 2 for greater, less, or equal respectively this can be used to easily pipe the output to other commands

## Library

This example shows how to use the library to compare two versions and print out the comparison

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