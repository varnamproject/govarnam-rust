fn main() {
    println!("Starting");
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=dylib=govarnam");
}
