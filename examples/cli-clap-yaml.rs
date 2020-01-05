#[macro_use]
extern crate viperus;
extern crate env_logger;
extern crate clap;

 
fn main() {

    env_logger::init();
    let matches=clap::App::new("cliclapyaml")
    .arg( 
        clap::Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true),
    )
    .arg(
        clap::Arg::with_name("url")
        .short("u")
        .long("url")
        .value_name("url")
        .help("Sets a custom url")
        .takes_value(true),
    ).get_matches();

    viperus::load_clap(matches);
    viperus::bond_clap("url", "service.url");    
    viperus::load_file(&path!("examples","example.yaml"), viperus::Format::YAML);


    println!("this is cli-clap-yaml talkin.");
    println!("the wonderful uel is {}",viperus::get::<String>("service.url").unwrap());


}