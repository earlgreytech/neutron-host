use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "hypervisor_comap";
const CONTRACT_DIR_NAME: &'static str = "contract_subsetting";

#[test]
fn test_peek_subsets() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";

    // Construct a single String from the subsets
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_2);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}

#[test]
#[should_panic]
fn negtest_peek_subsets_wrong_value() {
    let mut debugdata = DebugDataInjector::default();

    let key = "this is the key";
    let value_subset_1 = "<value part 1!>";
    let value_subset_2 = "<value part 2!>";
    let value_subset_3 = "<value part 3!>";
    let value_subset_wrong = "<WRONG subset!>";

    // Construct a single String from the subsets, to be added to input codata
    let mut value = String::from(value_subset_1);
    value.push_str(value_subset_wrong);
    value.push_str(value_subset_3);

    // This will be used by contract to peek the comap value (one push for each subset, since a peek consumes the key it uses)
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);
    debugdata.inject_stack.push_str(key);

    debugdata
        .inject_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    // We expect the contract to split the comap value into the subsets
    debugdata
        .expect_stack
        .push_str(value_subset_1, "value_subset_1");
    debugdata
        .expect_stack
        .push_str(value_subset_2, "value_subset_2");
    debugdata
        .expect_stack
        .push_str(value_subset_3, "value_subset_3");

    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME, debugdata);
}
