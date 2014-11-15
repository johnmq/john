extern crate john;
extern crate raft_rs;

use john::raft::{HttpIntercommunication, JohnCommand, JohnResponse, JohnQuery, JohnReplicationLog};

use raft_rs::intercommunication::{Intercommunication};
use raft_rs::replication::{ReplicationLog};
use raft_rs::node::{Node};

use std::{rand, num};
use std::time::duration::Duration;

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
/// Look up our server raft host name.
fn get_raft_host() -> String {
    getenv("RAFT_HOST")
        .unwrap_or(format!("localhost:{}", get_server_port()))
}

#[cfg(not(test))]
/// Look up our server raft introduction.
fn get_raft_introduction() -> Option < String > {
    getenv("RAFT_INTRODUCE")
}

#[cfg(not(test))]
fn main() {
    let mut comm: HttpIntercommunication < JohnCommand > = Intercommunication::new();
    let mut node: Node < JohnCommand, JohnQuery, JohnResponse > = Node::new();
    let log: JohnReplicationLog = ReplicationLog::new();

    let election_timeout = 150 + num::abs(rand::random::< i64 >() % 150);

    node.start(get_raft_host().as_slice(), &mut comm, log, Duration::milliseconds(election_timeout));

    raft_rs::intercommunication::start(comm);

    match get_raft_introduction() {
        Some(introduction) => node.introduce(introduction.as_slice()),
        _ => (),
    }

    john::Server::new(get_server_port(), node).start();
}
