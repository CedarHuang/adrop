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
