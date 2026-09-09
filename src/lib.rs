use godot::prelude::*;

pub struct GodotVerseExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotVerseExtension {}
