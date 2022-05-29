use super::*;
use log::debug;

/// YamlAdapter map a Yaml file in a linear multilevel key/value array
///
/// the adaptor could be consumed by Viperous
pub struct YamlAdapter {
    source: String,
    data: serde_yaml::Mapping,
    //config_map: crate::map::Map,
}

impl YamlAdapter {
    pub fn new() -> Self {
        YamlAdapter {
            source: String::default(),
            data: serde_yaml::Mapping::new(),
        }
    }

    /// load_file
    ///
    /// # Arguments
    /// * `name`
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

impl ConfigAdapter for YamlAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = serde_yaml::from_str::<serde_yaml::Mapping>(&self.source)?;

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        //let mut kpath;

        for (k, v) in self.data.iter() {
            if let serde_yaml::Value::String(s) = k {
                let kpath = s.to_owned();

                rec_yaml(&mut res, &kpath, v);
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
                let kk = kpath.to_string();
                rec_yaml(config_map, &kk, vv);
            }
        }
        serde_yaml::Value::String(s) => {
            config_map.add(kpath, s.clone());
        }

        serde_yaml::Value::Number(num) => {
            let i = num.as_i64().unwrap_or_default() as i32;

            config_map.add(kpath, i);
        }

        serde_yaml::Value::Bool(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ayaml_basic() {
        let mut adp = YamlAdapter::new();
        adp.load(&mut "test: true\n".as_bytes()).unwrap();
        adp.parse().unwrap();
        let map = adp.get_map();
        let test_value = map.get::<bool>("test").unwrap();
        assert_eq!(test_value, true);
    }
}
