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
    
    // Initialize arrays that will recieve the popped values before pushing
    let mut array_u8: [u8; 5] = [0,0,0,0,0];
    let mut array_u16: [u16; 5] = [0,0,0,0,0];
    let mut array_u32: [u32; 5] = [0,0,0,0,0];
    let mut array_u64: [u64; 5] = [0,0,0,0,0];
    let mut array_i8: [i8; 5] = [0,0,0,0,0];
    let mut array_i16: [i16; 5] = [0,0,0,0,0];
    let mut array_i32: [i32; 5] = [0,0,0,0,0];
    let mut array_i64: [i64; 5] = [0,0,0,0,0];
    
    //let mut array_address: [i64; 5] = [0,0,0,0,0];
    
    // Pop values into array
    pop_costack_fixed_array_u8(&mut array_u8).unwrap();
    pop_costack_fixed_array_u16(&mut array_u16).unwrap();
    pop_costack_fixed_array_u32(&mut array_u32).unwrap();
    pop_costack_fixed_array_u64(&mut array_u64).unwrap();
    pop_costack_fixed_array_i8(&mut array_i8).unwrap();
    pop_costack_fixed_array_i16(&mut array_i16).unwrap();
    pop_costack_fixed_array_i32(&mut array_i32).unwrap();
    pop_costack_fixed_array_i64(&mut array_i64).unwrap();
    
    // Push array values for assertion
    push_costack_array_u8(&array_u8);
    push_costack_array_u16(&array_u16);
    push_costack_array_u32(&array_u32);
    push_costack_array_u64(&array_u64);
    push_costack_array_i8(&array_i8);
    push_costack_array_i16(&array_i16);
    push_costack_array_i32(&array_i32);
    push_costack_array_i64(&array_i64);

    // Assert that the state of our output stack matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack
    
    __exit(5);
}
