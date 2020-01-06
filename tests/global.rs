extern crate log; 
extern crate viperus;

 
fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_global()
{
    init();


    viperus::load_file(".env", viperus::Format::ENV).unwrap();
    let ok=viperus::get::<String>("TEST_BOOL").unwrap();
    assert_eq!("true",ok);

    viperus::add_default("default", true);
    assert_eq!(viperus::get::<bool>("default").unwrap(),true);

    viperus::add("default",false);

    assert_ne!(viperus::get::<bool>("default").unwrap(),true);


    
}

