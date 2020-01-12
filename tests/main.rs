#[macro_use]
extern crate log;
extern crate clap;
extern crate viperus;

use clap::{App, Arg, SubCommand};
fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_main() {
    init();
    info!("test clap args");

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
        .arg(
            Arg::with_name("argdefault")
                .short("a")
                .long("argdefault")
                .help("an argument with default")
                .takes_value(true)
                .default_value("none"),
        )
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

    let mut v = viperus::Viperus::new();
    #[cfg(feature = "fmt-env")]
    v.load_file(".env", viperus::Format::ENV).unwrap();
    #[cfg(feature = "fmt-clap")]{
    v.load_clap(matches).expect("strange...");
    v.bond_clap("v", "verbose");
    v.bond_clap("argdefault", "argdefault");
    }
    v.add("verbose", true);


    let f_verbose = v.get::<bool>("verbose").unwrap();
    debug!("verbose {:?}", f_verbose);

    #[cfg(feature = "fmt-env")]
    info!(
        "RUST_LOG={}",
        dotenv::var("RUST_LOG").unwrap_or(String::from("none"))
    );
    assert_eq!(true, f_verbose);

    #[cfg(feature = "cache")]
    {
        v.cache(true);
        let f_verbose = v.get::<bool>("verbose").unwrap();
        assert_eq!(true, f_verbose);
        let f_verbose = v.get::<bool>("verbose").unwrap();
        assert_eq!(true, f_verbose);
        v.cache(false);
    }

    #[cfg(feature = "fmt-clap")]{
    let f_argdefault = v.get::<String>("argdefault").unwrap();
    assert_eq!("none", f_argdefault);
    }
}
