mod common;

use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "smoke_test_deploy_call";

// Test that deploying a contract, then calling it again actually works
#[test]
fn test_deploy_call() {
    for target in vec!["debug", "release"] {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        let result = harness.deploy_binary_using_default_callsystem(&get_contract_path_target(CONTRACT_NAME, target), context.clone());
        assert_eq!(result.status, 1);
        let result2 = harness.call_using_default_callsystem(context);
        assert_eq!(result2.status, 1);
    }
}
