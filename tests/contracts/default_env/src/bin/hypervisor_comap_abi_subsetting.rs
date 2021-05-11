//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
extern crate panic_halt;

// 3 subsets of 15 bytes each
const SUBSET_COUNT: usize = 3;
const SUBSET_SIZE: usize = 15;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

// This contract uses a key (Pushed several times) on costack to repetedly peek subsets of a codata entry (to the costack)
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    __system_call(DEBUG_DATA_FEATURE, 3); // DebugDataFunctions::PushResultMap
    
    // Push keys to costack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    __forward_input_costack(); // Injected input stack -> input for peek_result_comap in loop
    
    // Peek all the subsets of the comap value to the costack
    let mut abi_data: [u32; SUBSET_COUNT] = [0; SUBSET_COUNT];
    for i in 0..SUBSET_COUNT {
        abi_data[i] = __peek_result_comap(SUBSET_SIZE * i, SUBSET_SIZE);
    }
    __forward_input_costack(); // Result input from peek_result_comap in loop -> Output stack for assertion
    
    // Push ABI data to stack
    for i in 0..SUBSET_COUNT {
        let abi_data_buf = u32::to_ne_bytes(abi_data[i]);
        __push_costack(abi_data_buf.as_ptr(), 4);
    }

    // Assert the result of the comap subset peeking
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack
    
    __exit(5);
}
