use neutron_host::harness::*;
use neutron_host::interface::*;

use crate::common::*;
use crate::*;

const DIR_NAME: &'static str = "general_smoke";
const CONTRACT_DIR_NAME: &'static str = "contract_smoke";

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
    single_default_execution!(DIR_NAME, CONTRACT_DIR_NAME);
}
