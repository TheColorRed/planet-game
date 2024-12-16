use godot::prelude::*;

pub use celestial_bodies::Planet;

struct PlanetExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PlanetExtension {}
