use crate::neutronerror::*;
use crate::codata::*;
use crate::callsystem::*;
use std::collections::hash_map::*;

pub trait VMHypervisor{
    /// Creates the initial state, including potentially storing state to the database, decoding of bytecode, etc
    fn enter_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>;
    //note: hypervisors should own the relevant VM. Each execution (ie, sub-call etc) will produce a new VMHypervisor
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>;
    fn set_result(&mut self, result: u64);
    fn set_error(&mut self, code: u64);
    /// Called when exiting the VM, should commit state etc
    fn exit_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>;
}

#[derive(PartialEq, Debug)]
pub enum VMResult{
    Ended,
    ElementCall(u32, u32)
}

#[derive(Default)]
pub struct VMManager{
    pub vm_builders: HashMap<u32, fn() -> Box<dyn VMHypervisor>>
}



