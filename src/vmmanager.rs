use crate::neutronerror::*;
use crate::codata::*;
use std::collections::hash_map::*;

pub trait VMHypervisor{
    //note: hypervisors should own the relevant VM. Each execution (ie, sub-call etc) will produce a new VMHypervisor
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>;
    fn set_result(&mut self, result: u32);
    fn set_error(&mut self, code: u32);
}

pub enum VMResult{
    Ended,
    ElementCall(u32, u32)
}

#[derive(Default)]
pub struct VMManager{
    pub vm_builders: HashMap<u32, fn() -> Box<dyn VMHypervisor>>
}



