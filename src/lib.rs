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

use std::io;

/// Push command - stateless
///
/// Used to push messages to rivers like this:
///
/// ```
/// john::PushCommand::new().execute("river_name", "message");
/// ```
pub struct PushCommand;

impl PushCommand {
    /// Constructor ::new()
    ///
    /// Creates new instance of PushCommand
    pub fn new() -> PushCommand {
        PushCommand
    }

    /// Used to execute push command, specifying a river name and message
    /// This can be called multiple times with different arguments
    /// since PushCommand is stateless
    pub fn execute(&self, river: &str, message: &str) {
        println!("pushing message '{}' to river '{}'", river, message);
        let path = Path::new(format!("./tmp/rivers/{}", river));
        io::File::open_mode(&path, io::Append, io::Write).write_line(message);
    }
}

/// Result of PeekCommand, when it was successful
/// Contains message and new offset to specify to peek command
/// for continuous peeking
pub struct PeekResult {
    /// Contains message
    pub message: String,
    /// Contains next offset to be specified to read next message from river
    pub offset: uint
}

/// Peek command - stateless
///
/// Used to peek messages from rivers like this:
///
/// ```
/// // read latest message from river
/// john::PeekCommand::new().execute("river name", None);
/// // read message from river at specific offset
/// john::PeekCommand::new().execute("river name", Some(7));
/// ```
///
/// It returns Option < PeekResult >. When it was able to peek a message, the result will contain
/// peeked message and new offset to specify to peek command (if you want to get next message)
pub struct PeekCommand;

impl PeekCommand {
    /// Constructor ::new()
    ///
    /// Creates new instance of PeekCommand
    pub fn new() -> PeekCommand {
        PeekCommand
    }

    /// Used to execute peek command, specifying a river name and optionally offset to peek at
    pub fn execute(&self, river: &str, offset: Option < uint >) -> PeekResult {
        println!("peeking in river {}", river);
        let path = Path::new(format!("./tmp/rivers/{}", river));
        let file = io::BufferedReader::new(io::File::open(&path));
        match offset {
            Some(offset) => PeekResult { message: file.lines()[offset].to_string(), offset: offset + 1 },
            None => PeekResult { message: "hello world".to_string(), offset: 0 }
        }
    }

    fn read_by_offset(file: io::BufferedReader < io::File >, offset: uint) -> Option < String > {
        match file.lines().take(offset).last() {
            Ok(message) => Some(message.unwrap()),
            Err => None
        }
    }
}
