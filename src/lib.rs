//! viperus is an (in)complete configuration solution for Rust applications.
//!
//! I have already said that it is incomplete?
//! use at your own risk. ;-)
//! viperus handle some types of configuration needs and formats.
//!
//! It supports:
//! * setting defaults
//! * reading from JSON, TOML, YAML, envfile config files
//! * reading from environment variables
//! * reading from Clap command line flags
//! * setting explicit values
//!
//! Viperus uses the following decreasing precedence order.
//! * explicit call to `add`
//! * clap flag
//! * env
//! * config
//! * default
//!
#![warn(clippy::all)]
#[macro_use]

#[cfg(feature="global")]
extern crate lazy_static;


#[cfg(any(feature="fmt-yaml",feature="fmt-toml"))]
extern crate serde;
#[cfg(feature="ftm-yaml")]
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate dirs;

mod adapter;
mod map;
pub use adapter::AdapterResult;
pub use adapter::ConfigAdapter;

#[cfg(feature="cache")]
use std::cell::RefCell;

#[cfg(feature="ftm-calp")]
use clap;

pub use map::Map;
pub use map::ViperusValue;
use std::error::Error;
use std::fmt::Display;

use std::str::FromStr;


#[cfg(feature = "global")]
mod global;

#[cfg(feature = "global")]
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
    #[cfg(feature = "fmt-yaml")]
    YAML,
    #[cfg(feature = "fmt-json")]
    JSON,
    #[cfg(feature = "fmt-toml")] 
    TOML,
    #[cfg(feature = "fmt-env")] 
    ENV,
    #[cfg(feature = "fmt-javaproperties")]
    JAVAPROPERTIES,
}

/// A unified config Facade
///
/// Viperous manage config source from files, env and command line parameters in a unified manner
#[derive(Debug)]
pub struct Viperus<'a> {
    default_map: map::Map,
    config_map: map::Map,
    override_map: map::Map,

    #[cfg(feature = "fmt-clap")] 
    clap_matches: clap::ArgMatches<'a>,
    #[cfg(not(feature = "fmt-clap"))]    
    clap_matches: PhantomData<&'a u32>,
    
    #[cfg(feature = "fmt-clap")] 
    clap_bonds: std::collections::HashMap<String, String>,
    loaded_files: std::collections::LinkedList<(String, Format)>,
    #[cfg(feature = "cache")]
    cache_map: RefCell<map::Map>,
    #[cfg(feature = "cache")]
    cache_use: bool,

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
            #[cfg(feature = "fmt-clap")]
            clap_matches: clap::ArgMatches::default(),
            #[cfg(not(feature = "fmt-clap"))]
            clap_matches:PhantomData,
            
            #[cfg(feature = "fmt-clap")]
            clap_bonds: std::collections::HashMap::new(),
            loaded_files: std::collections::LinkedList::new(),
            #[cfg(feature = "cache")]
            cache_map: RefCell::new(map::Map::new()),
            #[cfg(feature = "cache")]
            cache_use: false,
        }
    }

    ///load_clap  brings in  the clap magic
    #[cfg(feature = "fmt-clap")] 
    pub fn load_clap(&mut self, matches: clap::ArgMatches<'v>) -> Result<(), Box<dyn Error>> {
        debug!("loading  {:?}", matches);

        self.clap_matches = matches;

        for &k in self.clap_matches.args.keys() {
            self.clap_bonds.insert(k.to_owned(), k.to_owned());
        }

        Ok(())
    }

    ///reload   all config file preserving the order
    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        self.config_map.drain();

        #[cfg(feature = "cache")]
        {
            if self.cache_use {
                self.cache(true);
            }
        }

        let lf = &self.loaded_files.iter().cloned().collect::<Vec<_>>();
        for (name, format) in lf {
            if std::path::Path::new(name).exists() {
                debug!("reloading  {} => {:?}", name, format);

                self.load_file(name, format.clone())?;
            } else {
                debug!("not exists  {} => {:?}", name, format);
            }
        }
        Ok(())
    }

    pub fn loaded_file_names(&self) -> Vec<String> {
        self.loaded_files.iter().map(|e| e.0.clone()).collect()
    }

    ///load_file load a config file using one of the preconfigured addapters
    ///then applay the adatpter using load_adapter method
    pub fn load_file(&mut self, name: &str, format: Format) -> Result<(), Box<dyn Error>> {
        debug!("loading  {}", name);

        match format {
            #[cfg(feature = "fmt-yaml")]   
            Format::YAML => {
                let mut adt = adapter::YamlAdapter::new();
                adt.load_file(name).unwrap();
                self.loaded_files.push_back((name.to_owned(), format));

                self.load_adapter(&mut adt)
            }
            #[cfg(feature = "fmt-json")]   
            Format::JSON => {
                let mut adt = adapter::JsonAdapter::new();
                adt.load_file(name).unwrap();
                self.loaded_files.push_back((name.to_owned(), format));

                self.load_adapter(&mut adt)
            }
   
            #[cfg(feature = "fmt-toml")]   
            Format::TOML => {
                let mut adt = adapter::TomlAdapter::new();
                adt.load_file(name).unwrap();
                self.loaded_files.push_back((name.to_owned(), format));

                self.load_adapter(&mut adt)
            }

            #[cfg(feature = "fmt-env")]   
            Format::ENV => {
                let mut adt = adapter::EnvAdapter::new();
                adt.load_file(name).unwrap();

                self.loaded_files
                    .push_back((adt.get_real_path().to_str().unwrap().to_owned(), format));
                self.load_adapter(&mut adt)
            }

            #[cfg(feature = "fmt-javaproperties")] 
             Format::JAVAPROPERTIES => {
                let mut adt = adapter::JavaPropertiesAdapter::new();
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
        T: Clone,
    {
        #[cfg(feature = "cache")]
        {
            if self.cache_use {
                let res = self.cache_map.borrow().get(key);

                if let Some(v) = res {
                    return Some(v);
                }
            }
        }

        let res = self.override_map.get(key);

        if let Some(v) = res {
            #[cfg(feature = "cache")]
            {
                if self.cache_use {
                    self.cache_map.borrow_mut().add(key, v.clone());
                }
            }
            return Some(v);
        }
    
        #[cfg(feature = "fmt-clap")] 
        let src = self.clap_bonds.get::<String>(&key.to_owned());
        #[cfg(feature = "fmt-clap")] 
        {
        if let Some(dst) = src {
            debug!("clap mapped {}=>{}", key, dst);

            if self.clap_matches.is_present(dst) {
                debug!("clap matched {}=>{}", key, dst);
                let res = self.clap_matches.value_of(dst);

                if let Some(v) = res {
                    let mv = &map::ViperusValue::Str(v.to_owned());
                    #[cfg(feature = "cache")]
                    {
                        if self.cache_use {
                            self.cache_map.borrow_mut().add(key, mv.clone().into());
                        }
                    }

                    return Some(mv.clone().into());
                }
            }
        }

    }

        let cfg = self.config_map.get(key);

        if cfg.is_some() {
            #[cfg(feature = "cache")]
            {
                if self.cache_use {
                    self.cache_map.borrow_mut().add(key, cfg.clone().unwrap());
                }
            }

            return cfg;
        }

        #[cfg(feature = "fmt-clap")] 
   {
        //default option value
        if let Some(dst) = src {
            debug!("clap default mapped {}=>{}", key, dst);
            if !self.clap_matches.is_present(dst) {
                debug!("clap default matched {}=>{}", key, dst);
                let res = self.clap_matches.value_of(dst);
                debug!("clap default value {}=>{} {:?}", key, dst, res);
                if let Some(v) = res {
                    let pval = v.parse::<T>().ok();
                    //UHMMMM TODO
                    #[cfg(feature = "cache")]
                    {
                        if self.cache_use {
                            self.cache_map.borrow_mut().add(key, pval.clone().unwrap());
                        }
                    }

                    return pval;
                }
            }
        }
    }

        let def = self.default_map.get(key);

        #[cfg(feature = "cache")]
        {
            if self.cache_use && def.is_some() {
                self.cache_map.borrow_mut().add(key, def.clone().unwrap());
            }
        }

        def
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

    #[cfg(feature = "fmt-clap")]   
    pub fn bond_clap(&mut self, src: &str, dst: &str) -> Option<String> {
        self.clap_bonds.insert(dst.to_owned(), src.to_owned())
    }

    /// add an default value to the configuration
    ///
    /// key is structured in components separated by a "."
    pub fn add_default<'a, T>(&'a mut self, key: &'a str, value: T) -> Option<T>
    where
        map::ViperusValue: From<T>,
        map::ViperusValue: Into<T>,
    {
        self.default_map.add(key, value)
    }

    /// cache the query results for small configs speedup is x4
    #[cfg(feature = "cache")]
    pub fn cache(&mut self, enable: bool) {
        self.cache_use = enable;

        if self.cache_use {
            let cache_old = &mut map::Map::new();
            std::mem::swap(cache_old, &mut self.cache_map.borrow_mut());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "fmt-json")]   
    fn lib_invalid_format() {
        init();
        let mut v = Viperus::default();
        v.load_file(&path!(".", "assets", "test.json"), Format::Auto)
            .unwrap();
    }
    #[test]
    fn lib_errors() {
        let e = ViperusError::Generic(String::from("generic"));
        let fe = format!("{}", e);
        let ex: Box<dyn Error> = Box::new(e);
        debug!("fe {}", fe);
        assert_ne!(ex.to_string(), "");
    }
    #[test]
    fn lib_works() {
        init();
        let mut v = Viperus::default();
        #[cfg(feature = "fmt-json")]   
        v.load_file(&path!(".", "assets", "test.json"), Format::JSON)
            .unwrap();
        #[cfg(feature = "fmt-yaml")]   
        v.load_file(&path!(".", "assets", "test.yaml"), Format::YAML)
            .unwrap();
        #[cfg(feature = "fmt-toml")]   
        v.load_file(&path!(".", "assets", "test.toml"), Format::TOML)
            .unwrap();

        #[cfg(feature = "fmt-javaproperties")]      
        v.load_file(
            &path!(".", "assets", "test.properties"),
            Format::JAVAPROPERTIES,
        )
        .unwrap();
        //v.load_file("asset\test.env", Format::JSON).unwrap();
        v.add("service.url", String::from("http://example.com"));
        debug!("final {:?}", v);

        let s: String = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s);
        #[cfg(feature = "fmt-cache")] 
   {
        v.cache(true);
        let s: String = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s);
        let s: String = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s);
        v.cache(false);
   }
        //test config
        #[cfg(feature = "fmt-json")] 
        {
        let json_b = v.get::<bool>("level1.key_json").unwrap();
        assert_eq!(true, json_b);
        }
        #[cfg(feature = "fmt-yaml")] 
   {
        let jyaml_b = v.get::<bool>("level1.key_yaml").unwrap();
        assert_eq!(true, jyaml_b);
   }

   #[cfg(feature = "fmt-javaproperties")] 
   {
        let jprop_b = v.get::<bool>("level1.java_properties").unwrap();
        assert_eq!(true, jprop_b);
   
        //test config with cache
        #[cfg(feature = "cache")]
        {
            v.cache(true);
            let jprop_b = v.get::<bool>("level1.java_properties").unwrap();
            assert_eq!(true, jprop_b);
            let jprop_b = v.get::<bool>("level1.java_properties").unwrap();
            assert_eq!(true, jprop_b);
            v.cache(false);
        }
    }
        //test default
        v.add_default("default", true);

        assert_eq!(v.get::<bool>("default").unwrap(), true);

        //test default with cache
        #[cfg(feature = "cache")]
        {
            v.cache(true);
            assert_eq!(v.get::<bool>("default").unwrap(), true);
            assert_eq!(v.get::<bool>("default").unwrap(), true);
            v.cache(false);
        }

        //reload
        v.reload().unwrap();

        assert_eq!(v.get::<bool>("default").unwrap(), true);
        //reload with cache
        #[cfg(feature = "cache")]
        {
            v.cache(true);
            v.reload().unwrap();
            assert_eq!(v.get::<bool>("default").unwrap(), true);
            v.cache(false);
        }
    }
}
