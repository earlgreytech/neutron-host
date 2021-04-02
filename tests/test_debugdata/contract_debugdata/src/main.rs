//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use no_std_compat::ptr;
//use neutron_star::*;
extern crate panic_halt;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    /*
    let msg = "Hello Costack!";
    __push_costack(msg.as_ptr(), msg.len());
    
    
    let msg = "Hello Costack trough logger!";
    __push_costack(msg.as_ptr(), msg.len());
    let count = 1 as u8;
    let count_ptr: *const u8 = &count;
    __push_costack(count_ptr, 1);
    __system_call(4, 1);
    */
    
    __system_call(0x8000_0001, 0);
    
    //let msg = "Hello Costack!";
    //__push_costack(msg.as_ptr(), msg.len());
    //__system_call(0x8000_0001, 2);
    __system_call(0x8000_0001, 1);
    
    let mut array: [u8; 25] = [0; 25];
    __pop_costack(array.as_mut_ptr(), array.len());
    __push_costack(array.as_ptr(), array.len());
    
    __system_call(0x8000_0001, 2);
    __exit(5);
}
