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
const ERR_STR_STACKITEMTOOLARGE: &str = "StackItemTooLarge (Placeholder name - The comap array wasn't aligned to the requested data type)";
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
    if abi_data == ABI_VALUE_U8 + ABI_ARRAY_BIT {
        let mut value: [u8; 5] = [0; 5];
        match read_result_comap_fixed_array_u8(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_u8(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_u8(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_u8(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_u8(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_u8(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U16 + ABI_ARRAY_BIT {
        let mut value: [u16; 5] = [0; 5];
        match read_result_comap_fixed_array_u16(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_u16(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_u16(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_u16(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_u16(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_u16(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U32 + ABI_ARRAY_BIT {
        let mut value: [u32; 5] = [0; 5];
        match read_result_comap_fixed_array_u32(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_u32(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_u32(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_u32(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_u32(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_u32(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_U64 + ABI_ARRAY_BIT {
        let mut value: [u64; 5] = [0; 5];
        match read_result_comap_fixed_array_u64(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_u64(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_u64(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_u64(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_u64(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_u64(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I8 + ABI_ARRAY_BIT {
        let mut value: [i8; 5] = [0; 5];
        match read_result_comap_fixed_array_i8(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_i8(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_i8(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_i8(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_i8(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_i8(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I16 + ABI_ARRAY_BIT {
        let mut value: [i16; 5] = [0; 5];
        match read_result_comap_fixed_array_i16(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_i16(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_i16(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_i16(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_i16(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_i16(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I32 + ABI_ARRAY_BIT {
        let mut value: [i32; 5] = [0; 5];
        match read_result_comap_fixed_array_i32(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_i32(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_i32(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_i32(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_i32(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_i32(): {}", ERR_STR_UNHANDLED),
        }
    } else if abi_data == ABI_VALUE_I64 + ABI_ARRAY_BIT {
        let mut value: [i64; 5] = [0; 5];
        match read_result_comap_fixed_array_i64(".namespace.keyname", &mut value) {
            Ok(5) => push_costack_array_i64(&value),
            Ok(actual_size) => println!("Error in read_result_comap_fixed_array_i64(): Expected array size 5, got {}", actual_size),
            Err(RecoverableError::ItemDoesntExist) => println!("Error in read_result_comap_fixed_array_i64(): {}", ERR_STR_ITEMDOESNTEXIST),
            Err(RecoverableError::StackItemTooLarge) => println!("Error in read_result_comap_fixed_array_i64(): {}", ERR_STR_STACKITEMTOOLARGE),
            _ => println!("Error in read_result_comap_fixed_array_i64(): {}", ERR_STR_UNHANDLED),
        }
    } else {
        println!("Error: Invalid ABI data (This should never happen)");
    }

    // Assert that the state of our output map matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack

    __exit(5);
}
