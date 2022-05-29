//! all the stuff that create a global instance of viperus
//!
//! the instance is "lazy_static" and protected by a mutex

use super::*;
use std::sync::mpsc::channel;

#[cfg(feature = "watch")]
use notify::Watcher;
use std::time::Duration;

use log::{error, info};
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "global")]
lazy_static::lazy_static! {
    /// the global instance
    static ref VIPERUS: Arc::<Mutex::<Viperus<'static>>> = Arc::new(Mutex::new(Viperus::new()));
}

/// Watch the config files and autoreload in case of change
///
/// the function starts a separate thread
/// TODO ad an unwatch_all() function;
#[cfg(feature = "watch")]
pub fn watch_all() -> Result<(), Box<dyn Error>> {
    let lf = VIPERUS.lock().unwrap().loaded_file_names();

    let vip = VIPERUS.clone();

    std::thread::spawn(move || {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        let mut watcher: notify::RecommendedWatcher =
            notify::Watcher::new(tx, Duration::from_secs(2)).unwrap();

        // Add a path to be watched. All files and directories at that path and

        for f in lf {
            watcher
                .watch(f, notify::RecursiveMode::NonRecursive)
                .unwrap();
        }

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            match rx.recv() {
                Ok(event) => {
                    info!("watch {:?}", event);
                    vip.lock().unwrap().reload().unwrap();
                }
                Err(e) => error!("watch error: {:?}", e),
            }
        }
    });

    Ok(())
}

/// load_file load a config file in the global instance
pub fn load_file(name: &str, format: Format) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_file(name, format)
}

/// load_adapter ask the adapter to parse her data and merges result
/// map in the internal configuration map of global instance
pub fn load_adapter(adt: &mut dyn ConfigAdapter) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_adapter(adt)
}

/// add an override value to the configuration
///
/// key is structured in components separated by a "."
pub fn add<T>(key: &str, value: T) -> Option<T>
where
    ViperusValue: From<T>,
    ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add(key, value)
}

/// get a configuration value of type T from global configuration in this order
/// * overridden key
/// * clap parameters
/// * config adapter sourced values
/// * default value
pub fn get<'a, 'b, T>(key: &'a str) -> Option<T>
where
    ViperusValue: From<T>,
    &'b ViperusValue: Into<T>,
    ViperusValue: Into<T>,
    T: FromStr,
    T: Clone,
{
    VIPERUS.lock().unwrap().get(key)
}

/// add an default value to the global configuration
///
/// key is structured in components separated by a "."
pub fn add_default<T>(key: &str, value: T) -> Option<T>
where
    ViperusValue: From<T>,
    ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add_default(key, value)
}

/// load_clap  brings in  the clap magic
#[cfg(feature = "fmt-clap")]
pub fn load_clap(matches: clap::ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_clap(matches)
}

/// bond a clap argument to a config key
#[cfg(feature = "fmt-clap")]
pub fn bond_clap(src: &str, dst: &str) -> Option<String> {
    VIPERUS.lock().unwrap().bond_clap(src, dst)
}

/// reload the configuration files
pub fn reload() -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().reload()
}

/// cache the query results for small configs speedup is x4
/// from v0.1.9 returns the previous state, useful for test setups.
#[cfg(feature = "cache")]
pub fn cache(enable: bool) -> bool {
    VIPERUS.lock().unwrap().cache(enable)
}

/// when enabled viperus will check for an environment variable any time Get request is made
/// checking  for a environment variable with a name matching the upper-cased key and prefixed with the
/// env_prefix if set.
pub fn automatic_env(enable: bool) {
    VIPERUS.lock().unwrap().automatic_env(enable)
}

/// prepend 'prefix' when querying environment variables
pub fn set_env_prefix(prefix: &str) {
    VIPERUS.lock().unwrap().set_env_prefix(prefix)
}
