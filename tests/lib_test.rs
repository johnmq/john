extern crate john;
extern crate test;

use john::{PushCommand, PeekCommand, PeekResult, ClearCommand};

#[test]
fn pushing_and_peeking_a_message() {
    ClearCommand::new().execute("a river");
    PushCommand::new().execute("a river", "hello world");

    let result = PeekCommand::new().execute("a river", None);

    match result {
        Some(PeekResult { message, offset }) => {
            assert_eq!("hello world", message.as_slice());
            assert_eq!(2, offset);
        },
        _ => assert!(false)
    }
}

#[test]
fn peeking_a_message_without_offset() {
    ClearCommand::new().execute("a river 1");
    PushCommand::new().execute("a river 1", "message 1");
    PushCommand::new().execute("a river 1", "message 2");

    let result = PeekCommand::new().execute("a river 1", None);

    match result {
        Some(PeekResult { message, offset }) => {
            assert_eq!("message 2", message.as_slice());
            assert_eq!(3, offset);
        },
        _ => assert!(false)
    }
}

#[test]
fn clearing_river() {
    PushCommand::new().execute("a river 2", "hello world");
    ClearCommand::new().execute("a river 2");

    match PeekCommand::new().execute("a river 2", None) {
        Some(_) => assert!(false),
        None => assert!(true)
    }
}

#[test]
fn peeking_with_offset() {
    ClearCommand::new().execute("a river 3");
    PushCommand::new().execute("a river 3", "message 1");
    PushCommand::new().execute("a river 3", "message 2");
    PushCommand::new().execute("a river 3", "message 3");

    let result = PeekCommand::new().execute("a river 3", Some(2));

    match result {
        Some(PeekResult { message, offset }) => {
            assert_eq!("message 2", message.as_slice());
            assert_eq!(3, offset);
        },
        _ => assert!(false)
    }
}

#[test]
fn peeking_with_too_big_offset() {
    ClearCommand::new().execute("a river 4");
    PushCommand::new().execute("a river 4", "message 1");
    PushCommand::new().execute("a river 4", "message 2");
    PushCommand::new().execute("a river 4", "message 3");

    let result = PeekCommand::new().execute("a river 4", Some(10));

    match result {
        Some(_) => assert!(false),
        None => assert!(true)
    }
}

#[test]
fn clearing_empty_river() {
    ClearCommand::new().execute("a river 5");
    ClearCommand::new().execute("a river 5");

    match PeekCommand::new().execute("a river 5", None) {
        Some(_) => assert!(false),
        None => assert!(true)
    }
}

#[bench]
fn clearing_an_empty_river(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    b.iter(|| {
        clear.execute("an empty river")
    })
}

#[bench]
fn clearing_an_river_with_some_messages(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    let push = john::PushCommand::new();

    clear.execute("a river with some messages");
    for _ in range(0i, 3000i) {
        push.execute("a river with some messages", "a huge message");
    }

    b.iter(|| {
        clear.execute("an empty river")
    })
}

#[bench]
fn peeking_last_message_from_empty_river(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    let peek = john::PeekCommand::new();

    clear.execute("another empty river");
    peek.execute("another empty river", None); // to ensure file is there

    b.iter(|| {
        peek.execute("another empty river", None);
    })
}

#[bench]
fn peeking_last_message_from_river_with_some_messages(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    let push = john::PushCommand::new();
    let peek = john::PeekCommand::new();

    clear.execute("another river with some messages");
    for _ in range(0i, 3000i) {
        push.execute("another river with some messages", "a huge message");
    }

    b.iter(|| {
        peek.execute("another river with some messages", None);
    })
}

#[bench]
fn peeking_some_message_from_empty_river(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    let peek = john::PeekCommand::new();

    clear.execute("another empty river v2");
    peek.execute("another empty river v2", None); // to ensure file is there

    b.iter(|| {
        peek.execute("another empty river v2", Some(10));
    })
}

#[bench]
fn peeking_some_message_from_river_with_some_messages(b: &mut test::Bencher) {
    let clear = john::ClearCommand::new();
    let push = john::PushCommand::new();
    let peek = john::PeekCommand::new();

    clear.execute("another river with some messages v2");
    for _ in range(0i, 3000i) {
        push.execute("another river with some messages v2", "a huge message");
    }

    b.iter(|| {
        peek.execute("another river with some messages v2", Some(10));
    })
}

#[bench]
fn continuous_push_to_river(b: &mut test::Bencher) {
    let push = john::PushCommand::new();

    b.iter(|| {
        push.execute("river for continuous push", "a huge message");
    })
}

#[bench]
fn continuous_push_to_river_teardown(b: &mut test::Bencher) {
    b.iter(|| {
        john::ClearCommand::new().execute("river for continuous push");
    })
}
