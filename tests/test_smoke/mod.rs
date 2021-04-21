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

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        harness.execute_debug_path_binary_using_default_callsystem("test_smoke", "contract_smoke", context);
}
