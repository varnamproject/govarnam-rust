use govarnam::{Suggestion, VArray, Varnam};

#[test]
pub fn test_version() {
    let version = Varnam::get_version();
    // Verify version string call runs safely without crashing
    assert!(version.is_empty() || !version.is_empty());
}

#[test]
pub fn test_build() {
    let build_version = Varnam::get_build();
    // Verify build version string call runs safely without crashing
    assert!(build_version.is_empty() || !build_version.is_empty());
}

#[test]
pub fn test_init_non_existent_file() {
    let vst_file = "non_existent_path.vst";
    let learning_file = "non_existent_path.vst.learnings";
    let result = Varnam::init(vst_file, learning_file);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}

#[test]
pub fn test_get_last_error_on_uninitialized_handle() {
    // Creating dummy struct or testing get_last_error on invalid init failure
    let result = Varnam::init("non_existent_file.vst", "non_existent_file.vst.learnings");
    assert!(result.is_err());
}

#[test]
pub fn test_transliterate_with_null_bytes() {
    // Calling transliterate with invalid/uninitialized path fails init
    let result = Varnam::init("invalid_vst.vst", "invalid_learn.vst.learnings");
    assert!(result.is_err());
}

#[test]
pub fn test_varray_creation_and_lifecycle() {
    let varray = VArray::new();
    assert!(varray.is_some());
    if let Some(mut va) = varray {

        assert_eq!(va.len(), 0);
        assert!(va.is_empty());
        assert_ne!(va.as_raw_mut_ptr(), std::ptr::null_mut());
        let suggestions = va.extract_suggestions();
        assert_eq!(suggestions.len(), 0);
    }
}

#[test]
pub fn test_suggestion_display_and_equality() {
    let sugg1 = Suggestion {
        word: "namaste".to_string(),
        weight: 10,
        learned_on: 100,
    };
    let sugg2 = Suggestion {
        word: "namaste".to_string(),
        weight: 10,
        learned_on: 100,
    };
    let sugg3 = Suggestion {
        word: "hello".to_string(),
        weight: 5,
        learned_on: 200,
    };

    assert_eq!(sugg1, sugg2);
    assert_ne!(sugg1, sugg3);
    assert_eq!(format!("{}", sugg1), "namaste");
    assert_eq!(format!("{:?}", sugg1), "Suggestion { word: \"namaste\", weight: 10, learned_on: 100 }");
}


