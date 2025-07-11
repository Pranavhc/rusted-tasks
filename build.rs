fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=advapi32");
        println!("cargo:warning=Linked advapi32 on Windows");
    }
}
// This build script links the `advapi32` library on Windows platforms.
// idk why this is necessary, it wasn't before, but with the latest rust version it is.
