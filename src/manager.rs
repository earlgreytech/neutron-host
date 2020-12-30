use crate::codata::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;

#[derive(Default)]
struct Manager{
    //vms: Vec<Box<dyn VMHypervisor>>
}

impl Manager{
    pub fn execute_neutron(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem, vmm: &VMManager) -> Result<(), NeutronError>{
        assert!(codata.context_count() > 0);
        assert!(vmm.vm_builders.len() > 0);
        let context = codata.peek_context(0)?;
        let mut hypervisor = match vmm.vm_builders.get(&context.self_address.version){
            Some(v) => v,
            None => {
                return Err(NeutronError::Recoverable(RecoverableError::InvalidVM));
            }
        }(); //should this be in a box?
        loop{
            let result = match hypervisor.execute(codata){
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            };
            match result{
                VMResult::Ended => {
                    break;
                },
                VMResult::ElementCall(element, function) => {
                    codata.enter_element();
                    match callsystem.call(codata, element, function){
                        Ok(v) => {
                            match v{
                                ElementResult::Result(result) => {
                                codata.exit_element();
                                    hypervisor.set_result(result);
                                },
                                ElementResult::NewCall => {
                                    codata.exit_element();
                                    match self.execute_neutron(codata, callsystem, vmm){
                                        Err(NeutronError::Recoverable(e)) => {
                                            hypervisor.set_error(e as u32);
                                        },
                                        Err(NeutronError::Unrecoverable(e)) => {
                                            return Err(NeutronError::Unrecoverable(e));
                                        },
                                        Ok(_) =>{
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            codata.exit_element();
                            match e{
                                NeutronError::Recoverable(v) => {
                                    hypervisor.set_error(v as u32);
                                },
                                NeutronError::Unrecoverable(e) => {
                                    return Err(NeutronError::Unrecoverable(e));
                                }
                            }
                        }
                    }
                }
            };
        }
        codata.enter_element();
        //gather and push call/execution results etc
        codata.exit_element();
        codata.pop_context().unwrap();
        //self.vms.pop().unwrap();
        Ok(())
    }
}