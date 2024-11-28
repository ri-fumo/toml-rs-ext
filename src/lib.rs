mod document;


use ::godot::init::*;


struct TomlRsExt;

#[gdextension(entry_point = toml_rs_ext_init)]
unsafe impl ExtensionLibrary for TomlRsExt {}
