#[macro_use]
extern crate log;
extern crate clap;
extern crate viperus;

use clap::{App,Arg,SubCommand};
#[test]
fn main() {
    info!("test clap args");

    let matches = App::new("My Super Program")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();


    
                          let mut v= viperus::Viperus::new();
                          v.load_clap(matches);
                          
                          v.bond_clap("v","verbose");


                          let fVerbose=v.get::<bool>("verbose");
                        
                          debug!("verbose {:?}",fVerbose);
                          
                        

    assert_eq!(1,2);

}
