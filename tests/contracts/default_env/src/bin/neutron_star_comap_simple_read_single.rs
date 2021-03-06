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

const ERR_STR_ITEMDOESNTEXIST: &str = "ItemDoesntExist (Either it's actually missing, or the ABI of the read comap item didn't match the functions's expectation)";
const ERR_STR_STACKITEMTOOLARGE: &str = "StackItemTooLarge (The comap item was larger than this function's data type)";
const ERR_STR_STACKITEMTOOSMALL: &str = "StackItemTooSmall (The comap item was smaller than this function's data type)";
const ERR_STR_UNHANDLED: &str = "Unhandled";

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    __system_call(DEBUG_DATA_FEATURE, 3); // DebugDataFunctions::PushResultMap
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack

    // Pop ABI data bytes and cast to u32
    let mut abi_data_buf: [u8; 4] = [0; 4];
    let _ = __pop_costack(abi_data_buf.as_mut_ptr(), 4);
    let abi_data = u32::from_le_bytes(abi_data_buf);

    // NOTE: This was a match statement originally, but a bug was messing that up
    if abi_data == ABI_VALUE_U8 {
        match read_result_comap_u8(".namespace.keyname") {
            Ok(v) => push_costack_u8(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_u8(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_u8(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_u8(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_u8(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U16 {
        match read_result_comap_u16(".namespace.keyname") {
            Ok(v) => push_costack_u16(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_u16(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_u16(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_u16(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_u16(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U32 {
        match read_result_comap_u32(".namespace.keyname") {
            Ok(v) => push_costack_u32(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_u32(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_u32(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_u32(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_u32(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U64 {
        match read_result_comap_u64(".namespace.keyname") {
            Ok(v) => push_costack_u64(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_u64(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_u64(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_u64(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_u64(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I8 {
        match read_result_comap_i8(".namespace.keyname") {
            Ok(v) => push_costack_i8(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_i8(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_i8(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_i8(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_i8(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I16 {
        match read_result_comap_i16(".namespace.keyname") {
            Ok(v) => push_costack_i16(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_i16(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_i16(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_i16(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_i16(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I32 {
        match read_result_comap_i32(".namespace.keyname") {
            Ok(v) => push_costack_i32(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_i32(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_i32(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_i32(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_i32(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I64 {
        match read_result_comap_i64(".namespace.keyname") {
            Ok(v) => push_costack_i64(v),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_i64(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_i64(): {}", ERR_STR_STACKITEMTOOLARGE),
            Err(RecoverableError::StackItemTooSmall) => println!("Error in read_result_comap_i64(): {}", ERR_STR_STACKITEMTOOSMALL),
            _ => println!("Error in read_result_comap_i64(): {}", ERR_STR_UNHANDLED),
        }
    } else {
        println!("Error: Invalid ABI data (This should never happen)");
    }

    // Assert that the state of our output map matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
