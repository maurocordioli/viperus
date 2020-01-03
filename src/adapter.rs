use serde_yaml;
use toml;

#[derive(Debug)]
pub struct AdapterError;

impl From<std::io::Error> for AdapterError {
    fn from(src: std::io::Error) -> Self {
        todo!("conversion error {}",src);
        AdapterError {}
    }
}

impl From<serde_yaml::Error> for AdapterError {
    fn from(src: serde_yaml::Error) -> Self {
        todo!("conversion error {}",src);
        AdapterError {}
    }
}

pub type AdapterResult<T> = Result<T, AdapterError>;

pub trait ConfigAdapter {
    fn parse(&mut self) -> AdapterResult<()>;
    fn get_map(&self) -> crate::map::Map;
}

pub struct YamlAdapter {
    source: String,
    data: serde_yaml::Mapping,
    config_map: crate::map::Map,
}

impl YamlAdapter {
    pub fn new() -> Self {
        YamlAdapter {
            source: String::default(),
            data: serde_yaml::Mapping::new(),
            config_map: crate::map::Map::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        self.source = std::fs::read_to_string(name)?;

        Ok(())
    }

    pub fn load_str(&mut self, source: &str) -> AdapterResult<()> {
        self.source = source.to_owned();

        Ok(())
    }
}
impl ConfigAdapter for YamlAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = serde_yaml::from_str::<serde_yaml::Mapping>(&self.source)?;

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        let mut kpath = String::default();

        for (k, v) in self.data.iter() {
            if let serde_yaml::Value::String(s) = k {
                kpath = s.to_owned();

                rec_yaml(&mut res, &kpath, &v);
            }
        }

        res
    }
}

fn rec_yaml(config_map: &mut crate::map::Map, kpath: &str, v: &serde_yaml::Value) {
    debug!("{:?} => {:?}", kpath, v);

    match v {
        serde_yaml::Value::Mapping(m) => {
            for (kk, vv) in m {
                if let serde_yaml::Value::String(s) = kk {
                    let kk = format!("{}.{}", kpath, s);
                    rec_yaml(config_map, &kk, vv);
                }
            }
        }

        serde_yaml::Value::Sequence(m) => {
            for vv in m {
                let kk = format!("{}", kpath);
                rec_yaml(config_map, &kk, vv);
            }
        }
        serde_yaml::Value::String(s) => {
            config_map.add(kpath, s.clone());
        }

        serde_yaml::Value::Bool(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}




pub struct JsonAdapter {
    source: String,
    data: serde_json::Map::<String,serde_json::Value>,
    config_map: crate::map::Map,
}

impl JsonAdapter {
    pub fn new() -> Self {
        JsonAdapter {
            source: String::default(),
            data: serde_json::Map::new(),
            config_map: crate::map::Map::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        self.source = std::fs::read_to_string(name)?;

        Ok(())
    }

    pub fn load_str(&mut self, source: &str) -> AdapterResult<()> {
        self.source = source.to_owned();

        Ok(())
    }
}
impl ConfigAdapter for JsonAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = serde_json::from_str::<serde_json::Map<String,serde_json::Value>>(&self.source).unwrap();

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        let mut kpath = String::default();

        for (k, v) in self.data.iter() {
            
                kpath = k.to_owned();

                rec_json(&mut res, &kpath, v);
            
        }

        res
    }
}


fn rec_json(config_map: &mut crate::map::Map, kpath: &str, v: &serde_json::Value) {
    debug!("{:?} => {:?}", kpath, v);

    match v {
        serde_json::Value::Object(m) => {
            for (kk, vv) in m {
               
                    let kk = format!("{}.{}", kpath, kk);
                    rec_json(config_map, &kk, vv);
                
            }
        }

        
        serde_json::Value::String(s) => {
            config_map.add(kpath, s.clone());
        }

        serde_json::Value::Bool(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}




pub struct TomlAdapter {
    source: String,
    data: toml::map::Map::<String,toml::Value>,
    config_map: crate::map::Map,
}

impl TomlAdapter {
    pub fn new() -> Self {
        TomlAdapter {
            source: String::default(),
            data: toml::map::Map::new(),
            config_map: crate::map::Map::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        self.source = std::fs::read_to_string(name)?;

        Ok(())
    }

    pub fn load_str(&mut self, source: &str) -> AdapterResult<()> {
        self.source = source.to_owned();

        Ok(())
    }
}
impl ConfigAdapter for TomlAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = toml::from_str::<toml::map::Map<String,toml::Value>>(&self.source).unwrap();

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        let mut kpath = String::default();

        for (k, v) in self.data.iter() {
            
                kpath = k.to_owned();

                rec_toml(&mut res, &kpath, v);
            
        }

        res
    }
}


fn rec_toml(config_map: &mut crate::map::Map, kpath: &str, v: &toml::Value) {
    debug!("{:?} => {:?}", kpath, v);

    match v {
        toml::Value::Table(m) => {
            for (kk, vv) in m {
               
                    let kk = format!("{}.{}", kpath, kk);
                    rec_toml(config_map, &kk, vv);
                
            }
        }

        toml::Value::Integer(i) => {
            let i=*i as i32;
            config_map.add(kpath, i);
        }

        
        toml::Value::String(s) => {
            config_map.add(kpath, s.clone());
        }

        toml::Value::Boolean(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}
