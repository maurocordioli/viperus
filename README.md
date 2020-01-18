# viperus  [![Build Status](https://travis-ci.com/maurocordioli/viperus.svg?branch=master)](https://travis-ci.com/maurocordioli/viperus) [![Coverage Status](https://coveralls.io/repos/github/maurocordioli/viperus/badge.svg?branch=master)](https://coveralls.io/github/maurocordioli/viperus?branch=master)
 ̶g̶o̶  rust configuration with fangs!
 
viperus is an (in)complete configuration solution for Rust applications. 
inspired  heavly inspired by the wonderful go package <https://github.com/spf13/viper>
use at your own risk. ;-)
## 
no Go projects h̶a̶s̶ ̶b̶e̶e̶n̶ ̶h̶a̶r̶m̶e̶d̶ are built using Viperus :-)

## Recent Changes
* 0.1.9 optional automatic prefixed environment  variable mapping,basic error propagation, cleaned dependency
* 0.1.8 add cache feature, modular "featurization"
* 0.1.5 add watch_all files with autoreload
* 0.1.4 add format : java properties files
* 0.1.3 better clap args : default values
* 0.1.2 relaod config from files
* 0.1.1 fixes dcs
* 0.1.0 first release

## What is Viperus?
a package that handles  some types of configuration  modes with differente formats,cli params and environment  . 
It supports:

* setting defaults
* reading from JSON, TOML, YAML, dotenv file ,java properties config files
* reading from environment variables
* reading from Clap command line flags
* setting explicit values
* reload of all files
* whatch config files and reolad all in something changes
* caching

## Why Viperus?

beacuse I was migrating some go apps... and a was missing Viper ease of use :-)

Viperus uses the following decreasing precedence order.

 * explicit call to `add`
 * clap flag
 * config
 * environment  variables
 * default

Viperus merge configuration from toml,dotenv,json,yaml files and clap options in sigle typed hash structure.
with defaults, and type checking

you can create a stand alone Viperus object or "enjoy" a global instance ( thread safe protected with a mutex)
via shadow functions load_file|get|add|load_clap that are routed to the static instance.  

```rust
//add a dotenv config file , keys defautls di env variables
viperus::load_file(".env", viperus::Format::ENV).unwrap();
//add another file
viperus::load_file("user.env", viperus::Format::ENV).unwrap();

 
//automatically map env variable staring with "TEST_" to config keys
viperus::set_env_prefix("TEST_");
viperus::automatic_env(true);

//watch the config and autoreload if something changes
viperus::watch_all();
   
//enable caching  -- file reload invalidates cache
// cache i thread safe for the global "static" instance
viperus::cache(true);
let ok=viperus::get::<bool>("TEST_BOOL").unwrap();
```
by the way , Yes I konw globals are evil. but as I was inspired by the  go package viper....
if you dislike globals you can opt-out disabling in your cargo.toml the feature "global".
   
## caching
you can enable caching for a x4 speed-up. 
cache is thread safe only when used with the global instance taht is behing a arc mutex
```rust
     viperus::cache(true);
```
reloading files with an excplicit ``` viperus::reload() ``` , 
or for effect of a file change when file watch is active invalidates the cache.

## logging/debug
the crate uses `log` facade , and test the `env_logger` you can set the env variable to RUST_LOG=viperus=[DEBUG LEVEL] with
[DEBUG LEVEL] = info|warning|debug  or RUST_LOG=[DEBUG LEVEL]
## features
the crate in "featurized" with the features enabled by default 
*  feature = "fmt-[format]" with [format] in 'json,end,toml,yaml,javaproperties,clap' enabling the relative format
*  feature ="global" enabling the global tread safe configuration
*  feature ="watch" enabling the automatic file reload wirh prerequisite feature=global
*  feature ="cache" aneglig caching 

single featues could be activated in a selective way  via cargo.toml 

```toml
[dependencies.viperus]
version = "0.1.8"
# do not include the default features, and optionally
default-features = false 
# cherry-pick individual features
features = ["global", "cache","watch","fmt-yaml"]
```
## Examples
you can find some integration tests in the test dir and also in the example folder
you can run example with cargo:
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
v.load_file(&path!(".","assets","test.properties"), Format::JAVAPROPERTIES).unwrap();
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
* remote configs
* better error propagation
* stabilize api
* improve documentation and examples
* improve my rust karma


