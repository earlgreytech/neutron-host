use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "test_debugdata";
const CONTRACT_DIR_NAME: &'static str = "contract_mirror";

#[test]
// Push a byte slice to contract
fn mirror_bytes() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata
        .injected_input_stack
        .push_bytes(var_bytes.as_bytes());
    debugdata
        .expected_output_stack
        .push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect the wrong content
fn mirror_negtest_wrong_content() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    let var_bytes_wrong = "bytestest";
    debugdata
        .injected_input_stack
        .push_bytes(var_bytes_wrong.as_bytes());
    debugdata
        .expected_output_stack
        .push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect fewer stack items than we get
fn mirror_negtest_too_many() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata
        .injected_input_stack
        .push_bytes(var_bytes.as_bytes());
    debugdata
        .injected_input_stack
        .push_bytes(var_bytes.as_bytes());
    debugdata
        .expected_output_stack
        .push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect more stack items than we get
fn mirror_negtest_too_few() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata
        .injected_input_stack
        .push_bytes(var_bytes.as_bytes());
    debugdata
        .expected_output_stack
        .push_bytes(var_bytes.as_bytes(), "var_bytes");
    debugdata
        .expected_output_stack
        .push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
// Push a str to contract
fn mirror_str() {
    let mut debugdata = DebugDataInjector::default();

    let var_str = "teststr";
    debugdata.injected_input_stack.push_str(var_str);
    debugdata.expected_output_stack.push_str(var_str, "var_str");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
// Push some unsigned values to contract
fn mirror_unsigned() {
    let mut debugdata = DebugDataInjector::default();

    let var_u64 = u64::MAX;
    debugdata.injected_input_stack.push_u64(var_u64);
    debugdata.expected_output_stack.push_u64(var_u64, "var_u64");

    let var_u32 = u32::MAX;
    debugdata.injected_input_stack.push_u32(var_u32);
    debugdata.expected_output_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    debugdata.injected_input_stack.push_u16(var_u16);
    debugdata.expected_output_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    debugdata.injected_input_stack.push_u8(var_u8);
    debugdata.expected_output_stack.push_u8(var_u8, "var_u8");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
