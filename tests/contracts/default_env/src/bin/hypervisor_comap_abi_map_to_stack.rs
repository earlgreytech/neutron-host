//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
extern crate panic_halt;

// We need to provide a fixed starting point and size limit for data peeked from comap
const DATA_READ_START: usize = 0;
const MAX_RESULT_SIZE: usize = 25;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

// This contract takes a resulut comap entry (found by key popped from input costack) and pushes it to output costack
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    __system_call(DEBUG_DATA_FEATURE, 3); // DebugDataFunctions::PushResultMap

    // Load key into costack and use it as parameter for comap lookup
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    __move_input_to_output_costack(); // Injected input stack -> Input for peek_result_comap

    let abi_data = __peek_result_comap(DATA_READ_START, MAX_RESULT_SIZE);
    __move_input_to_output_costack(); // Result input from peek_result_comap -> Output stack for assertion

    // Push ABI data to stack
    let abi_data_buf = u32::to_ne_bytes(abi_data);
    __push_costack(abi_data_buf.as_ptr(), 4);

    // Assert the result of the comap lookup
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
