mod common;

use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "hypervisor_costack_mirror";

#[test]
// Push a byte slice to contract
fn test_mirror_bytes() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata.inject_stack.push_bytes(var_bytes.as_bytes());
    debugdata.expect_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect the wrong content
fn negtest_mirror_wrong_bytes() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    let var_bytes_wrong = "bytestest";
    debugdata.inject_stack.push_bytes(var_bytes_wrong.as_bytes());
    debugdata.expect_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect fewer stack items than we get
fn negtest_mirror_too_many_bytes() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata.inject_stack.push_bytes(var_bytes.as_bytes());
    debugdata.inject_stack.push_bytes(var_bytes.as_bytes());
    debugdata.expect_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
// Expect more stack items than we get
fn negtest_mirror_too_few_bytes() {
    let mut debugdata = DebugDataInjector::default();

    let var_bytes = "testbytes";
    debugdata.inject_stack.push_bytes(var_bytes.as_bytes());
    debugdata.expect_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");
    debugdata.expect_stack.push_bytes(var_bytes.as_bytes(), "var_bytes");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
// Push a str to contract
fn test_mirror_str() {
    let mut debugdata = DebugDataInjector::default();

    let var_str = "teststr";
    debugdata.inject_stack.push_str(var_str);
    debugdata.expect_stack.push_str(var_str, "var_str");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
// Push some unsigned values to contract
fn test_mirror_unsigned() {
    let mut debugdata = DebugDataInjector::default();

    let var_u64 = u64::MAX;
    debugdata.inject_stack.push_u64(var_u64);
    debugdata.expect_stack.push_u64(var_u64, "var_u64");

    let var_u32 = u32::MAX;
    debugdata.inject_stack.push_u32(var_u32);
    debugdata.expect_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    debugdata.inject_stack.push_u16(var_u16);
    debugdata.expect_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    debugdata.inject_stack.push_u8(var_u8);
    debugdata.expect_stack.push_u8(var_u8, "var_u8");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_mirror_unsigned_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let var_u64 = u64::MAX;
    debugdata.inject_stack.push_u64(var_u64);
    debugdata.expect_stack.push_u64(var_u64, "var_u64");

    let var_u32 = u32::MAX;
    debugdata.inject_stack.push_u32(var_u32);
    debugdata.expect_stack.push_u32(var_u32, "var_u32");

    let var_u16 = u16::MAX;
    debugdata.inject_stack.push_u16(42 as u16);
    debugdata.expect_stack.push_u16(var_u16, "var_u16");

    let var_u8 = u8::MAX;
    debugdata.inject_stack.push_u8(var_u8);
    debugdata.expect_stack.push_u8(var_u8, "var_u8");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
// Push some signed values to contract
fn test_mirror_signed() {
    let mut debugdata = DebugDataInjector::default();

    let var_i64 = i64::MIN;
    debugdata.inject_stack.push_i64(var_i64);
    debugdata.expect_stack.push_i64(var_i64, "var_i64");

    let var_i32 = i32::MIN;
    debugdata.inject_stack.push_i32(var_i32);
    debugdata.expect_stack.push_i32(var_i32, "var_i32");

    let var_i16 = i16::MIN;
    debugdata.inject_stack.push_i16(var_i16);
    debugdata.expect_stack.push_i16(var_i16, "var_i16");

    let var_i8 = i8::MIN;
    debugdata.inject_stack.push_i8(var_i8);
    debugdata.expect_stack.push_i8(var_i8, "var_i8");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
// Push some signed values to contract
fn negtest_mirror_signed_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let var_i64 = i64::MIN;
    debugdata.inject_stack.push_i64(var_i64);
    debugdata.expect_stack.push_i64(var_i64, "var_i64");

    let var_i32 = i32::MIN;
    debugdata.inject_stack.push_i32(var_i32);
    debugdata.expect_stack.push_i32(var_i32, "var_i32");

    let var_i16 = i16::MIN;
    debugdata.inject_stack.push_i16(-42 as i16);
    debugdata.expect_stack.push_i16(var_i16, "var_i16");

    let var_i8 = i8::MIN;
    debugdata.inject_stack.push_i8(var_i8);
    debugdata.expect_stack.push_i8(var_i8, "var_i8");

    single_default_execution!(CONTRACT_NAME, debugdata);
}
