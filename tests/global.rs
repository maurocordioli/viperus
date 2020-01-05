#[macro_use]
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
    let ok=viperus::get::<bool>("TEST_BOOL").unwrap();

    assert_eq!(true,ok);

}

