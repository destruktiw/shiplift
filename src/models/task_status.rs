/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.40) is used. For example, calling `/info` is the same as calling `/v1.40/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.40
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatus {
    #[serde(rename = "Timestamp")]
    timestamp: Option<String>,
    #[serde(rename = "State")]
    state: Option<::models::TaskState>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Err")]
    err: Option<String>,
    #[serde(rename = "ContainerStatus")]
    container_status: Option<::models::TaskStatusContainerStatus>,
}

impl TaskStatus {
    pub fn new() -> TaskStatus {
        TaskStatus {
            timestamp: None,
            state: None,
            message: None,
            err: None,
            container_status: None,
        }
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        self.timestamp = Some(timestamp);
    }

    pub fn with_timestamp(mut self, timestamp: String) -> TaskStatus {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn timestamp(&self) -> Option<&String> {
        self.timestamp.as_ref()
    }

    pub fn reset_timestamp(&mut self) {
        self.timestamp = None;
    }

    pub fn set_state(&mut self, state: ::models::TaskState) {
        self.state = Some(state);
    }

    pub fn with_state(mut self, state: ::models::TaskState) -> TaskStatus {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::TaskState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn with_message(mut self, message: String) -> TaskStatus {
        self.message = Some(message);
        self
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn reset_message(&mut self) {
        self.message = None;
    }

    pub fn set_err(&mut self, err: String) {
        self.err = Some(err);
    }

    pub fn with_err(mut self, err: String) -> TaskStatus {
        self.err = Some(err);
        self
    }

    pub fn err(&self) -> Option<&String> {
        self.err.as_ref()
    }

    pub fn reset_err(&mut self) {
        self.err = None;
    }

    pub fn set_container_status(&mut self, container_status: ::models::TaskStatusContainerStatus) {
        self.container_status = Some(container_status);
    }

    pub fn with_container_status(
        mut self,
        container_status: ::models::TaskStatusContainerStatus,
    ) -> TaskStatus {
        self.container_status = Some(container_status);
        self
    }

    pub fn container_status(&self) -> Option<&::models::TaskStatusContainerStatus> {
        self.container_status.as_ref()
    }

    pub fn reset_container_status(&mut self) {
        self.container_status = None;
    }
}