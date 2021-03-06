//! Transports for communicating with the docker daemon
extern crate tokio_codec;

use Result;

use hyper::Body;
use hyper::Method;
use hyper::Uri;
use hyper::Request;
use hyper::client::ResponseFuture;
use hyper::rt::Future;
use std::convert::Into;

use errors::Error;
use futures::future;
use http::StatusCode;
use std::fmt::Debug;
use futures::Stream;
use super::lines::Lines;
use http::uri::PathAndQuery;
use serde_json::from_str as de_from_str;
use std::str::FromStr;
use std::str;
use tokio::fs::File;
use std::path::Path;
use self::tokio_codec::FramedWrite;
use self::tokio_codec::BytesCodec;
use futures::Sink;
use hyper::Chunk;
use bytes::Bytes;

pub type ResponseFutureWrapper = Box<Future<Item=ResponseFuture, Error=Error> + Send>;

pub(crate) fn build_request<B>(method: Method, uri: Uri, body: B)
    -> Result<Request<Body>>
    where
        B: Into<Body>,
{
    let body: Body = body.into();

    Request::builder()
        .method(method)
        .uri(uri)
        .body(body)
        .map_err(Error::from)
}


pub(crate) fn status_code(future: ResponseFutureWrapper) -> impl Future<Item=StatusCode, Error=Error> + Send {
    future
        .and_then(|w| w.map_err(Error::from))
        .and_then(|response| {

            debug!("GET");
            future::ok(response.status())
        })
        .map_err(Error::from)
}


pub(crate) fn parse_to_trait<T>(future: ResponseFutureWrapper) -> impl Future<Item=T, Error=Error> + Send
    where
        T : for<'a> ::serde::Deserialize<'a> + Send + 'static
{
    future
        .and_then(|w| w
            .and_then(|response|
                response.into_body().concat2())
            .map_err(Error::from)
            .and_then(|chunk| {
                println!("{:?}", chunk.as_ref());
                de_from_str::<T>(str::from_utf8(chunk.as_ref())?)
                    .map_err(Error::from)
            })

        )
}


pub(crate) fn parse_to_lines(future: ResponseFutureWrapper) ->
    impl Stream<Item=String, Error=Error>
{
    future
        .and_then(|w| w
            .map_err(Error::from)
            .and_then(|response| {
                let body = response
                    .into_body()
                    .map_err(Error::from)
                    .map({ |a|
                        a.into_bytes().clone()
                    });

                let lines = Lines::new(body);

                Ok(lines)
            })
            .map_err(Error::from)
        )
        .flatten_stream()
}


pub(crate) fn parse_to_stream<T>(future: ResponseFutureWrapper) ->
        impl Stream<Item=Result<T>, Error=Error>
    where
        T : for<'a> ::serde::Deserialize<'a> + Send + Debug + 'static
{
    future
        .and_then(|w| w
            .map_err(Error::from)
            .and_then(|response| {
                let body = response
                    .into_body()
                    .map_err(Error::from)
                    .map({ |a|
                        a.into_bytes().clone()
                    });

                let lines = Lines::new(body);

                let mapped = lines
                    .map(|chunk| {
                        let as_str = str::from_utf8(chunk.as_ref())?;
                        let t = de_from_str::<T>(as_str)
                            .map_err(Error::from);
                        t
                    });

                Ok(mapped)
            })
            .map_err(Error::from)
        )
        .flatten_stream()
}

#[allow(dead_code)]
pub(crate) fn parse_to_file(future: ResponseFutureWrapper, filepath: &'static str)
    -> impl Future<Item=(), Error=Error>
{
    let stream = future
        .and_then(|w| w
            .map_err(Error::from)
            .and_then(|response| {
                let body = response
                    .into_body()
                    .map_err(Error::from);

                Ok(body)
            })
        )
        .flatten_stream();

    let file = File::create(Path::new(filepath));

    file
        .map_err(Error::from)
        .and_then(|file| {
            let write = FramedWrite::new(file, BytesCodec::new())
                .with(|chunk : Chunk| {
                    future::ok::<_, Error>(Bytes::from(chunk))
                });

            stream
                .forward(write)
                .and_then(|_|
                    Ok(()))
                .map_err(Error::from)
        })
}


pub(crate) fn compose_uri(host: &Uri, path: &str, query: &str) -> Result<Uri>
{
    let mut parts = host.clone().into_parts();
    let path_query = PathAndQuery::from_str(
        format!("{}?{}", path, query).as_ref())?;

    parts.path_and_query = Some(path_query);
    let res = Uri::from_parts(parts);

    debug!("URI parse result: {:?}", res);

    Ok(Uri::from(res?))
}


/*
/// Extract the error message content from an HTTP response that
/// contains a Docker JSON error structure.
#[allow(dead_code)]
fn get_error_message(res: &mut Response) -> Option<String> {
    let mut output = String::new();

    if res.read_to_string(&mut output).is_ok() {
        json::Json::from_str(output.as_str())
            .ok()
            .as_ref()
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("message"))
            .and_then(|x| x.as_string())
            .map(|x| x.to_owned())
    } else {
        None
    }
}
*/