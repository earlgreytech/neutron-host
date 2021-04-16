use crate::harness::*;
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

use std::cell::RefCell;
use std::env;

const DIR_NAME: &'static str = "test_comap";

const CONTRACT_DIR_NAME: &'static str = "contract_subsetting";

#[test]
fn comap_peek_subsets() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_2);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    stack.push_str(key);
    stack.push_str(key);
    stack.push_str(key);

    map.push_key(key.as_bytes(), value.as_bytes()).unwrap();

    // We expect the contract to split the comap value into the subsets
    expected_stack.push_str(value_subset_1, "value_subset_1");
    expected_stack.push_str(value_subset_2, "value_subset_2");
    expected_stack.push_str(value_subset_3, "value_subset_3");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
fn comap_peek_subsets_negtest_wrong_value() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

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
    stack.push_str(key);
    stack.push_str(key);
    stack.push_str(key);

    map.push_key(key.as_bytes(), value.as_bytes()).unwrap();

    // We expect the contract to split the comap value into the subsets
    expected_stack.push_str(value_subset_1, "value_subset_1");
    expected_stack.push_str(value_subset_2, "value_subset_2");
    expected_stack.push_str(value_subset_3, "value_subset_3");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}
