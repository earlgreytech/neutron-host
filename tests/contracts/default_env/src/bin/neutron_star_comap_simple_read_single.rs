//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
use neutron_star::*;
use neutron_star::syscalls::*;
extern crate panic_halt;

const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    __system_call(DEBUG_DATA_FEATURE, 3); // DebugDataFunctions::PushResultMap
    __system_call(DEBUG_DATA_FEATURE, 1); // DebugDataFunctions::PushInputStack
    
    // Pop ABI data bytes and cast to u32
    let mut abi_data_buf: [u8; 4] = [0; 4]; 
    let _ = __pop_costack(abi_data_buf.as_mut_ptr(), 4);
    let abi_data = u32::from_le_bytes(abi_data_buf); 
    
    // NOTE: This was a match statement originally, but a bug was messing that up
    if abi_data == ABI_VALUE_U64 {
        let value = read_comap_u64(".namespace.keyname").unwrap();
        push_costack_u64(value);
    }
    else if abi_data == ABI_VALUE_U32 {
        let value = read_comap_u32(".namespace.keyname").unwrap();
        push_costack_u32(value);
    }
    else if abi_data == ABI_VALUE_U16 {
        let value = read_comap_u16(".namespace.keyname").unwrap();
        push_costack_u16(value);
    }
    else if abi_data == ABI_VALUE_U8 {
        let value = read_comap_u8(".namespace.keyname").unwrap();
        push_costack_u8(value);
    }
    else if abi_data == ABI_VALUE_I64 {
        let value = read_comap_i64(".namespace.keyname").unwrap();
        push_costack_i64(value);
    }
    else if abi_data == ABI_VALUE_I32 {
        let value = read_comap_i32(".namespace.keyname").unwrap();
        push_costack_i32(value);
    }
    else if abi_data == ABI_VALUE_I16 {
        let value = read_comap_i16(".namespace.keyname").unwrap();
        push_costack_i16(value);
    }
    else if abi_data == ABI_VALUE_I8 {
        let value = read_comap_i8(".namespace.keyname").unwrap();
        push_costack_i8(value);
    }
    else {
        println!("Error: Invalid ABI data (This should never happen)");
    }
    
    // Assert that the state of our output map matches provided expected state
    __system_call(DEBUG_DATA_FEATURE, 2); // DebugDataFunctions::AssertOutputStack
    
    __exit(5);
}
