#[macro_use]
extern crate log;
extern crate clap;
extern crate viperus;
extern crate tempfile;
use std::io::Write;

use std::fs::File;


fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
#[cfg(feature = "fmt-watch")]
fn test_watch() {
    init();


    let mut cfg=tempfile::NamedTempFile::new().unwrap();
    //let cfgFile= cfg.as_file();
    cfg.write_all("level1:\n   key1: true\n".as_bytes()).unwrap();
    let cfg_path=cfg.into_temp_path();
    debug!("temp file is {}",cfg_path.to_str().unwrap());
    
    #[cfg(feature = "fmt-yaml")]
    viperus::load_file(cfg_path.to_str().unwrap(), viperus::Format::YAML).unwrap();
    viperus::watch_all().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    {
        debug!("write new file...{}",cfg_path.to_str().unwrap());

    let mut cfg_change=File::create(cfg_path.to_str().unwrap()).unwrap();
 
    cfg_change.write_all("level1:\n   key1: false\nlevel2: none\n".as_bytes()).unwrap();
    cfg_change.flush().unwrap();
    

    debug!("write new file...done");
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    let ok = viperus::get::<bool>("level1.key1").unwrap();
    assert_eq!(false, ok);
     
}




