/// Compute the path to a smart contract binary in this testing folder
pub fn get_contract_path(test_dir: &str, contract_dir: &str) -> String {
    let path_str = &format!(
        "./tests/{}/{}/target/thumbv6m-none-eabi/debug/contract-binary",
        test_dir, contract_dir
    );
    path_str.to_string()
}

#[macro_export]
macro_rules! single_default_execution {
    // Handles both string litterals and constant string variables (becasue they resolve to the former)
    ($TEST_DIR:expr, $CONTRACT_DIR:expr) => {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($TEST_DIR, $CONTRACT_DIR);
        harness.execute_binary_using_default_callsystem(&contract_path, context);
    };

    ($TEST_DIR:expr, $CONTRACT_DIR:expr, $DEBUGDATA:ident) => {
        let mut harness = TestHarness::default();
        harness.debugdata = $DEBUGDATA;
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($TEST_DIR, $CONTRACT_DIR);
        harness.execute_binary_using_default_callsystem(&contract_path, context);
    };
}
