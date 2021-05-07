mod common;

use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "smoke_test_minimal";

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
    single_default_execution!(CONTRACT_NAME);
}
