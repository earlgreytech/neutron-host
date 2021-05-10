mod common;

use neutron_host::harness::*;
use neutron_host::interface::*;

use common::*;

#[test]
fn test_gas_remaining() {
    let result = single_default_execution!("hypervisor_context_info_gas_remaining");

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
