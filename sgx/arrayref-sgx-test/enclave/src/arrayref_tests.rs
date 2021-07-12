use arrayref::*;

use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

//#[test]
pub fn test_arrayref() {
    let foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bar = array_ref!(foo, 1, 11);
    println!("I am checking that I can dereference bar[0] = {}", bar[0]);
}
