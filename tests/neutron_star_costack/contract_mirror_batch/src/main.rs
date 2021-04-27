//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
use neutron_star::syscalls::*;
extern crate panic_halt;


const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Reverse the order of the provided stack, which together with the flipped order caused by 
    // the following pushing/popping will leave us with the original order to compare!
    __system_call(DEBUG_DATA_FEATURE, 11); // DebugDataFunctions::ReverseInputStack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    
    push_costack_u64(pop_costack_u64().unwrap());
    push_costack_u32(pop_costack_u32().unwrap());
    push_costack_u16(pop_costack_u16().unwrap());
    push_costack_u8(pop_costack_u8().unwrap());
    push_costack_i64(pop_costack_i64().unwrap());
    push_costack_i32(pop_costack_i32().unwrap());
    push_costack_i16(pop_costack_i16().unwrap());
    push_costack_i8(pop_costack_i8().unwrap());
    push_costack_address(&pop_costack_address().unwrap());
    
    // Assert that the state of our output stack matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack
    
    __exit(5);
}
