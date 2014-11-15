extern crate john;
extern crate raft_rs;

use john::raft::{HttpIntercommunication, JohnCommand, JohnResponse, JohnQuery, JohnReplicationLog};

use raft_rs::intercommunication::{Intercommunication, start};
use raft_rs::replication::{ReplicationLog};
use raft_rs::node::{Node};

use std::{rand, num};
use std::time::duration::Duration;
use std::io::timer::sleep;

#[test]
fn run_one_raft_node() {
    let mut comm: HttpIntercommunication < JohnCommand > = Intercommunication::new();
    let mut node: Node < JohnCommand, JohnQuery, JohnResponse > = Node::new();
    let log: JohnReplicationLog = ReplicationLog::new();

    let election_timeout = 150 + num::abs(rand::random::< i64 >() % 150);

    node.start("localhost:3000", &mut comm, log, Duration::milliseconds(election_timeout));

    let sig = start(comm);

    sleep(Duration::milliseconds(350));

    sig.send(0);
}
