use crate::codata::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;

#[derive(Default)]
struct Manager{
    vms: Vec<Box<dyn VMHypervisor>>
}

impl Manager{
    fn execute_neutron(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem, vmm: &VMManager) -> Result<(), NeutronError>{
        let callback_element = 0;
        let callback_function = 0;
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
                    match callsystem.call(codata, element, function){
                        Ok(v) => {
                            match v{
                                ElementResult::Result(result) => {
                                    hypervisor.set_result(result);
                                },
                                ElementResult::NewCall => {
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
        self.vms.pop().unwrap();
        Ok(())
    }
}