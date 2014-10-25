# John [![Build Status](https://travis-ci.org/johnmq/john.svg)](https://travis-ci.org/johnmq/john)

> High-performance, persistent, reliable and dumb-simple messaging queue in Rust.

## Disclaimer

This project is still under heavy development and lack key features and
therefore is not recommended for production usage. Missing features:

- Security (encryption between clients and server, between servers; and/or
  consumer key).
- Replication (work in progress here:
  [johnmq/raft-rs](https://github.com/johnmq/raft-rs)).

Contributions are highly welcome. And by the way, I'm looking for collaborators
to make this project production ready faster.

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
./target/john
```

On some OS it may not work resulting in weird errors about not being able to
find libraries. In that case it should be sufficient to run it like this:
`LD_LIBRARY_PATH=./target/deps ./target/john`

