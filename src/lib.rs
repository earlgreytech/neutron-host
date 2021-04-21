pub mod interface;
pub mod db;
pub mod element_interfaces;
pub mod testbench;
pub mod codata;
pub mod neutronerror;
pub mod narm_hypervisor;
pub mod callsystem;
pub mod vmmanager;
pub mod manager;
pub mod harness;
pub extern crate neutron_common as addressing;

extern crate num;
#[macro_use]
extern crate num_derive;

extern crate narm;

use rand::Rng;

pub fn reset_to_random_address(address: &mut crate::addressing::NeutronAddress) {
    address.data.copy_from_slice(&rand::thread_rng().gen::<[u8; 32]>()[0..20]);
}
pub fn new_random_address() -> crate::addressing::NeutronAddress{
    let mut a = crate::addressing::NeutronAddress::default();
    reset_to_random_address(&mut a);
    a
}

trait AddressDecoding{
    fn decode(&self) -> Vec<u8>;
}

impl AddressDecoding for crate::addressing::NeutronAddress{
    fn decode(&self) -> Vec<u8>{
        let mut v = Vec::with_capacity(4 + 20);
        v.extend(&self.version.to_le_bytes());
        v.extend(&self.data);
        v
    }
}
