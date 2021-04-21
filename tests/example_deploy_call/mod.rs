use crate::harness::*;
use crate::*;

use elf::File;
use neutron_common::RecoverableError;
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
use neutron_host::neutronerror::*;

use std::cell::RefCell;
use std::env;

// Test that deploying a contract, then calling it again actually works
#[test]
fn test_example_deploy_call() {
    for target in vec!["debug", "release"]{
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        let result = harness.deploy_binary_using_default_callsystem(&TestHarness::get_binary_path("example_deploy_call", "contract_deploy_call", target), context.clone());
        assert_eq!(result.status, 1);
        let result2 = harness.call_using_default_callsystem(context);
        assert_eq!(result2.status, 1);
    }
}


