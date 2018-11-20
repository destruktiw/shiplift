use build::ContainerListOptions;
use communicate::util::AsSlice;
use futures::Future;
use hyper::Body;
use models::{ContainerConfig, ContainerCreateResponse};
use rep::Container as ContainerRep;
use serde_json;
use std::sync::Arc;
use transport::{
    interact::{InteractApi, InteractApiExt},
    parse::parse_to_trait,
};
use Error;

/// Interface for docker containers
pub struct Containers {
    interact: Arc<InteractApi>,
}

impl Containers {
    /// Exports an interface for interacting with docker containers
    pub(crate) fn new(interact: Arc<InteractApi>) -> Containers {
        Containers { interact }
    }

    /// Lists the container instances on the docker host
    pub fn list(
        &self,
        opts: &ContainerListOptions,
    ) -> impl Future<Item = Vec<ContainerRep>, Error = Error> {
        let path = "/containers/json";
        let query = opts.serialize();
        let args = (path, query.as_slice());

        parse_to_trait::<Vec<ContainerRep>>(self.interact.get(args))
    }

    /// Returns a builder interface for creating a new container instance
    pub fn create(
        &self,
        opts: &ContainerConfig,
    ) -> impl Future<Item = ContainerCreateResponse, Error = Error> {
        let path = "/containers/create";

        let body = serde_json::ser::to_string(opts).map(|s| Body::from(s)).ok();
        let args = (path, None, body);

        parse_to_trait(self.interact.post_json(args))
    }
}
