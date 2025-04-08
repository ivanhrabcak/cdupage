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
                        .map(OsStr::to_str)
                        .flatten()
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
        use std::path::Path;

        use cbindgen::Style;
        let clang = cbindgen::Builder::new();
        clang
            .with_crate(env!("CARGO_MANIFEST_DIR"))
            .with_documentation(true)
            .with_pragma_once(true)
            .with_language(cbindgen::Language::C)
            .with_style(Style::Type)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("bindings/bindings.h");
    }
}
