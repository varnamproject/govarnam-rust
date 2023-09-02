fn main() {
    println!("Starting build script");
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=dylib=govarnam");
}
