extern crate http;
extern crate iron;
extern crate router;
extern crate raft_rs;

use std::io::net::ip::{Ipv4Addr, Port};
use std::str;

use std::sync::{Arc, Mutex};

use self::router::{Router, Params};
use self::iron::{Iron, Request, Response, IronResult, IronError};
use self::iron::status;
use self::iron::middleware::Handler;

use self::raft_rs::node::{Node};
use self::raft_rs::intercommunication::{Package};

use serialize::json;

use commands::{PeekCommand, PushCommand};

use raft::{JohnCommand, JohnResponse, JohnQuery};

/// Http Server to make pushes, peeks and clears
pub struct Server {
    port: Port,
    node: Box < Arc < Mutex < Node < JohnCommand, JohnQuery, JohnResponse > > > >,
    endpoint: Box < Arc < Mutex < Sender < Package < JohnCommand > > > > >,
}

impl Server {
    /// Creates new instance of server
    pub fn new(
        port: Port, node: Node < JohnCommand, JohnQuery, JohnResponse >,
        endpoint: Sender < Package < JohnCommand > >
        ) -> Server {
            Server {
                port: port,
                node: box Arc::new(Mutex::new(node)),
                endpoint: box Arc::new(Mutex::new(endpoint))
            }
        }

    /// Starts listening server on specified port
    pub fn start(&mut self) {
        let mut router = Router::new();

        router.get("/hello/:name", Server::hello);
        router.get("/peek/:river", Server::peek);
        router.get("/peek/:river/:offset", Server::peek);
        router.post("/push/:river", Server::push);

        router.get("/raft/leader", RaftLeaderHandler::new(self.node.clone()));
        router.post("/raft/package", RaftPackageHandler::new(
                self.node.clone(),
                self.endpoint.clone(),
                ));


        Iron::new(router).listen(Ipv4Addr(0, 0, 0, 0), self.port);
    }

    fn hello(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.find::< Router, Params >().unwrap();
        let name = params.find("name").unwrap();

        Ok(Response::with(status::Ok, format!("Hello, {}!", name)))
    }

    fn peek(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.find::< Router, Params >().unwrap();
        let river = params.find("river").unwrap();
        let offset = from_str::< uint >(params.find("offset").unwrap_or(""));

        match PeekCommand::new().execute(river, offset) {
            Some(result) => Ok(Response::with(
                    status::Ok,
                    json::encode(&result)
                    )),
            _ => Ok(Response::with(status::NotFound, ""))
        }
    }

    fn push(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.find::< Router, Params >().unwrap();
        let river = params.find("river").unwrap();
        let message = str::from_utf8(req.body.as_slice());

        match message {
            Some(message) => {
                PushCommand::new().execute(river, message);
                Ok(Response::with(status::Created, ""))
            },
            None => Ok(Response::with(status::BadRequest, "unable to parse response body as utf8"))
        }

    }
}

/// /raft/leader handler
struct RaftLeaderHandler {
    mutexed_node: Box < Arc < Mutex < Node < JohnCommand, JohnQuery, JohnResponse > > > >,
}

impl RaftLeaderHandler {
    fn new(node: Box < Arc < Mutex < Node < JohnCommand, JohnQuery, JohnResponse > > > >) -> RaftLeaderHandler {
        RaftLeaderHandler {
            mutexed_node: node,
        }
    }
}

impl Handler for RaftLeaderHandler {
    fn call(&self, _: &mut Request) -> IronResult < Response > {
        let node = self.mutexed_node.lock();
        let leader_host = node.fetch_leader();
        node.cond.signal();

        match leader_host {
            Some(host) => Ok(Response::with(status::Ok, host.host)),
            _ => Ok(Response::with(status::NotFound, "No leader elected")),
        }
    }

    fn catch(&self, _: &mut Request, err: IronError) -> (Response, IronResult<()>) {
        (Response::with(status::InternalServerError, format!("Raft Error: {}", err)), Err(err))
    }
}

#[allow(dead_code)]
/// /raft/package handler
struct RaftPackageHandler {
    mutexed_node: Box < Arc < Mutex < Node < JohnCommand, JohnQuery, JohnResponse > > > >,
    mutexed_endpoint: Box < Arc < Mutex < Sender < Package < JohnCommand > > > > >,
}

impl RaftPackageHandler {
    fn new(
        node: Box < Arc < Mutex < Node < JohnCommand, JohnQuery, JohnResponse > > > >,
        endpoint: Box < Arc < Mutex < Sender < Package < JohnCommand > > > > >
        ) -> RaftPackageHandler {
            RaftPackageHandler {
                mutexed_node: node,
                mutexed_endpoint: endpoint,
            }
        }
}

impl Handler for RaftPackageHandler {
    fn call(&self, req: &mut Request) -> IronResult < Response > {
        println!("Got here!");

        let package: Package < JohnCommand > = json::decode(
            str::from_utf8(
                req.body.as_slice())
            .unwrap())
            .ok()
            .expect("Invalid json payload");

        println!("Got package: {}", package);

        let endpoint = self.mutexed_endpoint.lock();

        endpoint.send(package.clone());

        endpoint.cond.signal();

        Ok(Response::with(status::Ok, "dummy"))
    }

    fn catch(&self, _: &mut Request, err: IronError) -> (Response, IronResult<()>) {
        (Response::with(status::InternalServerError, format!("Raft Error: {}", err)), Err(err))
    }
}
