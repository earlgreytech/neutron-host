use crate::*;

// These will throw lots of unused import warnings because some are only used in macros
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::db::MemoryGlobalState;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::element_interfaces::logging::StdoutLogger;
use neutron_host::interface::*;
use neutron_host::manager::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;
use neutron_host::harness::*;

use std::cell::RefCell;
use std::env;

const DIR_NAME: &'static str = "test_comap";

const CONTRACT_DIR_NAME: &'static str = "contract_map_to_stack";

#[test]
fn comap_peek() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";

    stack.push_str(key);
    expected_stack.push_str(value, "comap_value");

    map.push_key(key.as_bytes(), value.as_bytes()).unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(DIR_NAME, CONTRACT_DIR_NAME, context);
}

#[test]
#[should_panic]
fn comap_peek_negtest_wrong_key() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let value = "this is the value";

    stack.push_str(key);
    expected_stack.push_str(value, "comap_value");

    map.push_key(wrong_key.as_bytes(), value.as_bytes())
        .unwrap(); // Push wrong key as contract input

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(DIR_NAME, CONTRACT_DIR_NAME, context);
}

#[test]
#[should_panic]
fn comap_peek_negtest_wrong_value() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";
    let wrong_value = "this is the WRONG value";

    stack.push_str(key);
    expected_stack.push_str(value, "comap_value");

    map.push_key(key.as_bytes(), wrong_value.as_bytes())
        .unwrap(); // Push wrong value as contract input

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(DIR_NAME, CONTRACT_DIR_NAME, context);
}

#[test]
fn comap_peek_trucated() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    stack.push_str(key);
    expected_stack.push_str(value_fitting, "comap_value");

    map.push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(DIR_NAME, CONTRACT_DIR_NAME, context);
}

#[test]
#[should_panic]
fn comap_peek_trucated_negtest() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";

    // This contract limits size of peeked comap value to 25 bytes
    let _value_fitting = "this is a 25-byte value!!";
    let value_unfitting = "this is a 25-byte value!! Except for this part!";

    stack.push_str(key);
    expected_stack.push_str(value_unfitting, "comap_value"); // We expect the un-truncated value, so assertion will fail

    map.push_key(key.as_bytes(), value_unfitting.as_bytes())
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(DIR_NAME, CONTRACT_DIR_NAME, context);
}
