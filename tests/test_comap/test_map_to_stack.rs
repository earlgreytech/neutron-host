use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "test_comap";
const CONTRACT_DIR_NAME: &'static str = "contract_map_to_stack";

#[test]
fn comap_peek() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";

    debugdata.injected_input_stack.push_str(key);
    debugdata
        .expected_output_stack
        .push_str(value, "comap_value");

    debugdata
        .injected_result_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn comap_peek_negtest_wrong_key() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let value = "this is the value";

    debugdata.injected_input_stack.push_str(key);
    debugdata
        .expected_output_stack
        .push_str(value, "comap_value");

    debugdata
        .injected_result_map
        .push_key(wrong_key.as_bytes(), value.as_bytes())
        .unwrap(); // Push wrong key as contract input

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn comap_peek_negtest_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";
    let wrong_value = "this is the WRONG value";

    debugdata.injected_input_stack.push_str(key);
    debugdata
        .expected_output_stack
        .push_str(value, "comap_value");

    debugdata
        .injected_result_map
        .push_key(key.as_bytes(), wrong_value.as_bytes())
        .unwrap(); // Push wrong value as contract input

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
fn comap_peek_trucated() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    debugdata.injected_input_stack.push_str(key);
    debugdata
        .expected_output_stack
        .push_str(value_fitting, "comap_value");

    debugdata
        .injected_result_map
        .push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn comap_peek_trucated_negtest() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let _value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    debugdata.injected_input_stack.push_str(key);
    debugdata
        .expected_output_stack
        .push_str(value_unfitting, "comap_value"); // We expect the un-truncated value, so assertion will fail

    debugdata
        .injected_result_map
        .push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
