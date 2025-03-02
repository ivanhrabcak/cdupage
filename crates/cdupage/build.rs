fn main() {
    #[cfg(feature = "node")]
    node_bindgen::build::configure();
    #[cfg(feature = "node-types")]
    {
        use std::{
            ffi::OsStr,
            fs::{self, File},
            io::Write,
        };

        let result = fs::read_dir("./bindings");

        if let Ok(bindings_dir) = result {
            // https://github.com/Aleph-Alpha/ts-rs/issues/133
            let exports: Vec<_> = bindings_dir
                .filter_map(Result::ok)
                .filter_map(|p| {
                    p.path()
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .map(str::to_owned)
                })
                .filter(|f| f != "index")
                .map(|f| format!("export * from \"./{}\"", f))
                .collect();

            let mut file = File::create("./bindings/index.ts").unwrap();
            file.write_all(exports.join("\n").as_bytes()).unwrap();
        }
    }
    #[cfg(feature = "c_any_other_lang")]
    {
        let dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("ffi/defs.h");
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
}
