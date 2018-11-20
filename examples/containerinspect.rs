extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi};
use futures::{future, Future};
use std::env;

fn main() {
    let id = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        docker
            .container(id.into())
            .inspect()
            .then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}
