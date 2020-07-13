extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

pub fn run(){
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://fin/employees/api/employees?show-inactive=false".parse().unwrap();
    let work =
        client.get(uri).and_then(|res| {

            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)
            })
        });
    core.run(work).unwrap();
}