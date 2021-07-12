use arrayref::*;

use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};


pub fn test_arrayref() {
    let foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bar = array_ref!(foo, 1, 10);
    println!("I am checking that I can dereference bar[0] = {}", bar[0]);
}

pub fn simple_case_works() {
    fn check(expected: [u8; 3], actual: &[u8; 3]) {
        for (e, a) in (&expected).iter().zip(actual.iter()) {
            assert_eq!(e, a)
        }
    }
    let mut foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    {
        let bar = array_ref!(foo, 2, 3);
        check([2, 3, 4], bar);
    }
    check([0, 1, 2], array_ref!(foo, 0, 3));
    fn zero2(x: &mut [u8; 2]) {
        x[0] = 0;
        x[1] = 0;
    }
    zero2(array_mut_ref!(foo, 8, 2));
    check([0, 0, 10], array_ref!(foo, 8, 3));
}

pub fn test_5_array_refs() {
    let mut data: [usize; 128] = [0; 128];
    for i in 0..128 {
        data[i] = i;
    }
    let data = data;
    let (a,b,c,d,e) = array_refs!(&data, 1, 14, 3, 100, 10);
    assert_eq!(a.len(), 1 as usize);
    assert_eq!(b.len(), 14 as usize);
    assert_eq!(c.len(), 3 as usize);
    assert_eq!(d.len(), 100 as usize);
    assert_eq!(e.len(), 10 as usize);
    assert_eq!(a, array_ref![data, 0, 1]);
    assert_eq!(b, array_ref![data, 1, 14]);
    assert_eq!(c, array_ref![data, 15, 3]);
    assert_eq!(e, array_ref![data, 118, 10]);
}

pub fn test_5_array_refs_dotdot() {
    let mut data: [usize; 128] = [0; 128];
    for i in 0..128 {
        data[i] = i;
    }
    let data = data;
    let (a,b,c,d,e) = array_refs!(&data, 1, 14, 3; ..; 10);
    assert_eq!(a.len(), 1 as usize);
    assert_eq!(b.len(), 14 as usize);
    assert_eq!(c.len(), 3 as usize);
    assert_eq!(d.len(), 100 as usize);
    assert_eq!(e.len(), 10 as usize);
    assert_eq!(a, array_ref![data, 0, 1]);
    assert_eq!(b, array_ref![data, 1, 14]);
    assert_eq!(c, array_ref![data, 15, 3]);
    assert_eq!(e, array_ref![data, 118, 10]);
}

pub fn test_5_mut_xarray_refs() {
    let mut data: [usize; 128] = [0; 128];
    {
        // temporarily borrow the data to modify it.
        let (a,b,c,d,e) = mut_array_refs!(&mut data, 1, 14, 3, 100, 10);
        assert_eq!(a.len(), 1 as usize);
        assert_eq!(b.len(), 14 as usize);
        assert_eq!(c.len(), 3 as usize);
        assert_eq!(d.len(), 100 as usize);
        assert_eq!(e.len(), 10 as usize);
        *a = [1; 1];
        *b = [14; 14];
        *c = [3; 3];
        *d = [100; 100];
        *e = [10; 10];
    }
    assert_eq!(&[1;1], array_ref![data, 0, 1]);
    assert_eq!(&[14;14], array_ref![data, 1, 14]);
    assert_eq!(&[3;3], array_ref![data, 15, 3]);
    assert_eq!(&[10;10], array_ref![data, 118, 10]);
}

pub fn test_5_mut_xarray_refs_with_dotdot() {
    let mut data: [usize; 128] = [0; 128];
    {
        // temporarily borrow the data to modify it.
        let (a,b,c,d,e) = mut_array_refs!(&mut data, 1, 14, 3; ..; 10);
        assert_eq!(a.len(), 1 as usize);
        assert_eq!(b.len(), 14 as usize);
        assert_eq!(c.len(), 3 as usize);
        assert_eq!(d.len(), 100 as usize);
        assert_eq!(e.len(), 10 as usize);
        *a = [1; 1];
        *b = [14; 14];
        *c = [3; 3];
        *e = [10; 10];
    }
    assert_eq!(&[1;1], array_ref![data, 0, 1]);
    assert_eq!(&[14;14], array_ref![data, 1, 14]);
    assert_eq!(&[3;3], array_ref![data, 15, 3]);
    assert_eq!(&[10;10], array_ref![data, 118, 10]);
}