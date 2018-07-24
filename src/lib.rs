//! Shiplift is a multi-transport utility for maneuvering [docker](https://www.docker.com/) containers
//!
//! # examples
//!
//! ```no_run
//! extern crate shiplift;
//!
//! let docker = shiplift::Docker::new();
//! let images = docker.images().list(&Default::default()).unwrap();
//! println!("docker images in stock");
//! for i in images {
//!   println!("{:?}", i.RepoTags);
//! }
//! ```

#![recursion_limit = "256"]

// Optional ssl
// TODO: maybe there is a way to put this in another file
#[cfg(feature = "ssl")]
extern crate hyper_openssl;
#[cfg(feature = "ssl")]
extern crate openssl;

#[cfg(target_os = "linux")]
extern crate tokio_uds;
#[cfg(target_os = "linux")]
extern crate unix_socket;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;


extern crate byteorder;
extern crate flate2;
extern crate rustc_serialize;
extern crate tar;
extern crate url;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate http;
extern crate tokio;
extern crate futures;

pub mod builder;
pub mod errors;
pub mod reader;
pub mod rep;
mod tarball;
pub mod transport;
pub mod tty;
pub mod structs;

pub use errors::Error;
pub use errors::Result;

pub use structs::*;
pub use builder::*;