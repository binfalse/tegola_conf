use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    /// User defined data provider name. This is used by map layers to reference the data provider.
    pub name: String,

    /// The type of data provider. (i.e. “postgis”, “mvt_postgis”)
    #[serde(flatten)]
    pub provider_type: ProviderType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProviderType {
    /// Load data from a Postgres/PostGIS database.
    #[serde(rename = "postgis")]
    PostGIS(ProviderPostGIS),

    /// Load data from a Postgres/PostGIS database.
    #[serde(rename = "mvt_postgis")]
    MvtPostGIS(ProviderPostGIS),

    /// Load data from a GeoPackage database.
    #[serde(rename = "gpkg")]
    Gpkg(ProviderGeoPackage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderPostGIS {
    /// The database connection string.
    pub uri: String,

    /// The default SRID for this data provider
    ///
    /// Default: `3857`
    pub srid: Option<u32>,

    /// PostGIS Provider Layers define how Tegola will fetch data for a layer from a PostGIS Provider.
    pub layers: Vec<ProviderLayerPostgis>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderGeoPackage {
    /// The system file path to the GeoPackage you wish to connect to.
    pub filepath: String,

    /// GeoPackage Provider Layers define how Tegola will fetch data for a layer from a GeoPackage.
    pub layers: Vec<ProviderLayerGeoPackage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderLayerPostgis {
    /// The name that will be referenced from a map layer.
    pub name: String,

    /// The name of the geometry field in the table
    ///
    /// Default: `geom`
    pub geometry_fieldname: Option<String>,

    /// The layer geometry type. If not set, the table will be inspected at startup to try and infer the geometry type. Valid values are: Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon, GeometryCollection.
    pub geometry_type: Option<GeometryType>,

    /// The name of the feature ID field in the table. Only positive integer IDs are supported.
    ///
    /// Default: `gid`
    pub id_fieldname: Option<String>,

    /// Fields to include as tag values. Useful when using tablename
    pub fields: Option<Vec<String>>,

    /// The SRID for the table. Can be 3857 or 4326.
    pub srid: Option<Srid>,

    #[serde(flatten)]
    pub source: ProviderLayerSource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderLayerGeoPackage {
    /// The name of the feature ID field in the table. Only positive integer IDs are supported.
    ///
    /// Default: `fid`
    pub id_fieldname: Option<String>,

    /// Fields to include as tag values. Useful when using tablename
    pub fields: Option<String>,

    #[serde(flatten)]
    pub source: ProviderLayerSource,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u32)]
pub enum Srid {
    Srid3857 = 3857,
    Srid4326 = 4326,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderLayerSource {
    /// The name of the database table to query
    #[serde(rename = "tablename")]
    TableName(String),
    ///    Custom SQL. Requires a !BBOX! token
    #[serde(rename = "sql")]
    Sql(String),
}
