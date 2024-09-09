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

        // https://github.com/Aleph-Alpha/ts-rs/issues/133
        let exports: Vec<_> = fs::read_dir("./bindings")
            .unwrap()
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
