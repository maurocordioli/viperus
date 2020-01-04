

pub type AdapterResult<T> = Result<T, Box<dyn std::error::Error>>;

mod ayaml;
mod ajson;
mod atoml;
mod aenv;

pub use ayaml::*;
pub use ajson::*;
pub use atoml::*;
pub use aenv::*;

/// ConfigAdapter mediates from varius config format and Viperus
pub trait ConfigAdapter {
    /// parse create he interna rappresentation of the config file/mode
    fn parse(&mut self) -> AdapterResult<()>;
    /// get_map returns a key value map rappresentation of the actaul config
    fn get_map(&self) -> crate::map::Map;
}


#[cfg(test)]
mod tests {
    use super::*;
   

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn adapter_json_load() {
        init();

        let mut a = JsonAdapter::new();
        a.load_str("{ \"json\": true }").unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("json").unwrap();
        assert_eq!(jtrue, true);
    }


    #[test]
    fn adapter_yaml_load() {
        init();

        let mut a = YamlAdapter::new();
        a.load_str("yaml: true\n").unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("yaml").unwrap();
        assert_eq!(jtrue, true);
    }

    #[test]
    fn adapter_toml_load() {
        init();

        let mut a = TomlAdapter::new();
        a.load_str("[level1]\nkey1=true\nkeyi32=42\nkey=\"hello world!\"\n").unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("level1.key1").unwrap();
        assert_eq!(jtrue, true);

        let ji32 = map.get::<i32>("level1.keyi32").unwrap();
        assert_eq!(ji32, 42);


        let jstr = map.get::<String>("level1.key").unwrap();
        assert_eq!(jstr, "hello world!");
        
        
    }
}
