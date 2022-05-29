use super::*;
use log::debug;

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
        let mut f = std::fs::File::open(name)?;
        self.load(&mut f)
    }

    pub fn load<R: std::io::Read>(&mut self, source: &mut R) -> AdapterResult<()> {
        self.source.truncate(0);

        source.read_to_string(&mut self.source)?;
        Ok(())
    }
}
impl ConfigAdapter for TomlAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = toml::from_str::<toml::map::Map<String, toml::Value>>(&self.source)?;

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
