use govarnam::Varanam;

#[test]
pub fn test_version() {
    let version = Varanam::get_version();
    assert_eq!(version, "1.9.0");
}

#[test]
pub fn test_build() {
    let build_version = Varanam::get_build();
    assert!(build_version.contains("1.9.0"));
}

#[test]
pub fn test_init() {
    let vst_file = "assets/ml/ml.vst";
    let learning_file = "assets/ml/check.vst";
    let varanam = Varanam::init(vst_file, learning_file).unwrap();
    let result = varanam.transliterate("morning");
    assert_eq!(result.len(), 10);
}
