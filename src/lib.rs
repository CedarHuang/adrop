/*!
Simple and fast dedicated thread drop.

# Example

```rust
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

println!("Main ThreadId: {:?}", std::thread::current().id());
adrop(Test {});
// Output:
// Main ThreadId: ThreadId(1)
// Dropping HasDrop! ThreadId: ThreadId(2)

// Or you can use `Adrop` wrapper to realize automatic `adrop`:
let _ = Adrop::new(Test {});
```
*/

use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{channel, Sender},
        Mutex, Once,
    },
    thread::spawn,
};

type Trash = Box<dyn Send>;
type TrashSender = Sender<Trash>;

/// Pass the value to a dedicated thread for destruction.
///
/// # Examples
///
/// ```rust
/// use adrop::adrop;
///
/// struct Test {}
///
/// impl Drop for Test {
///     fn drop(&mut self) {
///         println!(
///             "Dropping HasDrop! ThreadId: {:?}",
///             std::thread::current().id()
///         );
///     }
/// }
///
/// println!("Main ThreadId: {:?}", std::thread::current().id());
/// adrop(Test {});
/// ```
///
/// Output:
///
/// ```text
/// Main ThreadId: ThreadId(1)
/// Dropping HasDrop! ThreadId: ThreadId(2)
/// ```
pub fn adrop<T: Send + 'static>(trash: T) {
    static mut TX: Option<Mutex<TrashSender>> = None;
    static TX_SET: Once = Once::new();
    TX_SET.call_once(|| {
        let (tx, rx) = channel();
        spawn(move || loop {
            let _ = rx.recv();
        });
        unsafe {
            TX = Some(Mutex::new(tx));
        }
    });
    unsafe {
        let _ = TX.as_ref().unwrap().lock().unwrap().send(Box::new(trash));
    }
}

/// `Adrop` wrapper can realize automatic `adrop`.
///
/// # Examples
///
/// ```rust
/// use adrop::Adrop;
///
/// struct Test {}
///
/// impl Drop for Test {
///     fn drop(&mut self) {
///         println!(
///             "Dropping HasDrop! ThreadId: {:?}",
///             std::thread::current().id()
///         );
///     }
/// }
///
/// println!("Main ThreadId: {:?}", std::thread::current().id());
/// let _ = Adrop::new(Test {});
/// ```
///
/// Output:
///
/// ```text
/// Main ThreadId: ThreadId(1)
/// Dropping HasDrop! ThreadId: ThreadId(2)
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adrop<T: Send + 'static> {
    content: Option<ManuallyDrop<T>>,
}

impl<T: Send> Adrop<T> {
    /// Wrap a value to be realize automatic `adrop`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use adrop::Adrop;
    /// Adrop::new(String::from("Hello World!"));
    /// ```
    pub fn new(content: T) -> Adrop<T> {
        Adrop::<T> {
            content: Some(ManuallyDrop::new(content)),
        }
    }

    /// Extracts the value from the `Adrop` container.  
    /// This allows the value to be dropped again.
    /// # Examples
    ///
    /// ```rust
    /// use adrop::Adrop;
    /// let s = Adrop::new(String::from("Hello World!"));
    /// let _ = s.into_inner();
    /// ```
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
