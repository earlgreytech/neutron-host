mod common;

use neutron_common::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "neutron_star_costack_mirror_batch";

// TODO: Add more granulate tests?

#[test]
fn test_mirror_batch() {
    let mut debugdata = DebugDataInjector::default();

    let var_u64: u64 = u64::MAX / 2;
    let var_u32: u32 = u32::MAX / 2;
    let var_u16: u16 = u16::MAX / 2;
    let var_u8: u8 = u8::MAX / 2;
    let var_i64: i64 = i64::MIN / 2;
    let var_i32: i32 = i32::MIN / 2;
    let var_i16: i16 = i16::MIN / 2;
    let var_i8: i8 = i8::MIN / 2;

    let mut var_address: NeutronAddress = NeutronAddress::default();
    var_address.version = u32::MAX / 2;
    for i in 0..20 {
        var_address.data[i] = 100 + (i as u8);
    }

    debugdata.inject_stack.push_u64(var_u64);
    debugdata.expect_stack.push_u64(var_u64, "var_u64");
    debugdata.inject_stack.push_u32(var_u32);
    debugdata.expect_stack.push_u32(var_u32, "var_u32");
    debugdata.inject_stack.push_u16(var_u16);
    debugdata.expect_stack.push_u16(var_u16, "var_u16");
    debugdata.inject_stack.push_u8(var_u8);
    debugdata.expect_stack.push_u8(var_u8, "var_u8");

    debugdata.inject_stack.push_i64(var_i64);
    debugdata.expect_stack.push_i64(var_i64, "var_i64");
    debugdata.inject_stack.push_i32(var_i32);
    debugdata.expect_stack.push_i32(var_i32, "var_i32");
    debugdata.inject_stack.push_i16(var_i16);
    debugdata.expect_stack.push_i16(var_i16, "var_i16");
    debugdata.inject_stack.push_i8(var_i8);
    debugdata.expect_stack.push_i8(var_i8, "var_i8");

    debugdata.inject_stack.push_address(var_address);
    debugdata
        .expect_stack
        .push_address(var_address, "var_address");

    single_default_execution!(CONTRACT_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_mirror_batch_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let var_u64: u64 = u64::MAX / 2;
    let var_u32: u32 = u32::MAX / 2;
    let var_u16: u16 = u16::MAX / 2;
    let var_u8: u8 = u8::MAX / 2;
    let var_i64: i64 = i64::MIN / 2;
    let var_i32: i32 = i32::MIN / 2;
    let var_i16: i16 = i16::MIN / 2;
    let var_i8: i8 = i8::MIN / 2;

    let wrong_var_i64: i64 = i64::MIN / 2 + 1;

    let mut var_address: NeutronAddress = NeutronAddress::default();
    var_address.version = u32::MAX / 2;
    for i in 0..20 {
        var_address.data[i] = 100 + (i as u8);
    }

    debugdata.inject_stack.push_u64(var_u64);
    debugdata.expect_stack.push_u64(var_u64, "var_u64");
    debugdata.inject_stack.push_u32(var_u32);
    debugdata.expect_stack.push_u32(var_u32, "var_u32");
    debugdata.inject_stack.push_u16(var_u16);
    debugdata.expect_stack.push_u16(var_u16, "var_u16");
    debugdata.inject_stack.push_u8(var_u8);
    debugdata.expect_stack.push_u8(var_u8, "var_u8");

    debugdata.inject_stack.push_i64(wrong_var_i64);
    debugdata.expect_stack.push_i64(var_i64, "var_i64");
    debugdata.inject_stack.push_i32(var_i32);
    debugdata.expect_stack.push_i32(var_i32, "var_i32");
    debugdata.inject_stack.push_i16(var_i16);
    debugdata.expect_stack.push_i16(var_i16, "var_i16");
    debugdata.inject_stack.push_i8(var_i8);
    debugdata.expect_stack.push_i8(var_i8, "var_i8");

    debugdata.inject_stack.push_address(var_address);
    debugdata
        .expect_stack
        .push_address(var_address, "var_address");

    single_default_execution!(CONTRACT_NAME, debugdata);
}
