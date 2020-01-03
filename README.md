# viperus
 ̶g̶o̶  rust configuration with fangs!
 
a incomplete rust package inspired by <https://github.com/spf13/viper>

## 
no Go projects are built using Viperus.

## Example
```rust


 let matches = App::new("My Super Program")
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();


    
   

let mut v = Viperus::new();
   
        v.load_clap(matches);
        v.load_file(&path!(".","assets","test.yaml"), Format::YAML).unwrap();
        v.load_file(&path!(".","assets","test.json"), Format::JSON).unwrap();
        v.load_file(&path!(".","assets","test.toml"), Format::TOML).unwrap();
   
        v.bond_clap("v","verbose");


        //v.load_file("asset\test.env", Format::JSON).unwrap();
        v.add("service.url", String::from("http://example.com"));
        debug!("final {:?}", v);

        let s: &str = v.get("service.url").unwrap();
        assert_eq!("http://example.com", s);
   
        let fVerbose=v.get::<bool>("verbose").unwrap();

        assert_eq!(true, fVerbose);
  
```rust
