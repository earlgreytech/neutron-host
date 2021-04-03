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
    __system_call(0x8000_0001, 0); // DebugDataFunctions::Available

    __system_call(0x8000_0001, 1); // DebugDataFunctions::PushInputStack
    
    let mut array: [u8; 500] = [0; 500];
    let actual_size = __pop_costack(array.as_mut_ptr(), array.len());
    __push_costack(array.as_ptr(), actual_size);
    
    __system_call(0x8000_0001, 2); // DebugDataFunctions::AssertOutputStack
    __exit(5);
}
