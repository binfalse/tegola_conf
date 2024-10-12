use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    /// The max zoom which should be cached.
    ///
    /// Default: No
    pub max_zoom: Option<u32>,

    /// The type of cache to use (file, redis, or s3)
    #[serde(flatten)]
    pub cache_type: CacheType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CacheType {
    /// Cache tiles in a directory on the local filesystem.
    #[serde(rename = "file")]
    CacheFile(CacheFileConfig),

    /// Cache tiles in Redis.
    #[serde(rename = "redis")]
    CacheRedis(CacheRedisConfig),

    /// Cache tiles in Amazon S3.
    #[serde(rename = "s3")]
    CacheS3(CacheS3Config),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheFileConfig {
    /// A directory on the file system to write the cached tiles to.
    pub basepath: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheRedisConfig {
    /// The type of connection (tcp or unix)
    ///
    /// Default: `tcp`
    pub network: Option<NetworkType>,

    /// The address of Redis in the form ip:port.
    ///
    /// Default: `127.0.0.1:6379`
    pub address: Option<String>,

    /// Password to use when connecting.
    pub password: Option<String>,

    /// Database to use.
    pub db: Option<u32>,

    /// Encrypt connection to the Redis server.
    ///
    /// Default: `false`
    pub ssl: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheS3Config {
    /// The name of the S3 bucket to use.
    pub bucket: String,

    /// A path prefix added to all cache operations inside the S3 bucket
    pub basepath: Option<String>,

    /// The region the bucket is in.
    ///
    /// Default: `us-east-1`
    pub region: Option<String>,

    /// The AWS access key id to use.
    pub aws_access_key_id: Option<String>,

    /// The AWS secret access key to use.
    pub aws_secret_access_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkType {
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "unix")]
    UNIX,
}
