//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
//use neutron_star::*;
extern crate panic_halt;

// We need to provide a fixed upper limit for popped stack items
const MAX_STACK_ITEM_SIZE: usize = 100;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Check if debug data ElementAPI is available
    __system_call(DEBUG_DATA_FEATURE, 0); // DebugDataFunctions::Available

    // Get length of provided input stack
    __system_call(DEBUG_DATA_FEATURE, 10); // DebugDataFunctions::GetInputStackLen
    let mut stack_size_buf: [u8; 1] = [0; 1];
    let _ = __pop_costack(stack_size_buf.as_mut_ptr(), 1);
    let stack_size = stack_size_buf[0];

    // Reverse order of provided input stack to counter the order flipping used by the next code section
    __system_call(DEBUG_DATA_FEATURE, 11); // DebugDataFunctions::ReverseInputStack

    // Mirror provided input stack to the output stack
    // Note: Pushing/popping item-wise like we do will flip the order of the stack,
    // but since we just flipped the order of the input stack it all works out as expected
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    for _i in 0..stack_size {
        let mut stack_item_buf: [u8; MAX_STACK_ITEM_SIZE] = [0; MAX_STACK_ITEM_SIZE];
        let actual_size = __pop_costack(stack_item_buf.as_mut_ptr(), MAX_STACK_ITEM_SIZE);
        __push_costack(stack_item_buf.as_ptr(), actual_size);
    }

    // Assert that the state of our output stack matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
