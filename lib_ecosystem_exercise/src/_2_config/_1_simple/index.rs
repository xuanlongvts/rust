use config::{Config, Environment, File};
use std::collections::HashMap;

pub fn main() {
	let settings = Config::builder()
		.add_source(File::with_name("src/_2_config/_1_simple/Settings"))
		.add_source(Environment::with_prefix("APP"))
		.build()
		.unwrap();

	// {"key": "189rjfadoisfj8923fjio", "debug": "false", "priority": "32"}
	println!(
		"{:?}",
		settings
			.try_deserialize::<HashMap<String, String>>()
			.unwrap()
	);
}
