use crate::bindings::v_array::{varray_t, Suggestion_t};

use super::bindings::varnam::*;
use std::io::Error;
use std::io::ErrorKind;
use std::{
    ffi::{c_int, CStr, CString},
    path::Path,
};

// #[derive(Debug, Clone)]
// pub struct VarnamError {
//     message: String,
// }

pub struct Varnam {
    handle_id: c_int,
}

impl Varnam {
    pub fn get_version() -> String {
        unsafe {
            let version = varnam_get_version();
            CStr::from_ptr(version).to_string_lossy().to_string()
        }
    }

    pub fn get_build() -> String {
        unsafe {
            let build_version = varnam_get_build();
            CStr::from_ptr(build_version).to_string_lossy().to_string()
        }
    }

    pub fn get_last_error(&self) -> String {
        unsafe {
            let error_string = varnam_get_last_error(self.handle_id);
            CStr::from_ptr(error_string).to_string_lossy().to_string()
        }
    }

    pub fn init<T: AsRef<Path>>(vst_file: T, learning_file: T, id: i32) -> Result<Self, Error> {
        if !vst_file.as_ref().exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The path provided for the Vst file is invalid",
            ));
        }

        let vst_file = vst_file.as_ref().to_string_lossy().to_string();
        let learning_file = learning_file.as_ref().to_string_lossy().to_string();
        unsafe {
            let _init_id = varnam_init(
                vst_file.as_ptr() as *const i8,
                learning_file.as_ptr() as *const i8,
                &id,
            );
        };
        // TODO: check error use init_id
        Ok(Varnam { handle_id: id })
    }

    pub fn transliterate<T: AsRef<str>>(&self, word: T) -> Vec<Suggestion_t> {
        let id: c_int = 1;
        let word = CString::new(word.as_ref()).unwrap();
        let mut varray_ptr = varray_t::init();
        unsafe { varnam_transliterate(self.handle_id, id, word.as_ptr(), &mut varray_ptr) };
        let varray_pointer = unsafe { *varray_ptr as varray_t };
        varray_pointer.into()
    }

    pub fn import<T: AsRef<Path>>(&self, path: T) -> Result<i32, Error> {
        if !path.as_ref().is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The path provided to export the learning file is invalid.",
            ));
        }
        let import_path = path.as_ref().to_string_lossy().to_string();
        let varnam_response =
            unsafe { varnam_import(self.handle_id, import_path.as_ptr() as *const i8) };
        Ok(varnam_response)
    }

    pub fn export<T: AsRef<Path>>(&self, path: T, words_per_file: i8) -> Result<i32, Error> {
        // Check if the parent directory exists.
        if path
            .as_ref()
            .parent()
            .map_or_else(
                || {
                    Err(Error::new(
                        ErrorKind::InvalidInput,
                        "The path provided to export the learning file is invalid.",
                    ))
                },
                |x| Ok(x),
            )?
            .is_dir()
        {
            let export_path = path.as_ref().to_string_lossy().to_string();
            let varanam_response = unsafe {
                varnam_export(
                    self.handle_id,
                    export_path.as_ptr() as *const i8,
                    words_per_file as c_int,
                )
            };
            println!(" Checking bad status code:{}", varanam_response);
            Ok(varanam_response)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "The directory provided to export the learning file does not exists.",
            ))
        }
    }

    pub fn debug(&self, value: bool) {
        unsafe { varnam_debug(self.handle_id, value as c_int) };
    }

    pub fn learn<T: AsRef<str>>(&self, word: T, weight: i32) -> i32 {
        unsafe { varnam_learn(self.handle_id, word.as_ref().as_ptr() as *const i8, weight) }
    }
}

impl Drop for Varnam {
    fn drop(&mut self) {
        unsafe { varnam_close(self.handle_id) }
    }
}
