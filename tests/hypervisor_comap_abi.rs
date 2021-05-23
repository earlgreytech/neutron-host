mod common;

use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_MAP_TO_STACK: &'static str = "hypervisor_comap_abi_map_to_stack";
const CONTRACT_STACK_TO_MAP: &'static str = "hypervisor_comap_abi_stack_to_map";
const CONTRACT_SUBSETTING: &'static str = "hypervisor_comap_abi_subsetting";

// Map to stack contract

#[test]
fn test_peek_header_size_1() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(raw_value, "comap_value");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    debugdata.inject_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
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

    debugdata.inject_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
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

    debugdata.inject_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
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

    debugdata.inject_map.push_key_abi(wrong_key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
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

    debugdata.inject_map.push_key_abi(key.as_bytes(), wrong_raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
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

    debugdata.inject_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), wrong_abi_data);

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

// Stack to map contract

#[test]
fn test_push_header_size_1() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
fn test_push_header_size_2() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = (HEADER_SIZE_2 as u32) + 0x0000_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
fn test_push_header_size_4() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = (HEADER_SIZE_4 as u32) + 0x1337_4200;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
#[should_panic]
fn negtest_push_header_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key!";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(wrong_key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
#[should_panic]
fn negtest_push_header_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let wrong_raw_value = "this is the WRONG value!";
    let abi_data = HEADER_SIZE_1 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(wrong_raw_value);
    debugdata.inject_stack.push_u32(abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
#[should_panic]
fn negtest_push_header_wrong_size() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;
    let wrong_abi_data = HEADER_SIZE_2 as u32;

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(raw_value);
    debugdata.inject_stack.push_u32(wrong_abi_data);

    debugdata.expect_map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data);

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

// Subsetting contract

#[test]
fn test_peek_header_subsets() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let abi_data = HEADER_SIZE_1 as u32;

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_2);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);

    debugdata.inject_map.push_key_abi(key.as_bytes(), value.as_bytes(), abi_data);

    // We expect the contract to split the comap value into the subsets
    // Note: Values and ABI data are separated to avoid costack conflicts in the contract!
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    single_default_execution!(CONTRACT_SUBSETTING, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_header_subsets_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let wrong_value_subset = "<WRONG subset!>";
    let abi_data = HEADER_SIZE_1 as u32;

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(wrong_value_subset);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);

    debugdata.inject_map.push_key_abi(key.as_bytes(), value.as_bytes(), abi_data);

    // We expect the contract to split the comap value into the subsets
    // Note: Values and ABI data are separated to avoid costack conflicts in the contract!
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    single_default_execution!(CONTRACT_SUBSETTING, debugdata);
}
