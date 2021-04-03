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

#[test]
fn mirror_bytes() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

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

#[test]
#[should_panic]
fn mirror_bytes_negtest_wrong_content() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

    let var_str = "teststring";
    let var_str_wrong = "stringtest";
    stack.push_bytes(var_str.as_bytes());
    result_stack.push_bytes(var_str_wrong.as_bytes(), "var_str");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        input_stack: stack,
        debug_codata: result_stack,
    };

    harness.load_contract_binary_default_path("test_debugdata", "contract_debugdata");
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
fn mirror_bytes_negtest_wrong_size() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

    let var_str = "teststring";
    let var_str_wrong = "teststringplussome";
    stack.push_bytes(var_str.as_bytes());
    result_stack.push_bytes(var_str_wrong.as_bytes(), "var_str");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        input_stack: stack,
        debug_codata: result_stack,
    };

    harness.load_contract_binary_default_path("test_debugdata", "contract_debugdata");
    initiateAndRun!(harness);
}

#[test]
fn mirror_unsigned() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

    let var_u64 = u64::MAX;
    stack.push_u64(var_u64);
    result_stack.push_u64(var_u64, "var_u64");

    let var_u32 = u32::MAX;
    stack.push_u32(var_u32);
    result_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    stack.push_u16(var_u16);
    result_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    stack.push_u8(var_u8);
    result_stack.push_u8(var_u8, "var_u8");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        input_stack: stack,
        debug_codata: result_stack,
    };

    harness.load_contract_binary_default_path("test_debugdata", "contract_debugdata");
    initiateAndRun!(harness);
}

#[test]
fn mirror_signed() {
    let mut stack = DebugCoDataStack::default();
    let mut result_stack = DebugCoData::default();

    let var_i64 = i64::MIN;
    stack.push_i64(var_i64);
    result_stack.push_i64(var_i64, "var_i64");

    let var_i32 = i32::MIN;
    stack.push_i32(var_i32);
    result_stack.push_i32(var_i32, "var_i32");

    let var_i16 = i16::MIN;
    stack.push_i16(var_i16);
    result_stack.push_i16(var_i16, "var_i16");

    let var_i8 = i8::MIN;
    stack.push_i8(var_i8);
    result_stack.push_i8(var_i8, "var_i8");

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        input_stack: stack,
        debug_codata: result_stack,
    };

    harness.load_contract_binary_default_path("test_debugdata", "contract_debugdata");
    initiateAndRun!(harness);
}
