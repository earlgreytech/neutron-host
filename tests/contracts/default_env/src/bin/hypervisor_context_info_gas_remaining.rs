//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
extern crate panic_halt;

// TODO: Import directly from testing harness!
const DEFAULT_TEST_GAS: u64 = 10000;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let gas_remaining_1 = __gas_remaining();
    let gas_remaining_2 = __gas_remaining();
    let gas_remaining_3 = __gas_remaining();

    // Error: One of the recieved gas remaining values was larger than the testing gas limit!
    if gas_remaining_1 >= DEFAULT_TEST_GAS || gas_remaining_2 >= DEFAULT_TEST_GAS || gas_remaining_3 >= DEFAULT_TEST_GAS {
        __exit(2);
    }
    // Error: One of the recieved gas remaining values was not smaller than a preceeding value!
    else if gas_remaining_3 >= gas_remaining_2 || gas_remaining_2 >= gas_remaining_1 {
        __exit(1);
    }
    // No error!
    else {
        __exit(0);
    }
}
