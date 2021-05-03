use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const CONTRACT_NAME: &'static str = "hypervisor_comap_abi_subsetting";

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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), value.as_bytes(), abi_data)
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    single_default_execution!(CONTRACT_NAME, debugdata);
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

    debugdata
        .inject_map
        .push_key_abi(key.as_bytes(), value.as_bytes(), abi_data)
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");
    debugdata.expect_stack.push_u32(abi_data, "abi_data");

    single_default_execution!(CONTRACT_NAME, debugdata);
}
