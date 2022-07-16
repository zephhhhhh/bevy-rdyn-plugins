use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use std::fs;

use crate::dyn_api::*;

/// API extension for bevy to allow loading mods into an application.
pub trait ModLoaderExt {
    /// Loads a mod from a specified file path into an application.
    /// # Example
    /// ```
    /// let mut app = App::new();
    /// match app.load_mod("plugins/plugin.dll") {
    ///     Some(plugin) => println!("Loaded!"),
    ///     None => println!("Failed to load!"),
    /// }
    /// ```
    fn load_mod(&mut self, mod_path: &str) -> Option<RustDynPlugin>;
    /// Load all mods found in a directory into an application.
    /// # Example
    /// ```
    /// let mut app = App::new();
    /// app.load_mods("plugins");
    /// ```
    fn load_mods(&mut self, mods_directory: &str) -> &mut Self;
}

/// Stores all the loaded plugins loaded via the "load_mods" extension method.
#[derive(Default)]
pub struct ModLoaderData {
    pub loaded_plugins: Vec<RustDynPlugin>,
}

impl Deref for ModLoaderData {
    type Target = Vec<RustDynPlugin>;

    fn deref(&self) -> &Self::Target {
        &self.loaded_plugins
    }
}

impl DerefMut for ModLoaderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.loaded_plugins
    }
}

impl ModLoaderExt for App {
    fn load_mod(&mut self, mod_path: &str) -> Option<RustDynPlugin> {
        #[cfg(feature = "verbose_loading")]
        info!("Loading mod from: '{}'", mod_path);

        match RustDynPlugin::load_from(mod_path) {
            Some(plugin) => {
                plugin.build(self);
                #[cfg(feature = "verbose_loading")]
                info!("Loaded mod: {:?}", plugin);
                Some(plugin)
            }
            None => {
                #[cfg(feature = "verbose_loading")]
                warn!("Failed to load plugin from: '{}'", mod_path);
                None
            }
        }
    }

    fn load_mods(&mut self, mods_directory: &str) -> &mut Self {
        let mut mod_loader_data = ModLoaderData::default();

        match fs::read_dir(mods_directory) {
            Err(err) => warn!("Could not find mods folder! {}", err),
            Ok(plugins) => plugins
                .flatten()
                .filter(|p| p.file_type().map_or(false, |f| f.is_file()))
                .for_each(|plugin| match plugin.path().to_str() {
                    None => {
                        #[cfg(feature = "verbose_loading")]
                        warn!("Failed to get path of plugin from: '{}'", mod_path);
                    }
                    Some(plugin_path) => {
                        if let Some(plugin) = self.load_mod(plugin_path) {
                            mod_loader_data.loaded_plugins.push(plugin);
                        }
                    }
                }),
        }
        
        self.insert_resource(mod_loader_data);
        self
    }
}
