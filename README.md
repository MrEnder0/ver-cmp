# Ver-CMP

Ver-CMP is a useful cli-tool and library for comparing semantic versions

> [!NOTE]
> The current minimum supported Rust version is: 1.60.0 (Last checked on 3/7/2024)

## Cli App

### Cli Installation

To build the cli tool, run the following command:

```bash
cargo build --bin ver_cmp_cli --features build-binary --release
```

### Cli Usage

The cli tool can be used to compare two versions and print out the comparison in many different formats

* Basic Comparison

```bash
ver-cmp --ver1 0.2.3 --ver2 0.2.1 
```
Output: 0.2.3 (the greater version)

* Comparison with Flags

```bash
ver-cmp --ver1 0.2.3 --ver2 0.2.1 -c  
```
Output: 0 (indicates  ver1 > ver2)

> [!TIP]
> The -c or --compare flag to return a 0 1 or 2 meaning greater, less, or equal respectively; this can be used to easily pipe the output to other commands

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

You can also take a look in the [tests](src/test.rs) for more examples of how to use the library