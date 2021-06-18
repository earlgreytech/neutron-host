//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_common::*;
use neutron_star::syscalls::*;
use neutron_star::*;
use neutron_star_rt::*;
extern crate panic_halt;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

const ERR_STR_ITEMDOESNTEXIST: &str = "ItemDoesntExist (Failed to pop value from costack)";
const ERR_STR_STACKITEMTOOLARGE: &str = "StackItemTooLarge (The costack value was larger than this function's data type)";
const ERR_STR_STACKITEMTOOSMALL: &str = "StackItemTooSmall (The costack value was smaller than this function's data type)";
const ERR_STR_UNHANDLED: &str = "Unhandled";

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Reverse the order of the provided stack, which together with the flipped order caused by
    // the following pushing/popping will leave us with the original order to compare!
    __system_call(DEBUG_DATA_FEATURE, 11); // DebugDataFunctions::ReverseInputStack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack

    match pop_costack_u8() {
        Ok(v) => push_costack_u8(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_u8(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_u8(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_u8(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_u8(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_u16() {
        Ok(v) => push_costack_u16(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_u16(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_u16(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_u16(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_u16(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_u32() {
        Ok(v) => push_costack_u32(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_u32(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_u32(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_u32(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_u32(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_u64() {
        Ok(v) => push_costack_u64(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_u64(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_u64(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_u64(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_u64(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_i8() {
        Ok(v) => push_costack_i8(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_i8(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_i8(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_i8(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_i8(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_i16() {
        Ok(v) => push_costack_i16(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_i16(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_i16(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_i16(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_i16(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_i32() {
        Ok(v) => push_costack_i32(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_i32(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_i32(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_i32(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_i32(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_i64() {
        Ok(v) => push_costack_i64(v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_i64(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_i64(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_i64(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_i64(): {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_address() {
        Ok(v) => push_costack_address(&v),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_address(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_address(): {}", ERR_STR_STACKITEMTOOLARGE),
        Err(RecoverableError::StackItemTooSmall) => println!("Error in pop_costack_address(): {}", ERR_STR_STACKITEMTOOSMALL),
        _ => println!("Error in pop_costack_address(): {}", ERR_STR_UNHANDLED),
    }

    // Assert that the state of our output stack matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
