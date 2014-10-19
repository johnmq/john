extern crate http;
extern crate iron;
extern crate router;

use std::io::net::ip::{Ipv4Addr, Port};

use self::router::{Router, Params};
use self::iron::{Iron, Request, Response, IronResult};
use self::iron::status;

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

        Iron::new(router).listen(Ipv4Addr(0, 0, 0, 0), self.port);
    }

    fn hello(req: &mut Request) -> IronResult < Response > {
        let params = req.extensions.find::< Router, Params >().unwrap();
        let name = params.find("name").unwrap();
        Ok(Response::with(status::Ok, format!("Hello, {}!", name)))
    }
}
