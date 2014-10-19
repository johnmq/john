extern crate john;

#[cfg(not(test))]
fn main() {
    john::Server::new(3100).start()
}
