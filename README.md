# viperus  [![Build Status](https://travis-ci.com/maurocordioli/viperus.svg?branch=master)](https://travis-ci.com/maurocordioli/viperus) [![Coverage Status](https://coveralls.io/repos/github/maurocordioli/viperus/badge.svg?branch=master)](https://coveralls.io/github/maurocordioli/viperus?branch=master)
 ~~go~~  rust configuration with fangs!

`viperus` is an (in)complete commandline configuration solution for Rust applications.
It is heavly inspired by the wonderful go package <https://github.com/spf13/viper>
Use it at your own risk. ;-)

no Go projects ~~has been harmed~~ are built consuming `viperus` :-)

## Recent Changes
* 0.1.10 adapters load cfg data from std:io::Read
* 0.1.9  optional automatic prefixed environment  variable mapping, basic error propagation, cleaned dependency
* 0.1.8  add cache feature, modular cargo  "feature" syntax
* 0.1.5  add watch_all files with autoreload
* 0.1.4  add format: java properties files
* 0.1.3  better clap args: default values
* 0.1.2  relaod config from files
* 0.1.1  fixes dcs
* 0.1.0  first release

## What is viperus?
Viperus is a package, that enables program configuration via an extendable
types system. Parameters can be incoprorated via cli parameters, environment
viariables. Parameters may be declared in configuration files as well.

Given implementation can handle:

* setting defaults
* reading from config files. Given formats are parsed:
  Dotenv, java-properties, JSON, TOML, YAML
* reading from environment variables
* reading from Clap command line flags
* setting explicit values
* reload of all files
* watch config files and reload, if any source value changes in first place
* caching

## Why viperus?

beacuse I was migrating some go apps ... and there was missing a rust tool that supplies `Viper` ease of use :-)

When config parameters are applied, Viperus uses the following decreasing precedence order:

 * explicit calls to `add`
 * clap flags values
 * config file parameters
 * environment variables
 * default parameters

`viperus` merges configuration parameters from

 * clap optoins
 * dotenv
 * json files
 * toml files
 * yaml files

in a single typed hash structure. The structure can be preset with default. Values are type checked.

You can create a standalone `viperus` object or "enjoy" a global
instance. The global instance is guarded with a Mutex to garantee
thread safty.

Structure elements of a static instance may be manipulated via the shadow functions

 * add
 * get
 * load_clap
 * load_file

```rust
// add a dotenv config file. keys defautls to the env variables
viperus::load_file(".env", viperus::Format::ENV).unwrap();

// add another file
viperus::load_file("user.env", viperus::Format::ENV).unwrap();

// automatically map env variable staring with "TEST_" to config keys
viperus::set_env_prefix("TEST_");
viperus::automatic_env(true);

// watch config parameters and autoreload changed values
viperus::watch_all();

// enable caching -- file reload invalidates cache
// the cache is thread safe for the global "static" instance
viperus::cache(true);
let ok=viperus::get::<bool>("TEST_BOOL").unwrap();
```

** Sidenote:  Yes I konw globals are evil. Inspiration was taken form the go package `viper` that is taking this route ....
If you dislike globals, or other preset defaults, go ahead and opt-out like this:

```cargo
cargo build --release --no-default-features --features "cache, fmt-clap, fmt-env, fmt-javaproperties, fmt-yaml, fmt-toml, notify, watch"
```

## caching
Enable caching and you will gain a x4 speed-up.
Cache is only thread safe if you use a global instance that is guarded with an arc mutex.

```rust
	viperus::cache(true);
```

The caching is invalidated, if you
 * reload any file with an excplicit ``` viperus::reload() ```
 * parameter changes inside a file, that is observed with the  `watch` feature

## logging/debug

The crate uses `log` facade. Testing is supported via the `env_logger` crate.
you can set the env variable to RUST_LOG=viperus=[DEBUG LEVEL] with

[DEBUG LEVEL] = info|warning|debug  or RUST_LOG=[DEBUG LEVEL]

## features
The crate may be adopted using cargo's "feature" semantic. The default enables all supported features:

*  feature = "fmt-[format]" with [format] in 'clap, env, javaproperties, json, toml, yaml' enabling the relative format
*  feature = "global" enabling the global tread safe configuration
*  feature = "watch" enabling the automatic file reload ( prerequisite: feature=global)
*  feature = "cache" enabling caching

single featues could be activated in a selective way  via cargo.toml

```toml
[dependencies.viperus]
version = "0.1.8"

# do not include the default features, and optionally
default-features = false

# cherry-pick individual features
features = ["global", "cache","watch","fmt-yaml"]
```

## Tests
All available integration tests are put into the subdirectory `tests`.
Reading the source code may help to get into the inner logic.

```
cargo test
```

## Examples

Inside the example subdirectory there is a reference implementation that consumes a yaml configuration file.

Compile and execute it like this:

```
cargo run --example cli-clap-yaml --
```

Since the `cli-clap-yaml` call does **not** define any commandline
arguments, the revealed parameters inside the config file will be parsed out.

In a second run, we call this example providing commandline parameters:

```
cargo run --example cli-clap-yaml -- -c ./example.conf -u http://nowhere/api/v1
```

Since the `cli-clap-yaml` does call **with** commandline
arguments, the revealed parameters inside the config file are overwritten.

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
[ ] remote configs
[ ] better error propagation
[ ] stabilize api
[ ] improve documentation and examples
[ ] improve my rust karma
