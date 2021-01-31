extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "event-sink")]
    {
        println!("cargo:rerun-if-changed=event_sink.c");
        println!("cargo:rerun-if-changed=event_sink.h");
    }

    #[cfg(feature = "invocation-listener")]
    {
        println!("cargo:rerun-if-changed=invocation_listener.c");
        println!("cargo:rerun-if-changed=invocation_listener.h");
    }

    println!(
        "cargo:rustc-link-search={}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    println!("cargo:rustc-link-lib=frida-gum");
    println!("cargo:rustc-link-lib=pthread");

    let bindings = bindgen::Builder::default()
        .header("frida-gum.h")
        .header("event_sink.h")
        .header("invocation_listener.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();

    #[cfg(feature = "event-sink")]
    cc::Build::new()
        .file("event_sink.c")
        .warnings_into_errors(true)
        .compile("event_sink");

    #[cfg(feature = "invocation-listener")]
    cc::Build::new()
        .file("invocation_listener.c")
        .warnings_into_errors(true)
        .compile("invocation_listener");
}
