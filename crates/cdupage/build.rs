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
}
