//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
use neutron_star::*;
extern crate panic_halt;


#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    println!("Hello smoke test! {}, {}", 0, 2);
    __exit(5);
}
