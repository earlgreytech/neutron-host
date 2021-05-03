/// Compute the path to a smart contract binary in this testing folder
pub fn get_contract_path_target(name: &str, target: &str) -> String {
    let path_str = &format!(
        "./tests/contracts/default_env/target/thumbv6m-none-eabi/{}/{}",
        target, name
    );
    path_str.to_string()
}

/// Same as above except debug target is assumed
pub fn get_contract_path(name: &str) -> String {
    get_contract_path_target(name, "debug")
}

// Does a one-off execution of a single smart contract (debug target assumed)
#[macro_export]
macro_rules! single_default_execution {
    // Handles both string litterals and constant string variables (becasue they resolve to the former)
    ($CONTRACT_NAME:expr) => {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($CONTRACT_NAME);
        harness.execute_binary_using_default_callsystem(&contract_path, context);
    };

    ($CONTRACT_NAME:expr, $DEBUGDATA:ident) => {
        let mut harness = TestHarness::default();
        harness.debugdata = $DEBUGDATA;
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($CONTRACT_NAME);
        harness.execute_binary_using_default_callsystem(&contract_path, context);
    };
}
