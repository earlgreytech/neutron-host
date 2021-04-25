use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "test_comap_abi";
const CONTRACT_DIR_NAME: &'static str = "contract_stack_to_map";

#[test]
fn test_comap_push_header_size_1() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
fn test_comap_push_header_size_2() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = (HEADER_SIZE_2 as u32) + 0x0000_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
fn test_comap_push_header_size_4() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = (HEADER_SIZE_4 as u32) + 0x1337_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_comap_push_header_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key!";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(wrong_key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_comap_push_header_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let wrong_raw_value = "this is the WRONG value!";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(wrong_raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_comap_push_header_wrong_size() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;
    let wrong_abi_data = HEADER_SIZE_2 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(wrong_abi_data);

    debugdata
        .expect_map
        .push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
