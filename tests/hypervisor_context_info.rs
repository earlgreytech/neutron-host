mod common;

use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

#[test]
fn test_gas_remaining() {
    let result = single_default_execution!("hypervisor_context_info_gas_remaining");

    match result.status {
        2 => {
            panic!("\n\nError: One of the recieved gas remaining values was larger than the testing gas limit!\n\n")
        }
        1 => {
            panic!("\n\nError: One of the recieved gas remaining values was not smaller than a preceeding value!\n\n")
        }
        0 => {} // No error!
        _ => {
            panic!("This should never happen")
        }
    }
}

#[test]
fn test_execution_type_bare_execution() {
    let result = single_default_execution!("hypervisor_context_info_execution_type");

    // Only 0, 1, and 2 are defined for ExecutionType
    if result.status > 2 {
        panic!("\n\nError: u32 returned from function didn't map to a valid ExecutionType!\n\n")
    }

    let result_execution_type = num::FromPrimitive::from_u32(result.status).unwrap();

    match result_execution_type {
        ExecutionType::Call => {
            panic!("\n\nError: Expected ExecutionType 'BareExecution'(2), found 'Call'(0)!\n\n")
        }
        ExecutionType::Deploy => {
            panic!("\n\nError: Expected ExecutionType 'BareExecution'(2), found 'Deploy'(1)!\n\n")
        }
        ExecutionType::BareExecution => {} // Correct type!
    }
}

#[test]
fn test_execution_type_deploy() {
    let mut harness = TestHarness::default();
    let context = ExecutionContext::create_default_random_context();
    let result =
        harness.deploy_binary_using_default_callsystem(&get_contract_path("hypervisor_context_info_execution_type"), context.clone());

    // Only 0, 1, and 2 are defined for ExecutionType
    if result.status > 2 {
        panic!("\n\nError: u32 returned from function didn't map to a valid ExecutionType!\n\n")
    }

    let result_execution_type = num::FromPrimitive::from_u32(result.status).unwrap();

    match result_execution_type {
        ExecutionType::Call => {
            panic!("\n\nError: Expected ExecutionType 'Deploy'(1), found 'Call'(0)!\n\n")
        }
        ExecutionType::Deploy => {} // Correct type!
        ExecutionType::BareExecution => {
            panic!("\n\nError: Expected ExecutionType 'Deploy'(1), found 'BareExecution'(2)!\n\n")
        }
    }
}

#[test]
fn test_self_address() {
    let mut debugdata = DebugDataInjector::default();
    let mut harness = TestHarness::default();

    // Create a context with random self address
    let context = ExecutionContext::create_default_random_context();

    debugdata.expect_stack.push_address(context.self_address, "self_address");

    harness.debugdata = debugdata;
    harness.execute_binary_using_default_callsystem(&get_contract_path("hypervisor_context_info_self_address"), context);
}

#[test]
fn test_origin() {
    let mut debugdata = DebugDataInjector::default();
    let mut harness = TestHarness::default();

    // Create a context with random self address and copy it to origin address
    let mut context = ExecutionContext::create_default_random_context();
    context.origin = context.self_address;

    debugdata.expect_stack.push_address(context.origin, "origin");

    harness.debugdata = debugdata;
    harness.execute_binary_using_default_callsystem(&get_contract_path("hypervisor_context_info_origin"), context);
}

#[test]
fn test_sender() {
    let mut debugdata = DebugDataInjector::default();
    let mut harness = TestHarness::default();

    // Create a context with random self address and copy it to sender address
    let mut context = ExecutionContext::create_default_random_context();
    context.sender = context.self_address;

    debugdata.expect_stack.push_address(context.sender, "sender");

    harness.debugdata = debugdata;
    harness.execute_binary_using_default_callsystem(&get_contract_path("hypervisor_context_info_sender"), context);
}
