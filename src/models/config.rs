use serde::{Deserialize, Serialize};

use super::{cache::Cache, map::Map, provider::Provider, webserver::Webserver};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The number of pixels to extend a tile’s clipping area
    ///
    /// Default: 64
    pub tile_buffer: Option<i32>,

    /// Configuration for the web server.
    pub webserver: Option<Webserver>,

    /// The providers configuration tells Tegola where your data lives. Data providers each have their own specific configuration, but all are required to have the following two config params:
    pub providers: Option<Vec<Provider>>,

    /// Tegola is responsible for serving vector map tiles, which are made up of numerous Map Layers. The name of the Map is used in the URL of all map tile requests (i.e. /maps/:map_name/:z/:x/:y).
    pub maps: Option<Vec<Map>>,

    /// Configuration for the caches for generated tiles.
    pub cache: Option<Cache>,
}
