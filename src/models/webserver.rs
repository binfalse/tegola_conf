use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Webserver {
    /// A string with the value for port.
    ///
    /// Default: `:8080`
    pub port: Option<String>,

    /// Set the hostname used to generate URLs for JSON based responses.
    ///
    /// Default: `HTTP Hostname in request`
    pub hostname: Option<String>,

    /// A prefix to add to all API routes. This is useful when tegola is behind a proxy (i.e. example.com/tegola). The prefix will be added to all URLs included in the capabilities endpoint responses.
    pub uri_prefix: Option<String>,

    /// Allows tegola to respond to tile request with user defined headers.
    ///
    /// Default CORS headers values:
    /// Access-Control-Allow-Origin	    `*`
    /// Access-Control-Allow-Methods	`GET, OPTIONS`
    pub headers: Option<HashMap<String, String>>,
}
