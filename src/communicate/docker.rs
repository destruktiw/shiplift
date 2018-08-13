use hyper::client::connect::Connect;
use hyper::Client;
use futures::Future;
use futures::future;
use std::fmt::Display;
use hyper::Body;
use hyper::rt::Stream;

use errors::{Result, Error, ErrorKind};

use transport::status_code;
use transport::compose_uri;
use transport::parse_to_trait;
use transport::parse_to_stream;
use communicate::util::AsSlice;

use std::env;

use build::{
    EventsOptions,
};

use representation::rep::{
    Change, Container as ContainerRep, ContainerCreateInfo, ContainerDetails, Exit, History,
    Image as ImageRep, ImageDetails, Info, NetworkCreateInfo, NetworkDetails as NetworkInfo,
    SearchResult, Status, Top, Version, Stats, Event
};

use hyper::Uri;
use hyper::StatusCode;
use hyper::Method;
use transport::parse::ResponseFutureWrapper;
use transport::interact::InteractApi;
use transport::interact::InteractApiExt;
use std::sync::Arc;
use std::borrow::Cow;
use errors::ErrorKind::InvalidUri;
use super::tcp_docker::TcpDocker;
#[cfg(target_os = "linux")]
use super::unix_docker::UnixDocker;
#[cfg(feature = "ssl")]
use super::ssl_tcp_docker::TcpSSLDocker;
use std::marker::PhantomData;
use communicate::Container;
use communicate::image::Image;
use communicate::Images;


/// Entry point interface for communicating with docker daemon
pub trait DockerApi
{
    /// Returns version information associated with the docker daemon
    fn version(&self) -> Box<Future<Item=Version, Error=Error> + Send>;

    /// Returns information associated with the docker daemon
    fn info(&self) -> Box<Future<Item=Info, Error=Error> + Send>;

    /// Returns a simple ping response indicating the docker daemon is accessible
    fn ping(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send>;

    /// Returns an iterator over streamed docker events
    fn events(&self, opts: &EventsOptions) -> Box<Stream<Item=Result<Event>, Error=Error> + Send>;

    /// Exports an interface for interacting with docker containers
    fn container(&self, id: Cow<'static, str>) -> Container;

    /// Exports an interface for interacting with docker image
    fn image<'a>(&self, id: Cow<'a, str>) -> Image<'a>;

    /// Exports an interface for interacting images
    fn images(&self) -> Images;
}

pub(crate) struct Docker<C>
    where C: Connect + 'static
{
    interact: Arc<InteractApi>,
    phantom: PhantomData<C>,
}

impl <C> Docker<C>
    where C: Connect + 'static
{
    pub(super) fn new_inner(interact: Arc<InteractApi>) -> Self
    {
        Self {
            interact,
            phantom: PhantomData
        }
    }
}

impl <C> DockerApi for Docker<C>
    where C: Connect + 'static
{
    fn version(&self) -> Box<Future<Item=Version, Error=Error> + Send> {
        let arg = "/version";

        Box::new(parse_to_trait::<Version>(self.interact.get(arg)))
    }

    fn info(&self) -> Box<Future<Item=Info, Error=Error> + Send> {
        let arg = "/info";

        Box::new(parse_to_trait::<Info>(self.interact.get(arg)))
    }

    fn ping(&self) -> Box<Future<Item=StatusCode, Error=Error> + Send> {
        let arg = "/_ping";

        Box::new(status_code(self.interact.get(arg)))
    }

    fn events(&self, opts: &EventsOptions) -> Box<Stream<Item=Result<Event>, Error=Error> + Send> {
        let query = opts.serialize();
        let arg = ("/events",  query.as_slice());

        Box::new(parse_to_stream::<Event>(self.interact.get(arg)))
    }

    fn container(&self, id: Cow<'static, str>) -> Container
    {
        let interact = self.interact.clone();
        Container::new(interact, id)
    }

    fn image<'a>(&self, id: Cow<'a, str>) -> Image<'a>
    {
        let interact = self.interact.clone();
        Image::new(interact, id)
    }

    fn images(&self) -> Images
    {
        let interact = self.interact.clone();
        Images::new(interact)
    }
}

fn default_uri(uri: Option<Uri>) -> Result<Uri> {
    use communicate::util::{URI_ENV, DEFAULT_URI};
    match uri {
        None => match env::var(URI_ENV) {
            Ok(var) => var.parse().map_err(Error::from),
            Err(e) => DEFAULT_URI.parse().map_err(Error::from),
        },
        Some(x) => Ok(x)
    }
}

/// Creates the docker struct relevant to the provided Uri
pub fn new_docker(host: Option<Uri>) -> Result<Box<DockerApi>>
{
    let host = default_uri(host)?;
    let scheme = host.scheme_part().map(|a| a.as_str().to_string());
    match scheme.as_slice() {
        Some(scheme) => match scheme {
            #[cfg(target_os = "linux")]
            "unix"  => UnixDocker::new(host),
            #[cfg(feature = "ssl")]
            "https" => TcpSSLDocker::new(host),
            "http"  => TcpDocker::new(host),
            _       => Err(ErrorKind::InvalidScheme.into()),
        }
        None => Err(ErrorKind::EmptyScheme.into())
    }
}