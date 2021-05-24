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
    __move_input_to_output_costack(); // Injected input stack -> input for peek_raw_result_comap in loop

    // Peek all the subsets of the comap value to the costack
    for i in 0..SUBSET_COUNT {
        __peek_raw_result_comap(SUBSET_SIZE * i, SUBSET_SIZE);
    }
    __move_input_to_output_costack(); // Result input from peek_raw_result_comap in loop -> Output stack for assertion

    // Assert the result of the comap subset peeking
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
