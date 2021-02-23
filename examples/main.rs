extern crate adrop;

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
}
