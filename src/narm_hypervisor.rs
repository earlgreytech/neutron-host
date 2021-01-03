use crate::narm::narmvm::*;
use crate::narm::*;
use crate::codata::*;
use std::collections::hash_map::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;
use crate::interface::*;
use crate::syscall_interfaces::storage::*;



#[derive(Default)]
pub struct NarmHypervisor{
    vm: NarmVM
}

impl NarmHypervisor{

}

impl VMHypervisor for NarmHypervisor{
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>{
        let context = codata.peek_context(0)?;
        Ok(VMResult::Ended)
    }
    fn set_result(&mut self, code: u32){

    }
    fn set_error(&mut self, code: u32){

    }
    /// Creates the initial state, including potentially storing state to the database, decoding of bytecode, etc
    fn enter_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>{
        let execution_type = codata.peek_context(0)?.execution_type;
        {
            //let storage = callsystem.
        }
        let code = match execution_type{
            ExecutionType::Call => {
                codata.push_stack(&[0x02, 0]);
                callsystem.private_call(codata, GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::LoadPrivateState as u32)?;
                codata.pop_stack()?
            },
            _ => {
                codata.peek_key("!.c".as_bytes())?
            }
        };
        self.vm.memory.add_memory(0x10000, code.len() as u32);
        match self.vm.copy_into_memory(0x10000, &code){
            Err(_) => {
                return Err(NeutronError::Unrecoverable(UnrecoverableError::ErrorInitializingVM));
            },
            _ => {}
        }
        let data = match execution_type{
            ExecutionType::Call => {
                codata.push_stack(&[0x02, 0x10]);
                callsystem.private_call(codata, GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::LoadPrivateState as u32)?;
                codata.pop_stack()?
            },
            _ => {
                codata.peek_key("!.d".as_bytes())?
            }
        };
        self.vm.memory.add_memory(0x80010000, data.len() as u32);
        match self.vm.copy_into_memory(0x10000, &data){
            Err(_) => {
                return Err(NeutronError::Unrecoverable(UnrecoverableError::ErrorInitializingVM));
            },
            _ => {}
        }

        match execution_type{
            ExecutionType::Deploy => {
                codata.push_stack(&[0x02, 0x00])?; //key
                codata.push_stack(&code)?; //value
                callsystem.private_call(codata, GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::StorePrivateState as u32)?;
                codata.push_stack(&[0x02, 0x10])?; //key
                codata.push_stack(&data)?; //value
                callsystem.private_call(codata, GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::StorePrivateState as u32)?;
            },
            _ => {}
        };

        Ok(())
    }
    /// Called when exiting the VM, should commit state etc
    fn exit_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>{
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




