# adrop

Simple and fast multi-threaded drop.

## Getting Started

Add the following dependency to your Cargo manifest...
```toml
[dependencies]
adrop = "0.1"
```

## Example

```rust
use adrop::*;
use std::thread;

struct Test {}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping HasDrop! ThreadId: {:?}", thread::current().id());
    }
}

fn main() {
    println!("Main ThreadId: {:?}", thread::current().id());
    adrop(Test {});
    // 
}
```

## License
- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
