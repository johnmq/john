extern crate john;

use john::{PushCommand, PeekCommand, PeekResult};

#[test]
fn pushing_and_peeking_a_message() {
    PushCommand::new().execute("a river", "hello world");
    let result = PeekCommand::new().execute("a river", None);
    match result {
        PeekResult { message, offset } => {
            assert_eq!("hello world", message.as_slice());
            assert_eq!(1, offset);
        }
    }
}

#[test]
fn peeking_a_message_without_offset() {
    PushCommand::new().execute("a river", "message 1");
    PushCommand::new().execute("a river", "message 2");
    let result = PeekCommand::new().execute("a river", None);
    match result {
        PeekResult { message, offset } => {
            assert_eq!("message 2", message.as_slice());
            assert_eq!(2, offset);
        }
    }
}
