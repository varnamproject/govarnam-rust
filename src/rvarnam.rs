use crate::bindings::v_array::{Suggestion, VArray};
use crate::bindings::varnam::*;

use std::ffi::{c_int, CStr, CString};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct Varnam {
    handle_id: c_int,
}

impl Varnam {
    pub fn get_version() -> String {
        unsafe {
            let version = varnam_get_version();
            if version.is_null() {
                String::new()
            } else {
                CStr::from_ptr(version).to_string_lossy().to_string()
            }
        }
    }

    pub fn get_build() -> String {
        unsafe {
            let build_version = varnam_get_build();
            if build_version.is_null() {
                String::new()
            } else {
                CStr::from_ptr(build_version).to_string_lossy().to_string()
            }
        }
    }

    pub fn get_last_error(&self) -> String {
        unsafe {
            let error_string = varnam_get_last_error(self.handle_id);
            if error_string.is_null() {
                String::new()
            } else {
                CStr::from_ptr(error_string).to_string_lossy().to_string()
            }
        }
    }

    pub fn init<T: AsRef<Path>>(vst_file: T, learning_file: T) -> Result<Self, Error> {
        let vst_path = vst_file.as_ref();
        if !vst_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The path provided for the Vst file is invalid",
            ));
        }

        let vst_str = vst_path.to_string_lossy();
        let learning_str = learning_file.as_ref().to_string_lossy();

        let c_vst = CString::new(vst_str.as_bytes())
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let c_learning = CString::new(learning_str.as_bytes())
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        let mut handle_id: c_int = 0;
        let status = unsafe {
            varnam_init(
                c_vst.as_ptr(),
                c_learning.as_ptr(),
                &mut handle_id as *mut c_int,
            )
        };

        if status != 0 || handle_id == 0 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to initialize Varnam handle (status: {})", status),
            ));
        }

        Ok(Varnam { handle_id })
    }

    pub fn transliterate<T: AsRef<str>>(&self, word: T) -> Vec<Suggestion> {
        let id: c_int = 1;
        let c_word = match CString::new(word.as_ref()) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let mut varray = match VArray::new() {
            Some(va) => va,
            None => return Vec::new(),
        };

        let status = unsafe {
            varnam_transliterate(
                self.handle_id,
                id,
                c_word.as_ptr(),
                varray.as_raw_mut_ptr(),
            )
        };

        if status != 0 {
            return Vec::new();
        }

        varray.extract_suggestions()
    }
}

impl Drop for Varnam {
    fn drop(&mut self) {
        if self.handle_id != 0 {
            unsafe { varnam_close(self.handle_id) }
        }
    }
}
