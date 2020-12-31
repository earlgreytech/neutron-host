use crate::narm::narmvm::*;
use crate::codata::*;
use std::collections::hash_map::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;



#[derive(Default)]
pub struct NarmHypervisor{
    vm: NarmVM
}

impl NarmHypervisor{

}

impl VMHypervisor for NarmHypervisor{
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>{
        Ok(VMResult::Ended)
    }
    fn set_result(&mut self, code: u32){

    }
    fn set_error(&mut self, code: u32){

    }
    /// Creates the initial state, including potentially storing state to the database, decoding of bytecode, etc
    fn enter_state(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem) -> Result<(), NeutronError>{
        Ok(())
    }
    /// Called when exiting the VM, should commit state etc
    fn exit_state(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem) -> Result<(), NeutronError>{
        Ok(())
    }
}



/*

Needed blockchain workflow:

Create relevant ElementAPIs for blockchain
Create callsystem and populate with relevant IDs
Create VM/hypervisor "builders" and populate into VMManger

foreach transaction{
    Create codata, populate with transaction ABI data
    Call NeutronManager to begin execution
}

Workflow for sub-call:

pause execution, returning VMResult
enter element to handle call arguments etc
codata is shifted around to add new top context
new hypervisor added to top context
exit element
resume execution (now top context will cause execution of the sub-call)
When it returns..
pop top context, leave relevant data on stacks
resume execution again?? (but somehow enter element again to organize data properly??)

open questions. Who owns hypervisors? 

*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adding_vm(){
        let f = || -> Box<dyn VMHypervisor>{
            Box::from(NarmHypervisor::default())
        };
        let mut vmm = VMManager::default();
        vmm.vm_builders.insert(2, f);
    }
}




