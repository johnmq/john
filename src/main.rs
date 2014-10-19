extern crate john;

use std::os::getenv;
use std::io::net::ip::Port;

#[cfg(not(test))]
/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> Port {
    getenv("PORT")
        .and_then(|s| from_str::<Port>(s.as_slice()))
        .unwrap_or(3000)
}

#[cfg(not(test))]
fn main() {
    john::Server::new(get_server_port()).start()
}
