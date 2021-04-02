extern crate elf;

use crate::harness::*;
use crate::*;

use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;
use neutron_host::{
    db::MemoryGlobalState, element_interfaces::debug_data::DebugDataInjector,
    element_interfaces::logging::StdoutLogger, manager::*,
};
use std::path::PathBuf;
use std::{cell::RefCell, env};

const MAX_GAS: u64 = 10000;

use neutron_host::element_interfaces::debug_data::*;

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
    let mut harness = TestHarness::default();
    harness.load_contract_binary_default_path("test_smoke", "contract_smoke");
    initiateAndRun!(harness);
}
