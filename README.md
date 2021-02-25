# adrop

Simple and fast dedicated thread drop.

[![Latest version](https://img.shields.io/crates/v/adrop.svg)](https://crates.io/crates/adrop)
[![Documentation](https://docs.rs/adrop/badge.svg)](https://docs.rs/adrop)
[![License](https://img.shields.io/crates/l/adrop.svg)](https://github.com/CedarHuang/adrop#license)

## Getting Started

Add the following dependency to your Cargo manifest...

```toml
[dependencies]
adrop = "0.2"
```

## Example

```rust
extern crate adrop;

use adrop::*;

struct Test {}

impl Drop for Test {
    fn drop(&mut self) {
        println!(
            "Dropping HasDrop! ThreadId: {:?}",
            std::thread::current().id()
        );
    }
}

fn main() {
    println!("Main ThreadId: {:?}", std::thread::current().id());
    adrop(Test {});
    // Output:
    // Main ThreadId: ThreadId(1)
    // Dropping HasDrop! ThreadId: ThreadId(2)
}
```

Or you can use `Adrop` wrapper to realize automatic `adrop`:

```rust
let _ = Adrop::new(Test {});
```

## License

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
