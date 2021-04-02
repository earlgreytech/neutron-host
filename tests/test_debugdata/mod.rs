extern crate elf;

use crate::harness::*;
use crate::*;

use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;
use neutron_host::{
    db::MemoryGlobalState, element_interfaces::debug_data::*,
    element_interfaces::logging::StdoutLogger, manager::*,
};
use std::path::PathBuf;
use std::{cell::RefCell, env};

const MAX_GAS: u64 = 10000;

use neutron_host::element_interfaces::debug_data::*;

// Test that the smart contract execution doesn't throw an error
#[test]
fn test_debugdata() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

    let var_u64 = u64::MAX;
    stack.push_u64(var_u64);
    result_stack.push_u64(var_u64 - 1, "var_u64");

    let var_u32 = u32::MAX;
    stack.push_u32(var_u32);
    result_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    stack.push_u16(var_u16);
    result_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    stack.push_u8(var_u8);
    result_stack.push_u8(var_u8, "var_u8");

    let var_str = "teststring";
    stack.push_bytes(var_str.as_bytes());
    result_stack.push_bytes(var_str.as_bytes(), "var_str");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        input_stack: stack,
        debug_codata: result_stack,
    };

    harness.load_contract_binary_default_path("test_debugdata", "contract_debugdata");
    initiateAndRun!(harness);
}