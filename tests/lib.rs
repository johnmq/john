extern crate john;

use john::{Queue, PushCommand, PopCommand, PeekCommand};

#[test]
fn test_push_and_pop() {
    let mut queue = Queue::new();

    let value = "hello world";

    PushCommand::execute(&mut queue, value.to_string());

    let popped = PopCommand::execute(&mut queue);

    match popped {
        Some(result) => assert_eq!(value, result.as_slice()),
        None => assert!(false)
    }
}

#[test]
fn test_multiple_pushes_and_pops() {
    let mut queue = Queue::new();

    let value_1 = "hello world";
    let value_2 = "bye bye world";

    PushCommand::execute(&mut queue, value_1.to_string());
    PushCommand::execute(&mut queue, value_2.to_string());

    let popped_1 = PopCommand::execute(&mut queue);
    let popped_2 = PopCommand::execute(&mut queue);

    match (popped_1, popped_2) {
        (Some(result_1), Some(result_2)) => {
            assert_eq!(value_1, result_2.as_slice());
            assert_eq!(value_2, result_1.as_slice());
        }
        _ => assert!(false)
    }
}

#[test]
fn test_peek() {
    let mut queue = Queue::new();

    let value = "hello world";

    PushCommand::execute(&mut queue, value.to_string());

    let peeked = PeekCommand::execute(&queue);
    let popped = PopCommand::execute(&mut queue);

    match (peeked, popped) {
        (Some(result_1), Some(result_2)) => {
            assert_eq!(value, result_1.as_slice());
            assert_eq!(value, result_2.as_slice());
        }
        _ => assert!(false)
    }
}
