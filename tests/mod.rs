// Every subfolder with tests need to be defined as modules here, otherwise they won't be executed
mod common;
mod example_deploy_call;
mod example_new_element;
mod general_smoke;
mod hypervisor_comap;
mod hypervisor_comap_abi;
mod hypervisor_costack;

extern crate num;
#[macro_use]
extern crate num_derive;
