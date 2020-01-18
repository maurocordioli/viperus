use std::path::Path;
use std::path::PathBuf;
use super::*;

/// EnvAdapter map a DotEnv file in a linear multilevel key/value array
/// 
/// the adaptor could be consumed by Viperous 
/// internally uses dotenv crate
pub struct EnvAdapter {
    data: std::collections::HashMap<String, String>,
    real_path:PathBuf,
}

 

impl EnvAdapter {
    pub fn new() -> Self {
        EnvAdapter {
            data: std::collections::HashMap::new(),
            real_path: PathBuf::default(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
    
        self.real_path=dotenv::from_filename(name)?;
        debug!("{:?}",self.real_path);
        Ok(())
    }

    pub fn get_real_path(&self) -> &Path {
        &self.real_path
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
