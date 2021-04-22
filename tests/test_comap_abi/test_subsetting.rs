use neutron_host::comap_abi_decoder::*;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

const DIR_NAME: &'static str = "test_comap_abi";

const CONTRACT_DIR_NAME: &'static str = "contract_subsetting";

#[test]
fn test_comap_peek_header_subsets() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let abi_data = HEADER_SIZE_1 as u32;

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_2);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    stack.push_str(key);
    stack.push_str(key);
    stack.push_str(key);

    map.push_key_abi(key.as_bytes(), value.as_bytes(), abi_data)
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    expected_stack.push_str(value_subset_1, "value_subset_1");
    expected_stack.push_u32(abi_data, "abi_data");
    expected_stack.push_str(value_subset_2, "value_subset_2");
    expected_stack.push_u32(abi_data, "abi_data");
    expected_stack.push_str(value_subset_3, "value_subset_3");
    expected_stack.push_u32(abi_data, "abi_data");

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
fn negtest_comap_peek_header_subsets_wrong_value() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let wrong_value_subset = "<WRONG subset!>";
    let abi_data = HEADER_SIZE_1 as u32;

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(wrong_value_subset);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    stack.push_str(key);
    stack.push_str(key);
    stack.push_str(key);

    map.push_key_abi(key.as_bytes(), value.as_bytes(), abi_data)
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    expected_stack.push_str(value_subset_1, "value_subset_1");
    expected_stack.push_u32(abi_data, "abi_data");
    expected_stack.push_str(value_subset_2, "value_subset_2");
    expected_stack.push_u32(abi_data, "abi_data");
    expected_stack.push_str(value_subset_3, "value_subset_3");
    expected_stack.push_u32(abi_data, "abi_data");

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
