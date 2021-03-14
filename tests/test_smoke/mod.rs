extern crate elf;

use crate::common::*;
use crate::*;

use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;
use neutron_host::{db::MemoryGlobalState, element_interfaces::logging::StdoutLogger, manager::*};
use std::path::PathBuf;
use std::{cell::RefCell, env};

const MAX_GAS: u64 = 10000;

// Test that the smart contract execution doesn't throw an error
#[test]
fn test_smoke() {
    let path = PathBuf::from(
        "./tests/test_smoke/contract_smoke/target/thumbv6m-none-eabi/debug/contract-binary",
    );

    let mut test_setup = TestSetup::default();
    initiate!(test_setup, path);

    println!("Beginning contract execution");
    let result = test_setup
        .manager
        .execute(
            &mut test_setup.codata,
            &test_setup.callsystem,
            &test_setup.vmm,
        )
        .unwrap();
    println!("Contract executed successfully!");
    println!("Gas used: {}", result.gas_used);
    println!("Status code: {:x}", result.status);
}
