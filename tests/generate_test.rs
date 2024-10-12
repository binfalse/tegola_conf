use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions},
};

use tegola_conf::{
    models::{
        cache::{Cache, CacheRedisConfig, CacheType, NetworkType},
        config::Config,
        map::{Map, MapLayer},
        provider::{
            GeometryType, Provider, ProviderLayerPostgis, ProviderLayerSource, ProviderPostGIS,
            ProviderType, Srid,
        },
        webserver::Webserver,
    },
    tegola_conf_write,
};
use utils::compare_files;

mod utils;

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    use tempfile::tempdir;
    let conf = Config {
                tile_buffer: Some(32),
                webserver: Some(Webserver {
                    port: Some(":8080".into()),
                    hostname: Some("localhost".into()),
                    uri_prefix: Some("prefix".into()),
                    headers: None,
                }),
                providers: Some(vec![
                    Provider {
                        name : "meun".into(),
                        provider_type: ProviderType::MvtPostGIS(
                            ProviderPostGIS{
                              uri : "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}?sslmode=${TEGOLA_POSTGIS_SSL}".into(),
                              srid : Some(4326),
                              layers: vec![
                                ProviderLayerPostgis{
                                    name : "sperrmuell".into(),
                                    geometry_fieldname : Some("wkb_geometry".into()),
                                    geometry_type: Some(GeometryType::Point),
                                    fields : Some(vec!["id".into(), "pickup".into(), "has_pic".into()]),
                                    source: ProviderLayerSource::Sql("SELECT * FROM meun.get_tile_data(!BBOX!, 'sperrmuell')".into()),
                                    id_fieldname: None,srid:None
                              },
                              ProviderLayerPostgis{
                                name :"verschenkenkiste".into(),
                                geometry_fieldname : Some("wkb_geometry".into()),
                                geometry_type:Some(GeometryType::Point),
                                fields : Some(vec!["id".into(), "pickup".into(), "has_pic".into()]),
                                source: ProviderLayerSource::TableName("tiledata".into()),
                                id_fieldname: Some("id".into()),srid:Some(Srid::Srid3857)
                              }],
                            })
                    }]),
                maps: Some(vec![
                    Map{
                        name: "meun".into(),
                        attribution: Some("meun.codeahoi.de".into()),
                        center: Some([12.1139, 54.0932, 11.0]),
                        bound: None,
                        tile_buffer: None,
                        layers: vec![
                            MapLayer{
                                provider_layer: "meun.sperrmuell".into(),
                                name: Some("sperrmuell layer".into()),
                                min_zoom : Some(1),
                                max_zoom : Some(50),
                                default_tags: Some(HashMap::from([("class".into(), "sperrmuell".into(),),])),
                                dont_simplify: Some(true)
                            },
                            MapLayer{
                                provider_layer: "meun.verschenkenkiste".into(),
                                name: Some("verschenkenkisten layer".into()),
                                min_zoom : Some(15),
                                max_zoom : Some(20),
                                default_tags: None, dont_simplify: Some(false)
                            }]
                        }]),
                cache: Some(Cache {
                    max_zoom: Some(13),
                    cache_type: CacheType::CacheRedis(
                        CacheRedisConfig {
                            network: Some(NetworkType::TCP),
                            address: Some("127.0.0.1:6379".into()),
                            password: Some("secret".into()),
                            db: Some(3),
                            ssl: Some(false)
                        }),
                })
            };

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("example-conf");
    let mut file = File::create(&file_path).unwrap();
    let result = tegola_conf_write(&mut file, &conf);
    assert!(result.is_ok());

    let written = OpenOptions::new().read(true).append(true).open(&file_path);
    let gold = OpenOptions::new()
        .read(true)
        .append(true)
        .open("./testfiles/dynamic-expect.toml");
    assert!(written.is_ok());
    assert!(gold.is_ok());

    let comparison = compare_files(written.unwrap(), gold.unwrap());
    assert!(comparison.is_ok());
    assert!(
        comparison.unwrap(),
        "generated file differs from expectation"
    );
    Ok(())
}
