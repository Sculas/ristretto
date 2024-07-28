#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::attributes::ModuleAccessFlags;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let data = data.to_vec();
    let mut cursor = Cursor::new(data);
    let _ = ModuleAccessFlags::from_bytes(&mut cursor);
});