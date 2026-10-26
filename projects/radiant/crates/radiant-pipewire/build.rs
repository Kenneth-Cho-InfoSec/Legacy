fn main() {
    let pipewire = pkg_config::Config::new()
        .atleast_version("0.3.65")
        .probe("libpipewire-0.3")
        .expect("PipeWire development files are required");
    let mut build = cc::Build::new();
    build.file("src/native.c");
    for include in pipewire.include_paths {
        build.include(include);
    }
    build.flag_if_supported("-std=c11");
    build.compile("radiant_pipewire_native");
    println!("cargo:rerun-if-changed=src/native.c");
}
