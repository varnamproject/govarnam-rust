#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::size_t;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::fmt::{Debug, Display};

#[link(name = "govarnam")]
extern "C" {
    fn varray_init() -> *mut varray_t;
    fn varray_push(array: *const varray_t, data: *const c_void);
    fn varray_get(array: *const varray_t, index: c_int) -> *mut c_void;
    fn varray_length(array: *const varray_t) -> c_int;
    fn varray_is_empty(array: *const varray_t) -> bool;
    fn varray_clear(array: *const varray_t);
    fn varray_free(array: *mut varray_t);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct varray_t {
    memory: *mut *mut c_void,
    allocated: size_t,
    used: size_t,
    index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Suggestion_t {
    pub word: *mut c_char,
    pub weight: c_int,
    pub learned_on: c_int,
}

/// Safe Rust representation of a transliteration suggestion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub word: String,
    pub weight: i32,
    pub learned_on: i32,
}

impl Suggestion {
    pub(crate) fn from_raw(raw: &Suggestion_t) -> Self {
        let word = if raw.word.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(raw.word).to_string_lossy().into_owned() }
        };

        Self {
            word,
            weight: raw.weight,
            learned_on: raw.learned_on,
        }
    }
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.word)
    }
}

/// Safe RAII wrapper managing the lifetime of a C `varray_t` allocation
pub struct VArray {
    ptr: *mut varray_t,
}

impl VArray {
    pub fn new() -> Option<Self> {
        let ptr = unsafe { varray_init() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn as_raw_mut_ptr(&mut self) -> *mut *mut varray_t {
        &mut self.ptr
    }

    pub fn len(&self) -> usize {
        if self.ptr.is_null() {
            0
        } else {
            unsafe { varray_length(self.ptr) as usize }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn extract_suggestions(&self) -> Vec<Suggestion> {
        let count = self.len();
        let mut suggestions = Vec::with_capacity(count);

        for index in 0..count {
            let raw_item = unsafe { varray_get(self.ptr, index as c_int) };
            if !raw_item.is_null() {
                let sugg_raw = unsafe { &*(raw_item as *const Suggestion_t) };
                suggestions.push(Suggestion::from_raw(sugg_raw));
            }
        }

        suggestions
    }
}

impl Drop for VArray {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                varray_clear(self.ptr);
                varray_free(self.ptr);
            }
        }
    }
}
