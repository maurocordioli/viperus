use super::*;
use toml;

/// TomlAdapter map a Toml file in a linear multilevel key/value array
/// 
/// the adaptor could be consumed by Viperous
/// internally it uses toml crate 
pub struct TomlAdapter {
    source: String,
    data: toml::map::Map<String, toml::Value>,
    //config_map: crate::map::Map,
}

impl TomlAdapter {
    pub fn new() -> Self {
        TomlAdapter {
            source: String::default(),
            data: toml::map::Map::new(),
            //config_map: crate::map::Map::new(),
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
        self.data = toml::from_str::<toml::map::Map<String, toml::Value>>(&self.source).unwrap();

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        //let mut kpath;

        for (k, v) in self.data.iter() {
            let kpath = k.to_owned();

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
            let i = *i as i32;
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
