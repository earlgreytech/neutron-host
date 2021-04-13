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

const DIR_NAME: &'static str = "test_debugdata";
const CONTRACT_DIR_NAME: &'static str = "contract_mirror";

#[test]
// Push a byte slice to contract
fn mirror_bytes() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_bytes = "testbytes";
    stack.push_bytes(var_bytes.as_bytes());
    assert_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
// Expect the wrong content
fn mirror_negtest_wrong_content() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_bytes = "testbytes";
    let var_bytes_wrong = "bytestest";
    stack.push_bytes(var_bytes_wrong.as_bytes());
    assert_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
// Expect fewer stack items than we get
fn mirror_negtest_too_many() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_bytes = "testbytes";
    stack.push_bytes(var_bytes.as_bytes());
    stack.push_bytes(var_bytes.as_bytes());
    assert_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
// Expect more stack items than we get
fn mirror_negtest_too_few() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_bytes = "testbytes";
    stack.push_bytes(var_bytes.as_bytes());
    assert_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");
    assert_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
// Push a str to contract
fn mirror_str() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_str = "teststr";
    stack.push_str(var_str);
    assert_stack.push_str(var_str, "var_str");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}

#[test]
// Push some unsigned values to contract
fn mirror_unsigned() {
    let mut stack = DebugCoStack::default();
    let mut assert_stack = WrappedDebugCoStack::default();

    let var_u64 = u64::MAX;
    stack.push_u64(var_u64);
    assert_stack.push_u64(var_u64, "var_u64");

    let var_u32 = u32::MAX;
    stack.push_u32(var_u32);
    assert_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    stack.push_u16(var_u16);
    assert_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    stack.push_u8(var_u8);
    assert_stack.push_u8(var_u8, "var_u8");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: assert_stack,
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_DIR_NAME);
    initiateAndRun!(harness);
}
