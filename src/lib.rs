#[macro_use]
extern crate lazy_static;

use std::sync::{
    mpsc::{channel, Sender},
    Mutex,
};

type Trash = Box<dyn Send>;
type TrashSender = Sender<Trash>;

fn run() -> TrashSender {
    let (tx, rx) = channel();
    let _ = std::thread::spawn(move || {
        while let Ok(trash) = rx.recv() {
            drop(trash);
        }
        panic!("trash thread is dead.");
    });
    tx
}

lazy_static! {
    static ref TX: Mutex<TrashSender> = Mutex::new(run());
}

pub fn adrop<T>(trash: T)
where
    T: Send + 'static,
{
    let _ = TX.lock().unwrap().send(Box::new(trash));
}
