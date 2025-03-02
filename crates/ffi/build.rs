use std::path::Path;

fn main() {
	
	let dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/ffi/binds.hpp");
	let config = cbindgen::Config {
		only_target_dependencies: true,
		pragma_once: true,
		using_namespaces: Some(vec!["std".to_string()]),
		usize_is_size_t: true,
		documentation: true,
		documentation_style: cbindgen::DocumentationStyle::Doxy,
		includes: vec![dir_path.to_str().unwrap().to_string()],
		language: cbindgen::Language::Cxx,
		..Default::default()
	};
	cbindgen::Builder::new()
		.with_crate(env!("CARGO_MANIFEST_DIR"))
		.with_cpp_compat(true)
		.with_std_types(true)
		.with_config(config)
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file("bindings/bindings.h");
}