//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
use neutron_star::*;
extern crate panic_halt;


#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let filename = "tests/example_new_element_file.txt";
    println!("Loading file: {}", filename);
    //NOTE: doing println! will clear costack, so make sure to not do any in between stack operations!
    __push_costack(filename.as_ptr(), filename.len());
    let len = __system_call(0x8000_0001, 1) as usize; //ReadFile
    let expected = "Hello World!!!";
    if len != expected.len(){
        __exit(1);
    }
    let mut data = [0; 30];
    let len2 = __pop_costack(data.as_mut_ptr(), 30) as usize;
    if len != len2{
        __exit(2);
    }
    println!("length of data: {}", len);
    println!("expected: {}", expected);
    let actual = core::str::from_utf8_unchecked(&data[0..len]);
    println!("actual: {}", actual);
    if expected == actual{
        __exit(0);
    }else{
        __exit(3);
    }
}
