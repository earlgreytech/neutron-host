mod harness;

// Every subfolder with tests need to be defined as modules here, otherwise they won't be executed
mod test_comap;
mod test_debugdata;
mod test_smoke;
mod example_new_element;

extern crate num;
#[macro_use]
extern crate num_derive;
