/* build.rs - build process
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){

    //build IT8951 
    let dst =cmake::Config::new("./")
        .no_build_target(true)
        //.out_dir(out_path)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=IT8951");
    x11();
    epaper();
}
fn x11(){
    if cfg!(feature = "dox") {
        return;
    }
    let deps = [
        ("gl", "1", "glx"),
        ("x11", "1.4.99.1", "xlib"),
        ("x11-xcb", "1.6", "xlib_xcb"),
        ("xcursor", "1.1", "xcursor"),
        ("xext", "1.3", "dpms"),
        ("xft", "2.1", "xft"),
        ("xi", "1.7", "xinput"),
        ("xinerama", "1.1", "xinerama"),
        ("xmu", "1.1", "xmu"),
        ("xrandr", "1.5", "xrandr"),
        ("xrender", "0.9.6", "xrender"),
        ("xpresent", "1", "xpresent"),
        ("xscrnsaver", "1.2", "xss"),
        ("xt", "1.1", "xt"),
        ("xtst", "1.2", "xtst"),
        ("xxf86vm", "1.1", "xf86vmode"),
    ];

    for &(dep, version, feature) in deps.iter() {
        let var = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));
        if env::var_os(var).is_none() {
            continue;
        }
        pkg_config::Config::new()
            .atleast_version(version)
            .probe(dep)
            .unwrap();
    }
}
fn epaper(){

    //link IT8951 static libary
    println!("cargo:rustc-link-search=IT8951-ePaper/Raspberry/lib");
    println!("cargo:rustc-link-search={}{}",env::var("OUT_DIR").unwrap(),"/build");
    println!("cargo:rustc-link-lib=IT8951");
    println!("cargo:rustc-link-lib=bcm2835");

    //bind IT8951
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
