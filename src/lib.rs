#[macro_use]
extern crate lazy_static;

use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
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

pub fn adrop<T: Send + 'static>(trash: T) {
    let _ = TX
        .lock()
        .expect("Theoretically will not fail")
        .send(Box::new(trash));
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adrop<T: Send + 'static> {
    content: Option<ManuallyDrop<T>>,
}

impl<T: Send> Adrop<T> {
    pub fn new(content: T) -> Adrop<T> {
        Adrop::<T> {
            content: Some(ManuallyDrop::new(content)),
        }
    }

    pub fn into_inner(mut self) -> T {
        let content = unsafe { ManuallyDrop::take(self.content.as_mut().unwrap()) };
        self.content = None;
        content
    }
}

impl<T: Send> Deref for Adrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.content.as_ref().unwrap()
    }
}

impl<T: Send> DerefMut for Adrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.content.as_mut().unwrap()
    }
}

impl<T: Send> Drop for Adrop<T> {
    fn drop(&mut self) {
        if let Some(content) = self.content.as_mut() {
            unsafe {
                adrop(ManuallyDrop::take(content));
            }
        }
    }
}
