extern crate http;
extern crate iron;
extern crate router;
extern crate bodyparser;

use std::io::net::ip::{Ipv4Addr, Port};

use self::router::{Router, Params};
use self::iron::{Iron, Request, Response, IronResult};
use self::iron::status;

use serialize::json;

use commands::{PeekCommand, PushCommand};

/// Http Server to make pushes, peeks and clears
pub struct Server {
    port: Port,
}

impl Server {
    /// Creates new instance of server
    pub fn new(port: Port) -> Server {
        Server {
            port: port,
        }
    }

    /// Starts listening server on specified port
    pub fn start(&mut self) {
        let mut router = Router::new();

        router.get("/hello/:name", Server::hello);
        router.get("/peek/:river", Server::peek);
        router.get("/peek/:river/:offset", Server::peek);
        router.post("/push/:river", Server::push);

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
        let message = req.body.as_slice();

        PushCommand::new().execute(river, message);

        Ok(Response::with(status::Created, ""))
    }
}
