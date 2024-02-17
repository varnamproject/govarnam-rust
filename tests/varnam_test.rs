use govarnam::Varnam;
use once_cell::sync::Lazy;

const varnam_client: Lazy<Varnam> =
    Lazy::new(|| Varnam::init("assets/ml/ml.vst", "assets/ml/check.vst", 22).unwrap());

#[test]
pub fn test_version() {
    let version = Varnam::get_version();
    assert_eq!(version, "1.9.0");
}

#[test]
pub fn test_build() {
    let build_version = Varnam::get_build();
    assert!(build_version.contains("1.9.0"));
}

#[test]
pub fn test_init() {
    let vst_file = "assets/ml/ml.vst";
    let learning_file = "assets/ml/check.vst";
    //let varnam = Varnam::init(vst_file, learning_file, 22).unwrap();
    let result = varnam_client.transliterate("morning");
    //let varnam_bruh = Varnam::init(vst_file, learning_file, 23).unwrap();
    //let result = varnam_bruh.transliterate("morning");
    assert_eq!(result.len(), 10);
    //drop(varnam);
}

#[test]
pub fn test_learn_unlearn() {
    let varnam = Varnam::init("assets/ml/ml.vst", "assets/ml/check.vst", 24).unwrap();
    //let varnam_sc = varnam.learn("മലയാളം", 6);
    //assert_eq!(varnam_sc, 0);
}

#[test]
pub fn test_import_export() {
    //let varnam = Varnam::init("assets/ml/ml.vst", "assets/ml/check.vst", 25).unwrap();
    let varnam = varnam_client;
    for word in ["മലയാളം", "ഹിന്ദി", "ഇംഗ്ലീഷ്"] {
        varnam.learn(word, 6);
    }

    let varnam_sc = varnam.export("assets/vlf-export", 10).unwrap();
    assert_eq!(varnam_sc, 0);

    let varnam_sc = varnam.import("assets/vlf-export-1.vlf").unwrap();
    assert_eq!(varnam_sc, 0);
}
