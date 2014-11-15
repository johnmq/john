#![crate_name = "john"]
#![comment = "Dead Simple Messaging Queue in Rust"]
#![license = "MIT"]

#![deny(missing_docs)]
#![deny(warnings)]

//! The main crate of the John Messaging Queue.
//!
//! John is a dead simple messaging queue for Rust.
//!
//! It can be used as a standalone library or as a messaging queue server.

extern crate serialize;

pub use server::Server;
pub use commands::{ClearCommand, PushCommand, PeekCommand, PeekResult};

mod river;
mod server;
mod commands;

/// Naive implementation of raft_rs exposed traits
pub mod raft;

