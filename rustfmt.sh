# This script runs rustfmt for the parts of the repo that uses it

rustfmt ./tests/*.rs
rustfmt ./tests/contracts/default_env/src/bin/*.rs
rustfmt ./src/element_interfaces/debug_data.rs
rustfmt ./src/narm_hypervisor.rs
rustfmt ./src/comap_abi_decoder.rs
rustfmt ./src/harness.rs
