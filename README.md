# Ver-CMP

A useful cli-tool and library for compairing semantic versions

## Cli Usage

```bash
ver-cmp --ver1 0.2.3 --ver2 0.2.1
```

### Output

```bash
0.2.3
```

## Library Usage

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