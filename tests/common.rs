#![allow(dead_code)] // Stuff used only in/by the macros generate warnings otherwise

/// Compute the path to a smart contract binary in this testing folder
#[cfg(test)]
pub fn get_contract_path_target(name: &str, target: &str) -> String {
    let path_str = &format!("./tests/contracts/default_env/target/thumbv6m-none-eabi/{}/{}", target, name);
    path_str.to_string()
}

/// Same as above except debug target is assumed
#[cfg(test)]
pub fn get_contract_path(name: &str) -> String {
    get_contract_path_target(name, "debug")
}

// Does a one-off execution of a single smart contract (debug target assumed)
// Can be assigned to a variable to produce a "NeutronResult" with execution status code and gas remaining
// Note: CONTRACT_NAME can be either a string literal or a constant string variable
#[cfg(test)]
#[macro_export]
macro_rules! single_default_execution {
    ($CONTRACT_NAME:expr) => {{
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($CONTRACT_NAME);
        harness.execute_binary_using_default_callsystem(&contract_path, context)
    }};

    ($CONTRACT_NAME:expr, $DEBUGDATA:ident) => {{
        let mut harness = TestHarness::default();
        harness.debugdata = $DEBUGDATA;
        let context = ExecutionContext::create_default_random_context();
        let contract_path = get_contract_path($CONTRACT_NAME);
        harness.execute_binary_using_default_callsystem(&contract_path, context)
    }};
}
