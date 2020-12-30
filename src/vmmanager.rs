use crate::neutronerror::*;
use crate::codata::*;
use std::collections::hash_map::*;

pub trait VMHypervisor{
    //note: hypervisors should own the relevant VM. Each execution (ie, sub-call etc) will produce a new VMHypervisor
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>;
}

pub enum VMResult{
    Ended,
    ElementCall(u32)
}

#[derive(Default)]
pub struct VMManager{
    pub vm_builders: HashMap<u32, fn() -> Box<dyn VMHypervisor>>
}

