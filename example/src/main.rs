use govarnam::Varnam;

fn main() {
    let varnam = Varnam::init(
        "./ml.vst", // Get this file from https://github.com/varnamproject/schemes
        "./ml.vst.learnings", // This file will be automatically created
    )
    .expect("Cannot initialize varnam");

    let results = varnam.transliterate("namaskkaaram");

    for item in results {
        println!(
            "Word: {}, Weight: {}, Learned on: {}",
            item.to_string(),
            item.weight,
            item.learned_on,
        );
    }
}
