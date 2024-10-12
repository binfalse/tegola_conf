use std::{
    error::Error,
    fs::{File, OpenOptions},
};

use tegola_conf::{tegola_conf_read, tegola_conf_write};
use utils::compare_files;

mod utils;

#[test]
fn test_read() -> Result<(), Box<dyn Error>> {
    use tempfile::tempdir;

    let conf = tegola_conf_read("./testfiles/example-conf.toml".into());
    assert!(conf.is_ok());

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("example-conf");
    let mut file = File::create(&file_path).unwrap();
    let result = tegola_conf_write(&mut file, &conf.unwrap());
    assert!(result.is_ok());

    // let gold = File::open("./testfiles/example-conf-generated.toml");
    let written = OpenOptions::new().read(true).append(true).open(&file_path);
    let gold = OpenOptions::new()
        .read(true)
        .append(true)
        .open("./testfiles/example-conf-expect.toml");
    assert!(written.is_ok());
    assert!(gold.is_ok());

    let comparison = compare_files(written.unwrap(), gold.unwrap());
    assert!(comparison.is_ok());
    assert!(comparison.unwrap());
    Ok(())
}
