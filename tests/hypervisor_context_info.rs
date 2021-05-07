mod common;

//use neutron_host::element_interfaces::debug_data::*;
use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

const CONTRACT_NAME: &'static str = "hypervisor_context_info_gas";

#[test]
fn test_gas() {
    let result = single_default_execution!(CONTRACT_NAME);

    match result.status {
        2 => {
            panic!("\n\nError: One of the recieved gas remaining values was larger than the testing gas limit!\n\n")
        }
        1 => {
            panic!("\n\nError: One of the recieved gas remaining values was not smaller than a preceeding value!\n\n")
        }
        0 => {} // No error!
        _ => {
            panic!("This should never happen")
        }
    }
}
