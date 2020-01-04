use std::collections::HashMap;

mod map_value;
pub use map_value::ViperusValue;

#[derive(Debug)]
pub struct Map {
    data: HashMap<String, ViperusValue>,
}


impl Default for Map { fn default() -> Self {Map::new()  } }   

impl Map {
    pub fn new() -> Self {
        Map {
            data: HashMap::new(),
        }
    }

    pub fn add<T>(&mut self, key: &str, value: T) -> Option<T>
    where
        ViperusValue: From<T>,
        ViperusValue: Into<T>,
    {
        match self.data.insert(key.to_string(), ViperusValue::from(value)) {
            None => None,
            Some(mv) => Some(mv.into()),
        }
    }

    pub fn add_value(&mut self, key: &str, value: ViperusValue) -> Option<ViperusValue> {
        self.data.insert(key.to_string(), value)
        //     let path: Vec<&str>=key.to_lowercase().split(".").collect();
        //     let pathLen = path.len();
        //    for pi  in 0..pathLen-1 {
        //        let v = self.data.get(path[pi]);
        //        if let None = v {
        //            let node=

        //        }

        //    }

        //     todo!("imlp add a key to the map")
    }

    pub fn get_value(&self, key: &str) -> Option<&ViperusValue> {
        self.data.get(key)
    }

    pub fn get<'a, T>(&'a self, key: &'a str) -> Option<T>
    where
        ViperusValue: From<T>,
        &'a ViperusValue: Into<T>,
    {
        match self.data.get(key) {
            None => None,
            Some(mv) => Some(mv.into()),
        }
    }

     


    pub fn merge(&mut self, src: &Map)  {
         
        for (k,v) in &src.data {

            self.add_value(k, v.clone());

        }

    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_add_get() {
        let mut m = Map::new();
        let mv0 = m.add_value("test.value", ViperusValue::I32(10));
        assert_eq!(None, mv0);
        let mv1 = m.get_value("test.value").unwrap();
        if let ViperusValue::I32(v1) = mv1 {
            assert_eq!(10, *v1);
        }  
    }

    #[test]
    fn test_map_get_32() {
        let mut m = Map::default();
        m.add_value("test.value2", ViperusValue::from("none"));

        let mv0 = m.add_value("test.value", ViperusValue::from(42));
        assert_eq!(None, mv0);
      
        let _a1 = m.add::<i32>("test.value_i32", 314).unwrap_or_default();
        let _a2 = m.add::<i32>("test.value_i32", 314).unwrap_or_default();
        
        let v1 = m.get::<i32>("test.value").unwrap();
        assert_eq!(42, v1);

        let v1_i32 = m.get::<i32>("test.value_i32").unwrap();
        assert_eq!(314, v1_i32);

         
        let v1_str = m.get::<&str>("test.value2").unwrap();
        assert_eq!("none", v1_str);

        
        
    }
    #[test]
    #[should_panic]
    fn invalid_map_get_32() {
        let mut m = Map::default();
        m.add_value("test.value2", ViperusValue::from("none"));

        assert!(m.get::<i32>("test.value2").is_none())
     
    }
}
