use super::*;

/// EnvAdapter map a DotEnv file in a linear multilevel key/value array
/// 
/// the adaptor could be consumed by Viperous 
/// internally uses dotenv crate
pub struct EnvAdapter {
    data: std::collections::HashMap<String, String>,
}

 

impl EnvAdapter {
    pub fn new() -> Self {
        EnvAdapter {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
    
        debug!("{:?}",dotenv::from_filename(name).unwrap());
        Ok(())
    }
}
impl ConfigAdapter for EnvAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = dotenv::vars().collect();
        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

     
        for (k, v) in self.data.iter() {
            
            res.add(k, v.to_owned());
        }

        res
    }
}
