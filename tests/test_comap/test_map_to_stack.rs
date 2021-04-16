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

const CONTRACT_DIR_NAME: &'static str = "contract_map_to_stack";

#[test]
fn comap_peek() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";

    stack.push_str(key);
    expected_stack.push_str(value, "comap_key");

    map.push_key(key.as_bytes(), value.as_bytes()).unwrap();

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
