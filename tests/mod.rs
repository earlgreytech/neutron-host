// Every subfolder with tests need to be defined as modules here, otherwise they won't be executed
mod example_deploy_call;
mod example_new_element;
mod test_comap;
mod test_comap_abi;
mod test_debugdata;
mod test_smoke;

extern crate num;
#[macro_use]
extern crate num_derive;
