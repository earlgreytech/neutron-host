//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
extern crate panic_halt;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

// This contract takes a key-value pair and an abi data on the input costack and pushes it to the output comap
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Push provided input stack to codata input stack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    
    // Pop ABI data bytes and cast to u32
    let mut abi_data_buf: [u8; 4] = [0; 4]; 
    let _ = __pop_costack(abi_data_buf.as_mut_ptr(), 4);
    __move_input_to_output_costack(); // The rest of the injected input stack -> Input for push_comap
    
    let abi_data = u32::from_ne_bytes(abi_data_buf); 
    
    // Push key/value from costack to comap
    __push_comap(abi_data);
    
    __system_call(DEBUG_DATA_FEATURE, 4); // DebugDataFunctions::AssertOutputMap
    
    __exit(5);
}
