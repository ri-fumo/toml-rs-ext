# toml-rs-ext
A simple Toml IO GDExtension with Rust. (Just a temporary solution)

### How to use?
Just create a file like this with an extension `.gdextension` in your project.

	[configuration]
	entry_symbol = "toml_rs_ext_init"
	compatibility_minimum = 4.1
	reloadable = true

	[libraries]
	<OS>.debug.<platform> = "res://path/to/debug/binary"
	<OS>.release.<platform> = "res://path/to/release/binary"
