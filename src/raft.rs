extern crate raft_rs;
extern crate http;
extern crate url;

use std::fmt::Show;
use std::io;

use self::raft_rs::intercommunication::{Intercommunication, Package, Endpoint};
use self::raft_rs::replication::{Committable, Receivable, Queriable, ReplicationLog};

use serialize::json;

use self::http::client::RequestWriter;
use self::url::Url;

use super::river::PeekResult;

/// Http implementations of raft_rs::intercommunication
pub struct HttpIntercommunication < T: Committable + Send + Show > {
    receiver: Receiver < Package < T > >,
    sender: Sender < Package < T > >,
    endpoint: Option < Sender < Package < T > > >,
}

impl HttpIntercommunication < JohnCommand > {
    /// Used to fetch endpoint Sender < Package < JohnCommand > > clone
    pub fn fetch_endpoint(&self) -> Sender < Package < JohnCommand > > {
        match self.endpoint {
            Some(ref tx) => tx.clone(),
            _ => panic!("[ERROR] HttpIntercommunication#fetch_endpoint :: Node is not registered yet")
        }
    }
}

impl Intercommunication < JohnCommand > for HttpIntercommunication < JohnCommand > {
    fn new() -> HttpIntercommunication < JohnCommand > {
        let (tx, rx) = channel();

        HttpIntercommunication {
            receiver: rx,
            sender: tx,
            endpoint: None,
        }
    }

    fn register(&mut self, host: String) -> Endpoint < JohnCommand > {
        let (tx, rx) = channel();

        self.endpoint = Some(tx.clone());

        Endpoint {
            host: host,
            rx: rx,
            tx: self.sender.clone(),
        }
    }

    fn receive(&mut self) -> Option < Package < JohnCommand > > {
        match self.receiver.try_recv() {
            Ok(package) => Some(package),
            _ => None,
        }
    }

    fn send(&mut self, recipient: String, package: Package < JohnCommand >) {
        let package_to_send = &package.clone();

        let url = format!("http://{}/raft/package", recipient);
        let payload = json::encode(package_to_send);

        println!("[INFO] Package encoded: {}", payload);

        let parsed_url = Url::parse(url.as_slice()).ok().expect("Invalid url");
        let mut req: RequestWriter = RequestWriter::new(http::method::Post, parsed_url).unwrap();

        let converted_payload = payload.as_slice().as_bytes();

        req.headers.content_length = Some(converted_payload.len());
        match req.write(converted_payload) {
            Ok(_) => match req.read_response() {
                Ok(_) => (),
                _ => (),
            },
            _ => (),
        }

        println!("[INFO] Sent {} to {}", package, recipient);
    }

    fn is_debug(&self) -> bool {
        true
    }
}

#[deriving(Show, Send, Clone, Encodable, Decodable)]
/// Represents john command
///
/// - Clear(river_name)
/// - Push(river_name, message)
/// - Peek(river_name, optional_offset)
pub enum JohnCommand {
    /// Clear command. Accepts river name as an argument
    Clear(String),

    /// Push command. Accepts river name and message as arguments
    Push(String, String),

    /// Peek command. Accepts river name and optional offset (i.e., Option < uint >)
    Peek(String, Option < uint >),
}

impl Committable for JohnCommand { }

#[deriving(Show, Send, Clone)]
/// Represents john response to any query
///
/// - ClearResponse(),
/// - PushResponse(),
/// - PeekResponse(peek_result),
pub enum JohnResponse {
    /// Response for clear command, empty
    ClearResponse,

    /// Response for push command, empty
    PushResponse,

    /// Response for peek command, contains PeekResult
    PeekResponse(PeekResult),
}

impl Receivable for JohnResponse { }

#[deriving(Show, Send, Clone)]
/// Represents any john query
///
/// - ClearQuery(river_name),
/// - PushQuery(river_name, message),
/// - PeekQuery(river_name, optional_offset),
pub enum JohnQuery {
    /// Clear query. Accepts river name as an argument
    ClearQuery(String),

    /// Push query. Accepts river name and message as arguments
    PushQuery(String, String),

    /// Peek query. Accepts river name and optional offset (i.e., Option < uint >)
    PeekQuery(String, Option < uint >),
}

impl Queriable for JohnQuery { }

//pub trait LogPersistence < T: Committable > {
//    fn commit(&self, entry: T) -> io::IoResult < () >;
//}

/// Representation of raft replication log in John
pub struct JohnReplicationLog;

#[allow(unused_variables)]
impl ReplicationLog < JohnCommand, JohnQuery, JohnResponse > for JohnReplicationLog {
    fn new() -> JohnReplicationLog {
        JohnReplicationLog
    }

    fn len(&self) -> uint {
        0
    }

    fn committed_offset(&self) -> uint {
        0
    }

    fn commit_upto(&mut self, new_committed_offset: uint) -> io::IoResult < () > {
        Ok(())
    }

    fn discard_downto(&mut self, new_len: uint) -> io::IoResult < () > {
        Ok(())
    }

    fn autocommit_if_safe(&mut self, majority_size: uint) {

    }

    fn enqueue(&mut self, entry: JohnCommand) -> io::IoResult < uint > {
        Ok(1)
    }

    fn persisted(&mut self, offset: uint, node: String) -> io::IoResult < uint > {
        Ok(1)
    }

    fn query_persistance(&mut self, query: JohnQuery, respond_to: Sender < JohnResponse >) {

    }
}
