mod common;

use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;
use neutron_star::syscalls::*;

use common::*;

const CONTRACT_WRITE_SINGLE: &'static str = "neutron_star_comap_simple_write_single";
const CONTRACT_WRITE_ARRAY: &'static str = "neutron_star_comap_simple_write_array";
const CONTRACT_READ_SINGLE: &'static str = "neutron_star_comap_simple_read_single";
const CONTRACT_READ_ARRAY: &'static str = "neutron_star_comap_simple_read_array";

// Write single contract

#[test]
fn test_write_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u8::MAX / 2;
    let abi_data = ABI_VALUE_U8;

    debugdata.inject_stack.push_u8(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_u8(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_u16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u16::MAX / 2;
    let abi_data = ABI_VALUE_U16;

    debugdata.inject_stack.push_u16(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_u16(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_u32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u32::MAX / 2;
    let abi_data = ABI_VALUE_U32;

    debugdata.inject_stack.push_u32(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_u32(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_u64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u64::MAX / 2;
    let abi_data = ABI_VALUE_U64;

    debugdata.inject_stack.push_u64(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_u64(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_i8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i8::MIN / 2;
    let abi_data = ABI_VALUE_I8;

    debugdata.inject_stack.push_i8(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_i8(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_i16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i16::MIN / 2;
    let abi_data = ABI_VALUE_I16;

    debugdata.inject_stack.push_i16(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_i16(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_i32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i32::MIN / 2;
    let abi_data = ABI_VALUE_I32;

    debugdata.inject_stack.push_i32(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_i32(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

#[test]
fn test_write_i64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i64::MIN / 2;
    let abi_data = ABI_VALUE_I64;

    debugdata.inject_stack.push_i64(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi_i64(key.as_bytes(), value);

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

// Write array contract

#[test]
fn test_write_array_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u8; 5] = [1, 2, 3, 4, 5];
    // It would be a bit pointless to convert a byte array to a byte vector...
    let value_bytes = value;
    let abi_data = ABI_VALUE_U8 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_u8(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_u16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u16; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..2 {
            value_bytes.push(u16::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U16 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_u16(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_u32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u32; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            value_bytes.push(u32::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U32 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_u32(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_u64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u64; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..8 {
            value_bytes.push(u64::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U64 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_u64(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_i8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i8; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        value_bytes.push(i8::to_le_bytes(value[i])[0]);
    }
    let abi_data = ABI_VALUE_I8 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_i8(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_i16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i16; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..2 {
            value_bytes.push(i16::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I16 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_i16(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_i32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i32; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            value_bytes.push(i32::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I32 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_i32(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

#[test]
fn test_write_array_i64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i64; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..8 {
            value_bytes.push(i64::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I64 + ABI_ARRAY_BIT;

    debugdata.inject_stack.push_array_i64(&value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_WRITE_ARRAY, debugdata);
}

// Read single contract
// Note: Only actually test the functions for reading the result comap,
// but since they are identical save for one line with the input map version it should be fine.

#[test]
fn test_read_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u8::MAX / 2;
    let abi_data = ABI_VALUE_U8;

    debugdata.expect_stack.push_u8(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_u8(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_u16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u16::MAX / 2;
    let abi_data = ABI_VALUE_U16;

    debugdata.expect_stack.push_u16(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_u16(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_u32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u32::MAX / 2;
    let abi_data = ABI_VALUE_U32;

    debugdata.expect_stack.push_u32(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_u32(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_u64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u64::MAX / 2;
    let abi_data = ABI_VALUE_U64;

    debugdata.expect_stack.push_u64(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_u64(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_i8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i8::MIN / 2;
    let abi_data = ABI_VALUE_I8;

    debugdata.expect_stack.push_i8(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_i8(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_i16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i16::MIN / 2;
    let abi_data = ABI_VALUE_I16;

    debugdata.expect_stack.push_i16(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_i16(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_i32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i32::MIN / 2;
    let abi_data = ABI_VALUE_I32;

    debugdata.expect_stack.push_i32(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_i32(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_i64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = i64::MIN / 2;
    let abi_data = ABI_VALUE_I64;

    debugdata.expect_stack.push_i64(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi_i64(key.as_bytes(), value);

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

// Read array contract

#[test]
fn test_read_array_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u8; 5] = [1, 2, 3, 4, 5];
    // It would be a bit pointless to convert a byte array to a byte vector...
    let value_bytes = value;
    let abi_data = ABI_VALUE_U8 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_u8(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_u16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u16; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..2 {
            value_bytes.push(u16::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U16 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_u16(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_u32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u32; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            value_bytes.push(u32::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U32 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_u32(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_u64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [u64; 5] = [1, 2, 3, 4, 5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..8 {
            value_bytes.push(u64::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_U64 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_u64(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_i8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i8; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        value_bytes.push(i8::to_le_bytes(value[i])[0]);
    }
    let abi_data = ABI_VALUE_I8 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_i8(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_i16() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i16; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..2 {
            value_bytes.push(i16::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I16 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_i16(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_i32() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i32; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            value_bytes.push(i32::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I32 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_i32(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}

#[test]
fn test_read_array_i64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value: [i64; 5] = [-1, -2, -3, -4, -5];
    let mut value_bytes: Vec<u8> = vec![];
    for i in 0..5 {
        for j in 0..8 {
            value_bytes.push(i64::to_le_bytes(value[i])[j]);
        }
    }
    let abi_data = ABI_VALUE_I64 + ABI_ARRAY_BIT;

    debugdata.expect_stack.push_array_i64(&value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.inject_map.push_key_abi(key.as_bytes(), &value_bytes, abi_data);

    single_default_execution!(CONTRACT_READ_ARRAY, debugdata);
}
