#[macro_use]
extern crate log;
#[cfg(feature = "clap")]
extern crate clap;
extern crate viperus;
#[cfg(feature = "clap")]
use clap::{App, Arg, SubCommand};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}
/// here there is an error
/// this integration tests could be run in parallel.... so the caching state is unknown
#[test]
#[cfg(feature = "global")]
fn test_global() {
    init();

    #[cfg(feature = "cache")]
    let old=viperus::cache(false);

    #[cfg(feature = "fmt-env")]
    {
        viperus::load_file(".env", viperus::Format::ENV).unwrap();
        let ok = viperus::get::<String>("TEST_BOOL").unwrap();
        assert_eq!("true", ok);
    }

    viperus::add_default("default", true);
    assert_eq!(viperus::get::<bool>("default").unwrap(), true);

    viperus::add("default", false);

    #[cfg(feature = "cache")]
    viperus::cache(old);


    assert_ne!(viperus::get::<bool>("default").unwrap(), true);
}

/// a mockup adapter for testonly
struct ZeroAdapter {}
impl viperus::ConfigAdapter for ZeroAdapter {
    fn parse(&mut self) -> viperus::AdapterResult<()> {
        Ok(())
    }

    fn get_map(&self) -> viperus::Map {
        let res = viperus::Map::new();
        res
    }
}

#[test]
#[cfg(all(feature = "global", feature = "clap"))]
fn test_main() {
    init();
    info!("test clap args");

    #[cfg(feature = "fmt-clap")]
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                //.required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("nocapture")
                .long("nocapture")
                .help("enable no capture"),
        )
        .arg(
            Arg::with_name("showoutput")
                .long("show-output")
                .help("enable showoutput"),
        )
        .arg(Arg::with_name("quiet").long("quiet").help("enable quiet"))
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    #[cfg(feature = "fmt-env")]
    viperus::load_file(".env", viperus::Format::ENV).unwrap();
    #[cfg(feature = "fmt-clap")]
    {
        viperus::load_clap(matches).expect("strange...");
        viperus::bond_clap("v", "verbose");
    }
    viperus::add("verbose", true);

    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    debug!("verbose {:?}", f_verbose);
    info!(
        "RUST_LOG={}",
        dotenv::var("RUST_LOG").unwrap_or(String::from("none"))
    );
    assert_eq!(true, f_verbose);

    viperus::reload().unwrap();
    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    assert_eq!(true, f_verbose);
}

#[test]
#[cfg(feature = "global")]
fn test_adapter() {
    init();
    info!("test adapter creation");

    #[cfg(feature = "fmt-env")]
    viperus::load_file(".env", viperus::Format::ENV).unwrap();
    let mut adp = ZeroAdapter {};
    viperus::load_adapter(&mut adp).unwrap();
    viperus::add("verbose", true);

    let f_verbose = viperus::get::<bool>("verbose").unwrap();
    assert_eq!(true, f_verbose);

    #[cfg(feature = "fmt-env")]
    {
        std::env::set_var("TEST_MATCH", "true");
        viperus::automatic_env(true);
        viperus::set_env_prefix("TEST_");

        let f_test_match = viperus::get::<bool>("match").unwrap();
        assert_eq!(true, f_test_match);
        #[cfg(feature = "cache")]
        {
            viperus::cache(true);
            let f_test_match = viperus::get::<bool>("match").unwrap();
            assert_eq!(true, f_test_match);
            let f_test_match = viperus::get::<bool>("match").unwrap();
            assert_eq!(true, f_test_match);
        }
    }
}

#[test]
#[cfg(feature = "global")]
fn test_std_env() {
    init();
    info!("test adapter creation");

 
        std::env::set_var("TEST_MATCH", "true");
        viperus::automatic_env(true);
        viperus::set_env_prefix("TEST_");

        let f_test_match = viperus::get::<bool>("match").unwrap();
        assert_eq!(true, f_test_match);
        #[cfg(feature = "cache")]
        {
            viperus::cache(true);
            let f_test_match = viperus::get::<bool>("match").unwrap();
            assert_eq!(true, f_test_match);
            let f_test_match = viperus::get::<bool>("match").unwrap();
            assert_eq!(true, f_test_match);
        }
 

    }