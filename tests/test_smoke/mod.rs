use neutron_host::interface::*;
use neutron_host::harness::*;

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        harness.execute_debug_path_binary_using_default_callsystem("test_smoke", "contract_smoke", context);
}
