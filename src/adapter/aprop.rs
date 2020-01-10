use super::*;


use java_properties::PropertiesIter;

use std::fs::File;

use std::io::BufReader;

/// JPropertiesAdapter map a ajava properties file in a linear multilevel key/value array
///
/// the adaptor could be consumed by Viperous
/// internally uses java_properties crate
pub struct JavaPropertiesAdapter {
    data: std::collections::HashMap<String, String>,
}

impl JavaPropertiesAdapter {
    pub fn new() -> Self {
        JavaPropertiesAdapter {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        // Reading advanced
        let   f = File::open(name)?;
      
        PropertiesIter::new(BufReader::new(f)).read_into(|k, v| {
            self.data.insert(k, v);
        })?;
        Ok(())
    }
}
impl ConfigAdapter for JavaPropertiesAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        
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
