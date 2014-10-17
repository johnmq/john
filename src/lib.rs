#![crate_name = "john"]
#![comment = "Dead Simple Messaging Queue in Rust"]
#![license = "MIT"]

#![deny(missing_doc)]
#![deny(warnings)]

//! The main crate of the John Messaging Queue.
//!
//! John is a dead simple messaging queue for Rust.
//!
//! It can be used as a standalone library or as a messaging queue server.

/// Struct that represents queue
pub struct Queue {
    data: Vec<String>
}

impl Queue {
    /// constructor
    ///
    /// Creates empty queue
    pub fn new() -> Queue {
        Queue { data: Vec::new() }
    }
}

/// Allows you to push messages to the queue
pub struct PushCommand(());

impl PushCommand {
    /// command executor
    ///
    /// Pushes a `message` to `queue`
    pub fn execute(queue: &mut Queue, message: String) {
        queue.data.push(message);
    }
}

/// Allows you to pop messages from the queue
pub struct PopCommand(());

impl PopCommand {
    /// command executor
    ///
    /// Pushes a `message` to `queue`
    pub fn execute(queue: &mut Queue) -> Option<String> {
        queue.data.pop()
    }
}

/// Allows you to peek at message at the top of the queue without popping it
pub struct PeekCommand(());

impl PeekCommand {
    /// command executor
    ///
    /// Pushes a `message` to `queue`
    pub fn execute(queue: &Queue) -> Option<String> {
        let res = queue.data.iter().next();
        match res {
            Some(result) => Some(result.clone()),
            _ => None
        }
    }
}
