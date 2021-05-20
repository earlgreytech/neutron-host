mod common;

use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;
use neutron_star::syscalls::*;

use common::*;

const CONTRACT_WRITE_SINGLE: &'static str = "neutron_star_comap_simple_write_single";
const CONTRACT_READ_SINGLE: &'static str = "neutron_star_comap_simple_read_single";

// Write single contract

#[test]
fn test_write_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u8::MAX / 2;
    let abi_data = ABI_VALUE_U8;

    debugdata.inject_stack.push_u8(value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &u8::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &u16::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &u32::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &u64::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &i8::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &i16::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &i32::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), &i64::to_le_bytes(value), abi_data)
        .unwrap();

    single_default_execution!(CONTRACT_WRITE_SINGLE, debugdata);
}

// Read single contract

#[test]
fn test_read_u64() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u64::MAX / 2;
    let abi_data = ABI_VALUE_U64;

    debugdata.expect_stack.push_u64(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &u64::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &u32::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &u16::to_le_bytes(value), abi_data)
        .unwrap();

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}

#[test]
fn test_read_u8() {
    let mut debugdata = DebugDataInjector::default();

    let key = ".namespace.keyname";
    let value = u8::MAX / 2;
    let abi_data = ABI_VALUE_U8;

    debugdata.expect_stack.push_u8(value, "value");
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &u8::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &i64::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &i32::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &i16::to_le_bytes(value), abi_data)
        .unwrap();

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), &i8::to_le_bytes(value), abi_data)
        .unwrap();

    single_default_execution!(CONTRACT_READ_SINGLE, debugdata);
}
