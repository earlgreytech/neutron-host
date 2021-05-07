mod common;

use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_MAP_TO_STACK: &'static str = "hypervisor_comap_map_to_stack";
const CONTRACT_STACK_TO_MAP: &'static str = "hypervisor_comap_stack_to_map";
const CONTRACT_SUBSETTING: &'static str = "hypervisor_comap_subsetting";

// Map to stack contract

#[test]
fn test_peek() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(value, "comap_value");

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let value = "this is the value";

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(value, "comap_value");

    debugdata
        .inject_map
        .push_key(wrong_key.as_bytes(), value.as_bytes())
        .unwrap(); // Push wrong key as contract input

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";
    let wrong_value = "this is the WRONG value";

    debugdata.inject_stack.push_str(key);
    debugdata.expect_stack.push_str(value, "comap_value");

    debugdata
        .inject_map
        .push_key(key.as_bytes(), wrong_value.as_bytes())
        .unwrap(); // Push wrong value as contract input

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

#[test]
fn test_peek_trucated() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    debugdata.inject_stack.push_str(key);
    debugdata
        .expect_stack
        .push_str(value_fitting, "comap_value");

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_trucated() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let _value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    debugdata.inject_stack.push_str(key);
    debugdata
        .expect_stack
        .push_str(value_unfitting, "comap_value"); // We expect the un-truncated value, so assertion will fail

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_MAP_TO_STACK, debugdata);
}

// Stack to map contract

#[test]
fn test_push() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(value);

    debugdata
        .expect_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
#[should_panic]
fn negtest_push_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let value = "this is the value";

    debugdata.inject_stack.push_str(wrong_key); // Push wrong key as contract input
    debugdata.inject_stack.push_str(value);

    debugdata
        .expect_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

#[test]
#[should_panic]
fn negtest_push_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";
    let wrong_value = "this is the WRONG value";

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(wrong_value); // Push wrong value as contract input

    debugdata
        .expect_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(CONTRACT_STACK_TO_MAP, debugdata);
}

// Subsetting contract

#[test]
fn test_peek_subsets() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";

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
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    single_default_execution!(CONTRACT_SUBSETTING, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_subsets_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let value_subset_wrong = "<WRONG subset!>";

    // Construct a single String from the subsets, to be added to input codata
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_wrong);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    single_default_execution!(CONTRACT_SUBSETTING, debugdata);
}
