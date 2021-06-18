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
const ERR_STR_STACKITEMTOOLARGE: &str = "StackItemTooLarge (Placeholder name - The costack array wasn't aligned to the requested data type)";
const ERR_STR_UNHANDLED: &str = "Unhandled";

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // Reverse the order of the provided stack, which together with the flipped order caused by
    // the following pushing/popping will leave us with the original order to compare!
    __system_call(DEBUG_DATA_FEATURE, 11); // DebugDataFunctions::ReverseInputStack
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack

    // Initialize arrays that will recieve the popped values before pushing
    let mut array_u8: [u8; 5] = [0; 5];
    let mut array_u16: [u16; 5] = [0; 5];
    let mut array_u32: [u32; 5] = [0; 5];
    let mut array_u64: [u64; 5] = [0; 5];
    let mut array_i8: [i8; 5] = [0; 5];
    let mut array_i16: [i16; 5] = [0; 5];
    let mut array_i32: [i32; 5] = [0; 5];
    let mut array_i64: [i64; 5] = [0; 5];
    let mut array_address: [NeutronAddress; 5] = [NeutronAddress::default(); 5];

    match pop_costack_fixed_array_u8(&mut array_u8) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_u8(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_u8(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_u8(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_u8: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_u16(&mut array_u16) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_u16(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_u16(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_u16(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_u16: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_u32(&mut array_u32) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_u32(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_u32(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_u32(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_u32: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_u64(&mut array_u64) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_u64(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_u64(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_u64(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_u64: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_i8(&mut array_i8) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_i8(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_i8(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_i8(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_i8: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_i16(&mut array_i16) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_i16(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_i16(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_i16(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_i16: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_i32(&mut array_i32) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_i32(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_i32(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_i32(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_i32: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_i64(&mut array_i64) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_i64(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_i64(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_i64(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_i64: {}", ERR_STR_UNHANDLED),
    }
    match pop_costack_fixed_array_address(&mut array_address) {
        Ok(5) => {}
        Ok(actual_size) => println!("Error in pop_costack_fixed_array_address(): Expected array size 5, got {}", actual_size),
        Err(RecoverableError::ItemDoesntExist) => println!("Error in pop_costack_fixed_array_address(): {}", ERR_STR_ITEMDOESNTEXIST),
        Err(RecoverableError::StackItemTooLarge) => println!("Error in pop_costack_fixed_array_address(): {}", ERR_STR_STACKITEMTOOLARGE),
        _ => println!("Error in pop_costack_fixed_array_address: {}", ERR_STR_UNHANDLED),
    }

    push_costack_array_u8(&array_u8);
    push_costack_array_u16(&array_u16);
    push_costack_array_u32(&array_u32);
    push_costack_array_u64(&array_u64);
    push_costack_array_i8(&array_i8);
    push_costack_array_i16(&array_i16);
    push_costack_array_i32(&array_i32);
    push_costack_array_i64(&array_i64);
    push_costack_array_address(&array_address);

    // Assert that the state of our output stack matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
