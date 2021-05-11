//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
extern crate panic_halt;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

// This contract takes a key-value pair on the input costack and pushes it to the output comap
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Push provided input stack to codata input stack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    __forward_input_costack(); // Injected input stack -> Input for push_raw_comap
    
    // Push key/value from costack to comap
    __push_raw_comap();
    
    __system_call(DEBUG_DATA_FEATURE, 4); // DebugDataFunctions::AssertOutputMap
    
    __exit(5);
}
