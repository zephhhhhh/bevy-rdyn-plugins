use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use bevy::prelude::Plugin;
use libloading::{Library, Symbol};

/// Name of symbol to be exported/imported to create the plugin.
pub const CREATE_RDYN_SYM_NAME: &'static [u8] = b"_create_rdyn_plugin";
/// The type required to be returned from the plugin creation function.
pub type RDynReturn = Box<dyn Plugin>;
/// Type that represents the function signature of create plugin symbol.
pub type CreateRDynPlugin = fn() -> RDynReturn;

/// Stores a Rust dynamic plugin along with the dynamic library from which it was loaded.
/// Automatically deferences to a box of a bevy Plugin and so can be used as such.
pub struct RustDynPlugin {
    /// The library the plugin was loaded from.
    pub library: Library,
    /// The plugin itself.
    pub plugin: Box<dyn Plugin>,
}

impl Deref for RustDynPlugin {
    type Target = Box<dyn Plugin>;

    fn deref(&self) -> &Self::Target {
        &self.plugin
    }
}

impl DerefMut for RustDynPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.plugin
    }
}

impl Debug for RustDynPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustDynPlugin")
            .field("library", &self.library)
            .field("plugin", &self.plugin.name())
            .finish()
    }
}

impl RustDynPlugin {
    /// Load a rust dynamic plugin from the specified path.
    /// # Unsafety
    /// Undefined behaviour expected if the symbol loaded from [CREATE_RDYN_SYM_NAME]
    /// does not match the function signature [CreateRDynPlugin]
    /// # Implementation
    /// This method is just a ease of use wrapper for the [load_rdyn_plugin] function.
    #[inline]
    #[allow(dead_code)]
    pub fn load_from(path: &str) -> Option<RustDynPlugin> {
        load_rdyn_plugin(path)
    }

    /// Tell rust not to release the library when it goes out of scope.
    /// # Use case
    /// Used when you do not want to store the library in the program,
    /// but you do not want the library to be freed.
    /// In this case you would "forget" the library to keep it loaded.
    #[inline]
    #[allow(dead_code)]
    pub fn forget_library(&self) {
        std::mem::forget(&self.library);
    }
}

/// Load a rust dynamic plugin from the specified path.
/// # Unsafety
/// Undefined behaviour expected if the symbol loaded from the symbol named 
/// [Create RDyn Plugin Symbol Name](CREATE_RDYN_SYM_NAME) within the loaded library
/// does not match the function signature [CreateRDynPlugin]
#[inline]
pub fn load_rdyn_plugin(path: &str) -> Option<RustDynPlugin> {
    let library = Library::new(path).ok()?;
    let create_plugin_sym: Symbol<CreateRDynPlugin> =
        unsafe { library.get(CREATE_RDYN_SYM_NAME) }.ok()?;
    let plugin = create_plugin_sym();
    Some(RustDynPlugin { library, plugin })
}
