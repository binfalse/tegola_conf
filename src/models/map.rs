use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    /// The map that will be referenced in the URL (i.e. /maps/:map_name.
    pub name: String,

    /// Attribution string to be included in the TileJSON.
    pub attribution: Option<String>,

    /// The bounds in latitude and longitude values, in the order left, bottom, right, top. Default: [-180.0, -85.0511, 180.0, 85.0511]
    pub bound: Option<[f32; 4]>,

    /// The center of the map to be displayed in the preview. ([lon, lat, zoom]).
    pub center: Option<[f64; 3]>,

    /// The number of pixels to extend a tile’s clipping area, defaults to 64 or the global value
    pub tile_buffer: Option<u32>,

    ///
    pub layers: Vec<MapLayer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapLayer {
    /// The name of the provider and provider layer using dot syntax. (i.e. my_postgis.rivers).
    pub provider_layer: String,

    /// Overrides the provider_layer name. Can also be used to group multiple provider_layers under the same namespace.
    pub name: Option<String>,

    /// The minimum zoom to render this layer at.
    pub min_zoom: Option<u32>,

    /// The maximum zoom to render this layer at.
    pub max_zoom: Option<u32>,

    /// Default tags to be added to features on this layer.
    pub default_tags: Option<HashMap<String, String>>,

    /// Boolean to prevent feature simplification from being applied.
    pub dont_simplify: Option<bool>,
}
