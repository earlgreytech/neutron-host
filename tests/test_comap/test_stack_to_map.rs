use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "test_comap";
const CONTRACT_DIR_NAME: &'static str = "contract_stack_to_map";

#[test]
fn comap_push() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value = "this is the value";

    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(value);

    debugdata
        .expect_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn comap_push_negtest_wrong_key() {
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

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn comap_push_negtest_wrong_value() {
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

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
