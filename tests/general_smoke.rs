mod common;

use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "general_smoke_test";

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
    single_default_execution!(CONTRACT_NAME);
}
