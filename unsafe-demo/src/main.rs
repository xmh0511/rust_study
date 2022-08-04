use core::slice;
use std::{
    alloc::{alloc, Layout},
    ops::DerefMut, any::Any,
};

mod utilities;

#[derive(Debug)]
struct Data {
    v: i32,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("destroy");
    }
}

fn show<'a, 'b>(s0: &'a Data, s1: &'b String) -> &'b str
where
    'b: 'a,
{
    //    let s = &s0.s;
    //    let f = &s[0..1];
    unsafe {
        let mut vu8 = vec![b'a', b'b', b'c'];
        let ptr = &vu8 as *const Vec<u8>;
        let mptr = ptr as *mut Vec<u8>;
        std::str::from_utf8_unchecked_mut(&mut (*mptr)[..])
    }
}

// struct Test{
// 	i:i32,
// 	ptr:* mut i32
// }

// impl Test {
// 	const fn new(v:i32)->Self{
//        unsafe{
// 		  let layout = Layout::new::<i32>();
// 		  let ptr = alloc(layout);
// 		  Self{
// 			i:10,
// 			ptr: ptr as * mut i32
// 		  }
// 	   }
// 	}
// }

impl From<i32> for Data {
    fn from(d: i32) -> Self {
        Data { v: d }
    }
}

fn instance() -> &'static mut Vec<Data> {
    static mut Global: Vec<Data> = Vec::new();
    unsafe { &mut Global }
}

fn main() {

    let mut r = utilities::Array::new(6);
    for i in 0..=5 {
        r[i] = i as i32 + 1;
    }
    for ele in &mut r {
        *ele = *ele + 1;
        // println!("{}", ele);
    }

    for i in 0..=5 {
        println!("{}", r[i]);
    }

    println!("------------------------------");

    let r1 = &r[1..3];
    //println!("{}",r1.len());
    for ele in r1 {
        println!("{ele}");
    }

    let r2 = &mut r[2..5];

    let dd = r;
    let v = vec![0,1,2];
    let c = &v[0..1];
}
