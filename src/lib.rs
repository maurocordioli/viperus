#![warn(clippy::all)]

extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate dirs;
mod adapter;
mod map;
use std::fmt::Display; 
use std::error::{Error};
use clap;

#[derive(Debug)]
pub enum ViperusError { Generic(String) }
impl Error for ViperusError {}
impl Display for ViperusError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter::<'_>) ->std::fmt::Result 
    {
        match &self {
            ViperusError::Generic(s) => write!(formatter, "Generic Error: {}", s)
                }
    }

}

#[macro_export]
macro_rules! path { 
    ( $ x : expr ) =>  (format!("{}",$x));
    ( $ x: expr, $($y:expr),+) =>  (format!("{}{}{}",$x,std::path::MAIN_SEPARATOR,path!($($y),+)))
     
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Auto,
    YAML,
    JSON,
    TOML,
    ENV,
}

#[derive(Debug)]
pub struct Viperus<'a> {
    config_map: map::Map,
    override_map: map::Map,
    clap_matches: clap::ArgMatches<'a>,
    clap_bonds : std::collections::HashMap<String,String>,
}

impl<'v> Default for Viperus<'v> { fn default() -> Self {Viperus::new()  } }   

impl<'v> Viperus<'v> {
    pub fn new() -> Self {
        Viperus {
            config_map: map::Map::new(),
            override_map: map::Map::new(),
            clap_matches: clap::ArgMatches::default(),
            clap_bonds: std::collections::HashMap::new(),
        }
    }


    pub fn load_clap(&mut self,matches:clap::ArgMatches<'v>) -> Result<(), Box<dyn Error>> {
        debug!("loading  {:?}", matches);
        
        self.clap_matches=matches;

        Ok(())
    }

    pub fn load_file(&mut self, name: &str, format: Format) -> Result<(),Box<dyn Error>> {
        debug!("loading  {}", name);
     
       match format {
            Format::YAML => {
                let mut adt = adapter::YamlAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            },
            Format::JSON => {
                let mut adt = adapter::JsonAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            },
            Format::TOML => {
                let mut adt = adapter::TomlAdapter::new();
                adt.load_file(name).unwrap();
               self.load_adapter(&mut adt)
            },

            Format::ENV => {
                let mut adt = adapter::EnvAdapter::new();
                adt.load_file(name).unwrap();
                self.load_adapter(&mut adt)
            },
                        
        _ => {
              Err::<(),Box<dyn Error>>(Box::new(ViperusError::Generic("Format not implemented".to_owned())))
            }
        }

    
    }

    pub fn load_adapter(&mut self, adt: &mut dyn adapter::ConfigAdapter) -> Result<(),Box<dyn  Error>> {
        adt.parse().unwrap();
        self.config_map.merge(&adt.get_map());
        Ok(())
    }

    pub fn get<'a, T>(&'a self, key: &'a str) -> Option<T>
    where
        map::MapValue: From<T>,
        &'a map::MapValue: Into<T>,
        map::MapValue: Into<T>,
       
    {

       let res= self.override_map.get(key);
       
       if let Some(v) = res {
        return Some(v)
   
        }
       
       let src= self.clap_bonds.get::<String>(&key.to_owned());
       if let Some(dst) = src {
        
         let res=self.clap_matches.value_of(dst);

         if let Some(v) = res {
            
            let mv=&map::MapValue::Str(v.to_owned());

           return Some(  mv.clone().into() );
   
         }
        }

      
       self.config_map.get(key)

    }

    pub fn add<'a, T>(&'a mut self, key: &'a str, value: T) -> Option<T>
    where
        map::MapValue: From<T>,
        map::MapValue: Into<T>,
    {
        self.override_map.add(key, value)
    }

   pub fn bond_clap(&mut self,src:&str,dst:&str)  -> Option<String>

{
self.clap_bonds.insert(dst.to_owned(), src.to_owned())
}
 

    
}

#[cfg(test)]
mod tests {
    use crate::map::MapValue;
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_works() {
        init();
        let mut v = Viperus::new();
        v.load_file(&path!(".","assets","test.yaml"), Format::YAML).unwrap();
        v.load_file(&path!(".","assets","test.json"), Format::JSON).unwrap();
        v.load_file(&path!(".","assets","test.toml"), Format::TOML).unwrap();
       
        //v.load_file("asset\test.env", Format::JSON).unwrap();
        v.add("service.url", String::from("http://example.com"));
        debug!("final {:?}", v);

        let s: String = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s); 
    }
}
