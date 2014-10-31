# John [![Build Status](https://travis-ci.org/johnmq/john.svg)](https://travis-ci.org/johnmq/john)

> High-performance, persistent, reliable and dumb-simple messaging queue in Rust.

Inspired by [Apache Kafka](http://kafka.apache.org/)

Queue is called River in johnmq.

## Disclaimer

This project is still under heavy development and lack key features and
therefore is not recommended for production usage. Missing features:

- Security (encryption between clients and server, between servers; and/or
  consumer key).
- Replication (work in progress here:
  [johnmq/raft-rs](https://github.com/johnmq/raft-rs)).

Contributions are highly welcome. And by the way, I'm looking for collaborators
to make this project production ready faster. Email me if you feel like: [waterlink000@gmail.com](mailto:waterlink000+johnmq@gmail.com)

## Usage

This project is powered by [cargo](http://doc.crates.io).

Clone & build the project:

```
git clone https://github.com/johnmq/john.git
cd john
cargo build
```

Optionally check that tests are passing (since your version of `rustc` could be
incompatible with current version of `john`):

```
./build.sh               # to run the same suite that is run on travis
cargo test -- --bench    # if you want to run benchmarks
```

Start john server:

```
./target/john            # you can specify port to run server on with PORT
                         # environment variable
```

On some OS it may not work resulting in weird errors about not being able to
find libraries. In that case it should be sufficient to run it like this:
`LD_LIBRARY_PATH=./target/deps DYLD_LIBRARY_PATH=./target/deps ./target/john`

Alternatively, you can just use cargo:

```
cargo run
```

Now lets play with curl:

```shell
# peek at en empty river
$ curl -v http://localhost:3000/peek/hello
> GET /peek/hello HTTP/1.1
> User-Agent: curl/7.30.0
> Host: localhost:3000
> Accept: */*
>
< HTTP/1.1 404 Not Found
< Content-Length: 0
< Content-Type: text/plain
<

# push to a river
$ curl -v -X POST http://localhost:3000/push/hello -d "hi, world"
> POST /push/hello HTTP/1.1
> User-Agent: curl/7.30.0
> Host: localhost:3000
> Accept: */*
> Content-Length: 12
> Content-Type: application/x-www-form-urlencoded
>
< HTTP/1.1 201 Created
< Content-Length: 0
< Content-Type: text/plain

# push to a river
$ curl -v -X POST http://localhost:3000/push/hello -d "hello, world"
> POST /push/hello HTTP/1.1
> User-Agent: curl/7.30.0
> Host: localhost:3000
> Accept: */*
> Content-Length: 12
> Content-Type: application/x-www-form-urlencoded
>
< HTTP/1.1 201 Created
< Content-Length: 0
< Content-Type: text/plain

# peek at non-empty river
$ curl -v http://localhost:3000/peek/hello
> GET /peek/hello HTTP/1.1
> User-Agent: curl/7.30.0
> Host: localhost:3000
> Accept: */*
>
< HTTP/1.1 200 OK
< Content-Length: 37
< Content-Type: text/plain
<
{"message":"hello, world","offset":3}

# Notice this `"offset": 3` it provides you with information enough to read the
# next message:
$ curl -v -X POST http://localhost:3000/push/hello -d "bye, world"
$ curl -v -X POST http://localhost:3000/push/hello -d "hi again, world"
$ curl -v http://localhost:3000/peek/hello/3       # notice this `peek/hello/3`
                                                   # - it is offset you want to
                                                   # read from
> GET /peek/hello/3 HTTP/1.1
> User-Agent: curl/7.30.0
> Host: localhost:3000
> Accept: */*
>
< HTTP/1.1 200 OK
< Content-Length: 35
< Content-Type: text/plain
<
{"message":"bye, world","offset":4}
```

When you are not specifying offset it reads the last message.  When you are
specifying offset it reads the message at this offset.  In both cases if there
is no message it returns `404 Not found`.

Client is responsible for managing his own offset. Server will just respond
with next offset for him.  That enables clients to read sequentially, re-read
some old messages, read from beginning (by specifying offset 0) or read
randomly (probably latter is not needed).

## Usage as library

`john` is build as library and only after that as a server. So you can install john in your regular Cargo project by adding this to your `Cargo.toml`:

```toml
[dependencies.john]

git = "https://github.com/johnmq/john.git"
```

### Pushing and Peeking a message

```rust
extern crate john;

use john::{PushCommand, PeekCommand, PeekResult, ClearCommand};

// ...

PushCommand::new().execute("a river", "hello world");
let result = PeekCommand::new().execute("a river", None);

match result {
    Some(PeekResult { message, offset }) => {
        assert_eq!("hello world", message.as_slice());
        assert_eq!(2, offset);
    },
    _ => fail!("Should have been Some(PeekResult)"),
}
```

### Peeking with offset

```
// second argument here is offset
PeekCommand::new().execute("a river 1", Some(4));
```

### Peeking with too big offset (> river size)

```
// it returns None when river is empty or river is smaller than requested offset
let result = PeekCommand::new().execute("a river 4", Some(10));    // => None
```

### Clearing a river

```
// After this command river becomes empty
ClearCommand::new().execute("a river 5");
```

### Further examples

You can find them in [tests/lib_test.rs](https://github.com/johnmq/john/blob/master/tests/lib_test.rs) - they are pretty straightforward.

## Contributing

1. Fork it https://github.com/johnmq/john/fork
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am "Add some feature")
4. Push to the branch (git push origin my-new-feature)
5. Create a new Pull Request
