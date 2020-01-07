# viperus  [![Build Status](https://travis-ci.com/maurocordioli/viperus.svg?branch=master)](https://travis-ci.com/maurocordioli/viperus) [![Coverage Status](https://coveralls.io/repos/github/maurocordioli/viperus/badge.svg?branch=master)](https://coveralls.io/github/maurocordioli/viperus?branch=master)
 ̶g̶o̶  rust configuration with fangs!
 
viperus is an (in)complete configuration solution for Rust applications. 
inspired  heavly inspired by the wonderful go package <https://github.com/spf13/viper>
I have already said that it is incomplete? 
use at your own risk. ;-)

## 
no Go projects are built using Viperus :-)

## What is Viperus?
handle some types of configuration needs and formats. 
It supports:

* setting defaults
* reading from JSON, TOML, YAML, envfile config files
* reading from environment variables
* reading from Clap command line flags
* setting explicit values

## Why Viperus?

beacuse I was migrating some go apps... and a was missing Viper ease of use :-)

Viperus uses the following decreasing precedence order.

 * explicit call to `add`
 * clap flag
 * env
 * config
 * default

Viperus merge configuration from toml,dotenv,json,yaml files and clap options in sigle typed hash structure.
with defaults, and type checking

you can create a stand alone Viperus object or "enjoy" a global instance ( thread safe protected with a mutex)
via shadow functions load_file|get|add|load_clap that are routed to the static instance.  


```rust
     viperus::load_file(".env", viperus::Format::ENV).unwrap();
     let ok=viperus::get::<bool>("TEST_BOOL").unwrap();
```
by the way , Yes I konw globals are evil. but as I was inspired by the  go package viper....
if you dislike globals you can opt-out disabling in your cargo.toml the feature "global".

## logging/debug
the crate uses `log` facade , and test the `env_logger` you can set the env variable to RUST=viperus=[DEBUG LEVEL] with
[DEBUG LEVEL] = info|warning|debug 


## Example
you can find some integration tests in the test dir and also in the example forlder
you can run example with cargo

```
cargo run --example cli-clap-yaml -- 
cargo run --example cli-clap-yaml -- -u http://nowhere/api/v1
```
the first run print the value from the example.yaml file 
the second from the cli arg


```rust


 let matches = App::new("My Super Program")
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();   

let mut v = Viperus::new();

//enable clap
v.load_clap(matches);
//enable a yaml json toml file
v.load_file(&path!(".","assets","test.yaml"), Format::YAML).unwrap();
v.load_file(&path!(".","assets","test.json"), Format::JSON).unwrap();
v.load_file(&path!(".","assets","test.toml"), Format::TOML).unwrap();

//link the "v" clap option to the key "verbose"
v.bond_clap("v","verbose");


//add an explicit overload 
v.add("service.url", String::from("http://example.com"));
debug!("final {:?}", v);

//get a typed key
let s: &str = v.get("service.url").unwrap();
assert_eq!("http://example.com", s);

//get a bool from configs or app args
let fVerbose=v.get::<bool>("verbose").unwrap();
assert_eq!(true, fVerbose);
  
```
## Todo
* type inference  for .env files from defaults 
* stabilize api
* documentation
* improove my rust karma

