#[macro_use]
extern crate viperus;
#[cfg(feature = "clap")]
extern crate clap;
extern crate env_logger;

//#[cfg(all(feature = "global", feature = "fmt-clap", feature = "fmt-yaml"))]
#[cfg(all(feature = "global", feature = "fmt-clap"))]
fn main() {
    println!("Viperus: opted in features `global`, `fmt-clap`");
    env_logger::init();

    let matches = clap::App::new("cliclapyaml")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                //.default_value("./examples/viperus.conf")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("url")
                .help("Sets a custom url")
                //.default_value("https://github.com/maurocordioli/viperus")
                .takes_value(true),
        )
        .get_matches();

    println!("Feature: `fmt-clap`, matching arguments ...");
    viperus::load_clap(matches).unwrap();
    viperus::bond_clap("config", "config.file");
    viperus::bond_clap("url", "service.url");

    #[cfg(feature = "fmt-yaml")]
    {
        println!("Feature: `fmt-yml`, parsing ...");
        viperus::load_file(&path!("examples", "example.yaml"), viperus::Format::YAML).unwrap();

        println!("Parsed parameters:");
        println!(
            " - config: {}",
            viperus::get::<String>("config.file").unwrap()
        );
        println!(" - url: {}", viperus::get::<String>("service.url").unwrap());
    }
}

#[cfg(not(all(feature = "global", feature = "fmt-clap")))]
fn main() {
    println!("Viperus: opted out feature `global`, `fmt-clap`");
}
