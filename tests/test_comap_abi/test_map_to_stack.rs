use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

const DIR_NAME: &'static str = "test_comap_abi";

const CONTRACT_DIR_NAME: &'static str = "contract_map_to_stack";

#[test]
fn test_comap_peek_header_size_1() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}

#[test]
fn test_comap_peek_header_size_2() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_2 as u32 + 0x0000_4200;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}

#[test]
fn test_comap_peek_header_size_4() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_4 as u32 + 0x4242_4200;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}

#[test]
#[should_panic]
fn negtest_comap_peek_header_wrong_key() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(wrong_key.as_bytes(), raw_value.as_bytes(), abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}

#[test]
#[should_panic]
fn negtest_comap_peek_header_wrong_value() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let wrong_raw_value = "this is the WRONG value";
    let abi_data = HEADER_SIZE_1 as u32;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(key.as_bytes(), wrong_raw_value.as_bytes(), abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}

#[test]
#[should_panic]
fn negtest_comap_peek_header_wrong_size() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let raw_value = "this is the value";
    let abi_data = HEADER_SIZE_1 as u32;
    let wrong_abi_data = HEADER_SIZE_2 as u32 + 0x0000_4200;

    stack.push_str(key);
    expected_stack.push_str(raw_value, "comap_value");
    expected_stack.push_u32(abi_data, "abi_data");

    map.push_key_abi(key.as_bytes(), raw_value.as_bytes(), wrong_abi_data)
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    let context = ExecutionContext::create_default_random_context();
    harness.execute_debug_path_binary_using_default_callsystem(
        DIR_NAME,
        CONTRACT_DIR_NAME,
        context,
    );
}
