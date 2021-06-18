//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
extern crate panic_halt;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    __self_address();

    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(0);
}
