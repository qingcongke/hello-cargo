extern crate hyper;

use hyper::rt::{self, Future};
use hyper::Client;

fn main() {
    rt::run(rt::lazy(|| {
        let client = Client::new();

        let uri = "http://www.baidu.com".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response:{}", res.status());
            })
            .map_err(|err| println!("Error:{}", err))
    }))
}
