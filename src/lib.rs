#![warn(clippy::all)]
#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate dirs;

mod adapter;
mod map;
pub use adapter::ConfigAdapter;
use clap;
pub use map::ViperusValue;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature="global")] 
mod global {
use super::*;
use std::sync::Mutex;

    lazy_static! {
    static ref VIPERUS: Mutex::<Viperus<'static>> = { Mutex::new(Viperus::new()) };
}

pub fn load_file(name: &str, format: Format) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_file(name, format)
}

/// global config load_adapter ask the adapter to parse her data and merges result map in the internal configartion map
pub fn load_adapter(adt: &mut dyn adapter::ConfigAdapter) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_adapter(adt)
}

/// add an override value to the cofiguration
///
/// key is structured in components separated by a "."
pub fn add<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add(key, value)
}

/// get a configuration value of type T from global configuration in this order
/// * overrided key
/// * clap parameters
/// * config adapter sourced values
/// * default value
pub fn get<'a, 'b, T>(key: &'a str) -> Option<T>
where
    map::ViperusValue: From<T>,
    &'b map::ViperusValue: Into<T>,
    map::ViperusValue: Into<T>,
    T: FromStr,
{
    let v = VIPERUS.lock().unwrap();
    v.get(key)
}

/// add an default value to the global cofiguration
///
/// key is structured in components separated by a "."
pub fn add_default<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add_default(key, value)
}

///load_clap  brings in  the clap magic
pub fn load_clap(matches: clap::ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_clap(matches)
}

pub fn bond_clap(src: &str, dst: &str) -> Option<String> {
    VIPERUS.lock().unwrap().bond_clap(src, dst)
}
}

pub use global::*;


#[derive(Debug)]
pub enum ViperusError {
    Generic(String),
}
impl Error for ViperusError {}
impl Display for ViperusError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ViperusError::Generic(s) => write!(formatter, "Generic Error: {}", s),
        }
    }
}

#[macro_export]
macro_rules! path {
    ( $ x : expr ) =>  (format!("{}",$x));
    ( $ x: expr, $($y:expr),+) =>  (format!("{}{}{}",$x,std::path::MAIN_SEPARATOR,path!($($y),+)))
}

///preconfigured file formats with stock adapters
#[derive(Debug, Clone, Copy)]
pub enum Format {
    Auto,
    YAML,
    JSON,
    TOML,
    ENV,
}

/// A unified config Facade
///
/// Viperous manage config source from files, env and command line parameters in a unified manner
#[derive(Debug)]
pub struct Viperus<'a> {
    default_map: map::Map,
    config_map: map::Map,
    override_map: map::Map,
    clap_matches: clap::ArgMatches<'a>,
    clap_bonds: std::collections::HashMap<String, String>,
}

impl<'v> Default for Viperus<'v> {
    fn default() -> Self {
        Viperus::new()
    }
}

impl<'v> Viperus<'v> {
    pub fn new() -> Self {
        Viperus {
            default_map: map::Map::new(),
            config_map: map::Map::new(),
            override_map: map::Map::new(),
            clap_matches: clap::ArgMatches::default(),
            clap_bonds: std::collections::HashMap::new(),
        }
    }

    ///load_clap  brings in  the clap magic
    pub fn load_clap(&mut self, matches: clap::ArgMatches<'v>) -> Result<(), Box<dyn Error>> {
        debug!("loading  {:?}", matches);

        self.clap_matches = matches;
        Ok(())
    }

    ///load_file load a config file using one of the precinnfigured addapters
    ///then applay the adatpter using load_adapter method
    pub fn load_file(&mut self, name: &str, format: Format) -> Result<(), Box<dyn Error>> {
        debug!("loading  {}", name);

        match format {
            Format::YAML => {
                let mut adt = adapter::YamlAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            }
            Format::JSON => {
                let mut adt = adapter::JsonAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            }
            Format::TOML => {
                let mut adt = adapter::TomlAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            }

            Format::ENV => {
                let mut adt = adapter::EnvAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            }

            _ => Err::<(), Box<dyn Error>>(Box::new(ViperusError::Generic(
                "Format not implemented".to_owned(),
            ))),
        }
    }

    /// load_adapter ask the adapter to parse her data and merges result map in the internal configartion map
    pub fn load_adapter(
        &mut self,
        adt: &mut dyn adapter::ConfigAdapter,
    ) -> Result<(), Box<dyn Error>> {
        adt.parse().unwrap();
        self.config_map.merge(&adt.get_map());
        Ok(())
    }

    /// get a configuration value of type T in this order
    /// * overrided key
    /// * clap parameters
    /// * config adapter sourced values
    pub fn get<'a, 'b, 'c, T>(&'a self, key: &'b str) -> Option<T>
    where
        map::ViperusValue: From<T>,
        &'c map::ViperusValue: Into<T>,
        map::ViperusValue: Into<T>,
        T: FromStr,
    {
        let res = self.override_map.get(key);

        if let Some(v) = res {
            return Some(v);
        }

        let src = self.clap_bonds.get::<String>(&key.to_owned());
        if let Some(dst) = src {
            if self.clap_matches.is_present(dst) {
                let res = self.clap_matches.value_of(dst);

                if let Some(v) = res {
                    let mv = &map::ViperusValue::Str(v.to_owned());
                    return Some(mv.clone().into());
                }
            }
        }

        let cfg = self.config_map.get(key);

        if cfg.is_some() {
            return cfg;
        }

        
        //default option value
        if let Some(dst) = src {
            if !self.clap_matches.is_present(dst) {
                let res = self.clap_matches.value_of(dst);

                if let Some(v) = res {
                    return v.parse::<T>().ok();
                }
            }
        }

       self.default_map.get(key)


    }

    /// add an override value to the cofiguration
    ///
    /// key is structured in components separated by a "."
    pub fn add<'a, T>(&'a mut self, key: &'a str, value: T) -> Option<T>
    where
        map::ViperusValue: From<T>,
        map::ViperusValue: Into<T>,
    {
        self.override_map.add(key, value)
    }

    pub fn bond_clap(&mut self, src: &str, dst: &str) -> Option<String> {
        self.clap_bonds.insert(dst.to_owned(), src.to_owned())
    }

    /// add an default value to the cofiguration
    ///
    /// key is structured in components separated by a "."
    pub fn add_default<'a, T>(&'a mut self, key: &'a str, value: T) -> Option<T>
    where
        map::ViperusValue: From<T>,
        map::ViperusValue: Into<T>,
    {
        self.default_map.add(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn lib_errors() {
        let e = ViperusError::Generic(String::from("generic"));
        let fe = format!("{}", e);
        let ex: Box<dyn Error> = Box::new(e);
        debug!("fe {}", fe);

        assert_ne!(ex.description(), "");
    }
    #[test]
    fn it_works() {
        init();
        let mut v = Viperus::default();
        v.load_file(&path!(".", "assets", "test.json"), Format::JSON)
            .unwrap();
        v.load_file(&path!(".", "assets", "test.yaml"), Format::YAML)
            .unwrap();
        v.load_file(&path!(".", "assets", "test.toml"), Format::TOML)
            .unwrap();
        //v.load_file("asset\test.env", Format::JSON).unwrap();
        v.add("service.url", String::from("http://example.com"));
        debug!("final {:?}", v);

        let s: String = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s);

        let json_b = v.get::<bool>("level1.key_json").unwrap();
        assert_eq!(true, json_b);

        let jyaml_b = v.get::<bool>("level1.key_yaml").unwrap();
        assert_eq!(true, jyaml_b);

        v.add_default("default", true);



        assert_eq!(v.get::<bool>("default").unwrap(),true);
    }
}
