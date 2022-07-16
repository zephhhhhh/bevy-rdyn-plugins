//! # Bevy Rusty Dynamic Plugins
//! Bevy Rusty Dynamic Plugins is an extension for the [Bevy game engine](https://bevyengine.org)
//! that allows dynamic plugin loading using the Rust ABI rather than the C ABI used in the
//! standard bevy implementation of [Dynamic Plugins](https://docs.rs/bevy/latest/bevy/app/derive.DynamicPlugin.html)
//! 
//! The Rust ABI is not stable and as such this extension may also be unstable on some platforms and in some use cases,
//! However from testing it is a lot more stable than the standard bevy implementation and also seeks to be more documented.
//! 
//! # Host example
//! 
//! ## main.rs
//! ```
//! use bevy::prelude::*;
//! use bevy_rdyn_plugins::*;
//!
//! fn main() {
//!    App::new()
//!         .load_mods("mods")
//!         .run();
//! }
//! ```
//! 
//! With that minimal effort you can have dynamic mod loading!
//! 
//! It is worth remembering however that any library that stores state in a global manner,
//! such as [Log](https://docs.rs/log/latest/log/) or [Bevy log](https://docs.rs/bevy/latest/bevy/log/index.html)
//! will not function correctly unless you initialise them again in the dynamic plugin or write an FFI safe wrapper to transfer
//! state across to the new dynamically loaded library.
//! This is due to the fact that global objects will not cross FFI or dynamic library boundaries.
//! Further explaination of the effects this has and how to work around it can be found [here](https://github.com/rust-lang/log/issues/421#issuecomment-829496956)
//! 
//! If you would like to enable plugins to use functionality from your program, it is a good idea to create
//! either a crate that exposes APIs for plugins to use, or add a lib.rs to your program that does the same,
//! It is also a good idea to re-export the bevy_rdyn_plugins library from the lib.rs file along with some other 
//! dependencies such as [Bevy](https://bevyengine.org) itself for the sake of ergonomics when developing a plugin.
//! 
//! ## lib.rs
//! ```
//! pub use bevy;
//! pub use bevy_rdyn_plugins;
//! ```
//! 
//! A prelude file can also be a good idea when you can use wildcard includes or specific includes to make working
//! with the API even easier. Don't forget to export this from your lib.rs also!
//! 
//! ## lib.rs
//! ```
//! pub use bevy;
//! pub use bevy_rdyn_plugins;
//! 
//! pub mod prelude;
//! ```
//! 
//! ## prelude.rs
//! ```
//! pub use bevy;
//! pub use bevy_rdyn_plugins;
//! 
//! pub use bevy::prelude::*;
//! pub use bevy_rdyn_plugins::*;
//! ```
//! 
//! # Plugin example
//! 
//! ## Cargo.toml
//! ```
//! [lib]
//! crate-type = ["dylib"]
//! 
//! [dependencies]
//! your_app = { path = "path/to/your/app" }
//! ```
//! 
//! The 'crate-type' specifies that the compiled binary should be built into a dynamic library.
//! 
//! ## lib.rs
//! ```
//! use your_app::prelude::*;
//! 
//! #[derive(RDynPlugin)]
//! pub struct ExamplePlugin;
//! 
//! impl Plugin for ExamplePlugin {
//!     fn build(&self, app: &mut App) {
//!         app.add_system(Self::on_update);
//!     }
//! }
//! 
//! impl ExamplePlugin {
//!     pub fn on_update(time: Res<Time>) {
//!         let current_time = time.seconds_since_startup();
//!         let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();
//!
//!         if at_interval(0.5) {
//!            println!("Hello from Example Plugin!");
//!         }
//!     }
//! }
//! 
//! ```
//! This example plugin prints a hello message every half a second using the 
//! [Time](https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html)
//! resource from the application, just like any other plugin you write in Bevy would!
//! 
//! This is because you are writing a normal Bevy plugin! The only exception is the 
//! ```
//! derive(RDynPlugin)
//! ```
//! neccessary to setup your file to be loaded by the host program. 
//! Dynamic plugins do require some thought when using global state libraries as stated earlier, 
//! this isn't very difficult to deal with however.
//! 
//! Here is the same example but using [Bevy log](https://docs.rs/bevy/latest/bevy/log/index.html) instead.
//! 
//! ## lib.rs
//! ```
//! use your_app::prelude::*;
//! 
//! #[derive(RDynPlugin)]
//! pub struct ExamplePlugin;
//! 
//! impl Plugin for ExamplePlugin {
//!     fn build(&self, app: &mut App) {
//!         // Required to initialise the global state for
//!         // logging on the newly loaded library, since the state
//!         // for logging is stored globally and will not transfer
//!         // across FFI boundaries.
//!         app.add_plugin(bevy::log::LogPlugin);
//!         app.add_system(Self::on_update);
//!     }
//! }
//! 
//! impl ExamplePlugin {
//!     pub fn on_update(time: Res<Time>) {
//!         let current_time = time.seconds_since_startup();
//!         let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();
//!
//!         if at_interval(0.5) {
//!             info!("Hello from Example Plugin!");
//!         }
//!     }
//! }
//! 
//! ```
//! 
//! Since we initialised the bevy logging on the newly loaded library, the logging will now work as normal.
//! We could also have transferred the global state across from the host to the guest without initialising
//! a seperate new state, however this is not always neccessary and is more effort to do so.

pub use rdyn_plugins::*;
pub use rdyn_plugins_macros::*;