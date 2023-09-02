# Govarnam-Rust

This Rust project provides an FFI wrapper to the govarnam shared library, enabling seamless integration and utilization of its language-related functionality within Rust applications.

## Installation

> note: This library assumes that the govarnam shared library is already installed and available on your system.

Add the library to your **Cargo.toml** file using the following command.
    
    cargo add --git https://github.com/varnamproject/govarnam-rust

## Usage
An example usages of this library.
```rs
use govarnam::Varanam;


fn main() {
    let varanam = Varanam::init(
        "schema/ml/ml.vst",
        "schema/learnings/learning-ml.vst",
    )
    .expect("Cannot initialize varnam");

    let results = varanam.transliterate("good morning");

    for item in results {
        println!(
            "Word: {}, Weight: {}, Learned on: {}",
            item.to_string(),
            item.weight,
            item.learned_on,
        );
    }
}
```
## License

Licensed under the terms of the [Mozilla Public License Version 2.0](LICENSE.txt)