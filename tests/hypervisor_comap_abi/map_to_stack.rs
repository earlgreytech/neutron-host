use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "hypervisor_comap_abi";
const CONTRACT_DIR_NAME: &'static str = "contract_map_to_stack";

#[test]
fn test_peek_header_size_1() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
fn test_peek_header_size_2() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_2 as u32 + 0x0000_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
fn test_peek_header_size_4() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_4 as u32 + 0x4242_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_header_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(wrong_key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_header_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let wrong_raw_value = "this is the WRONG value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), wrong_raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_header_wrong_size() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;
    let wrong_abi_data = HEADER_SIZE_2 as u32 + 0x0000_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), wrong_abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
