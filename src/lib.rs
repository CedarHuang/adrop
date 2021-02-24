#[macro_use]
extern crate lazy_static;

use std::{
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread::spawn,
};

type Trash = Box<dyn Send>;
type TrashSender = Sender<Trash>;

fn run() -> TrashSender {
    let (tx, rx) = channel();
    spawn(move || {
        while let Ok(trash) = rx.recv() {
            drop(trash);
        }
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
    let _ = TX
        .lock()
        .expect("Theoretically will not fail")
        .send(Box::new(trash));
}
