use crate::callsystem::*;
use crate::codata::*;
use crate::interface::*;
use crate::narm::narmvm::*;
use crate::narm::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use neutron_common::RecoverableError;
use std::cmp;

/*

For documentation see https://neutron.earlgrey.tech/spec/neutron-arm-vm

*/

#[derive(Default)]
pub struct NarmHypervisor {
    vm: NarmVM,
    errored: bool,
    result: Option<u64>,
    error: Option<u64>,
}

enum HypervisorState {
    Ended,
    ElementCall(u32, u32),
    Error(NeutronError),
}

impl NarmHypervisor {
    fn wrapped_execute(&mut self, codata: &mut CoData) -> Result<HypervisorState, NarmError> {
        let res_low = &LongRegister { register: 0 };
        let res_high = &LongRegister { register: 1 };
        if self.result.is_some() {
            let result = self.result.unwrap();
            self.vm.set_reg(res_low, (result & 0xFFFF_FFFF) as u32);
            self.vm
                .set_reg(res_high, ((result & 0xFFFF_FFFF_0000_0000) >> 32) as u32);
            self.result = None;
        }
        //note: error will overwrite a result
        if self.error.is_some() {
            //always set top 32nd bit of error (most errors will only be 32 bits)
            let error = self.error.unwrap() | 0x8000_0000;
            self.vm.set_reg(res_low, (error & 0xFFFF_FFFF) as u32);
            self.vm
                .set_reg(res_high, ((error & 0xFFFF_FFFF_0000_0000) >> 32) as u32);
            //should a flag be set here?
            self.error = None;
        }
        loop {
            self.vm.gas_remaining = codata.gas_remaining;
            let syscall = self.vm.execute()?;
            codata.gas_remaining = self.vm.gas_remaining;
            match syscall {
                0xFF => {
                    return Ok(HypervisorState::Ended);
                }
                
                0xFE => {
                    return Ok(HypervisorState::Error(NeutronError::Recoverable(
                        RecoverableError::ContractRevertedExecution,
                    )));
                }
                
                0x20 => {
                    return Ok(HypervisorState::ElementCall(
                        self.vm.external_get_reg(0),
                        self.vm.external_get_reg(1),
                    ));
                }

                //SVC 0x10: push_costack (buffer: pointer, size: u32)
                0x10 => {
                    let address = self.vm.external_get_reg(0);
                    let size = self.vm.external_get_reg(1);

                    let data = self.vm.memory.get_sized_memory(address, size)?;
                    match codata.push_output_stack(data) {
                        Ok(_) => {}
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    }
                }
                
                //SVC 0x11: pop_costack (buffer: pointer, max_size: u32) -> actual_size: u32 -- note: if buffer and max_size is 0, then the item will be popped without copying the item to memory and only the actual_size will be returned
                0x11 => {
                    let address = self.vm.external_get_reg(0);
                    let max_size = self.vm.external_get_reg(1);
                    let data = match codata.pop_input_stack() {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };
                    self.vm.external_set_reg(0, data.len() as u32);
                    if max_size != 0 {
                        self.vm.copy_into_memory(
                            address,
                            &data[0..(cmp::min(data.len(), max_size as usize))],
                        )?;
                    }
                }
                
                //SVC 0x14: clear_costack()
                0x14 => {
                    codata.clear_input_stack();
                }

                //SVC 0x40: push_raw_output(key: stack [u8], data: stack [u8])
                //Pop two values from costack and push as key and value to comap
                0x40 => {
                    // Since key and data is pushed in the "correct" order we pop the other way around
                    let data = match codata.pop_input_stack() {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };
                    let key = match codata.pop_input_stack() {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };

                    match codata.push_output_key(&key, &data) {
                        Ok(_) => {}
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    }
                }

                //SVC 0x42: peek_raw_input(key: stack [u8], max_size: u32) -> data: stack [u8]
                //Pop one value from costack and use as key to get input comap value, which is then pushed to costack (limited to provided max size)
                0x42 => {
                    let max_size = self.vm.external_get_reg(0) as usize;
                    let key = match codata.pop_input_stack() {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };

                    let mut data = match codata.peek_input_key(&key) {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };

                    // Truncate the data if above provided max size
                    data.truncate(max_size);

                    match codata.push_output_stack(&data) {
                        Ok(_) => {}
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    }
                }

                //SVC 0x44: peek_raw_result(key: stack [u8], max_size: u32) -> data: stack [u8]
                //Pop one value from costack and use as key to get result comap value, which is then pushed to costack (limited to provided max size)
                0x44 => {
                    let max_size = self.vm.external_get_reg(0) as usize;
                    let key = match codata.pop_input_stack() {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };

                    let mut data = match codata.peek_result_key(&key) {
                        Ok(d) => d,
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    };

                    // Truncate the data if above provided max size
                    data.truncate(max_size);

                    match codata.push_output_stack(&data) {
                        Ok(_) => {}
                        Err(e) => {
                            return Ok(HypervisorState::Error(e));
                        }
                    }
                }

                0 => {
                    assert!(false, "this should never happen");
                }
                _ => {
                    println!("Unknown element call: {}", syscall);
                }
            }
        }
    }
}

impl VMHypervisor for NarmHypervisor {
    fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError> {
        match self.wrapped_execute(codata) {
            Ok(v) => {
                match v {
                    HypervisorState::Ended => {
                        return Ok(VMResult::Ended(
                            self.vm.external_get_reg(0) & (!0x8000_0000),
                        )); //Bottom 31 bits of r0 is the "status code" of the contract
                    }
                    HypervisorState::ElementCall(element, function) => {
                        return Ok(VMResult::ElementCall(element, function));
                    }
                    HypervisorState::Error(e) => {
                        return Err(e);
                    }
                };
            }
            Err(e) => {
                dbg!(&e);
                println!("{}", self.vm.get_diagnostics_message());
                return Err(NeutronError::Recoverable(
                    RecoverableError::ContractExecutionError,
                )); //TODO, decode into useful info
            }
        }
    }

    fn set_result(&mut self, code: u64) {
        self.result = Some(code);
    }
    fn set_error(&mut self, code: u64) {
        self.error = Some(code);
    }
    /// Creates the initial state, including potentially storing state to the database, decoding of bytecode, etc
    fn enter_state(
        &mut self,
        codata: &mut CoData,
        callsystem: &CallSystem,
    ) -> Result<(), NeutronError> {
        let execution_type = codata.peek_context(0)?.execution_type;
        if execution_type == ExecutionType::Deploy && !codata.permissions().access_self {
            return Err(NeutronError::Recoverable(
                RecoverableError::PureCallOfImpureContract,
            ));
        }
        if execution_type == ExecutionType::Deploy {
            codata.permissions().assert_has_self_modification()?;
        }
        //TODO check flags for "can contract be upgraded" and if so and a pure call then return PureCallOfImpureContract
        let mut storage = callsystem.global_storage.as_ref().unwrap().borrow_mut();
        let code = match execution_type {
            ExecutionType::Call => storage.private_load_state(codata, &[0x02, 0])?,
            _ => codata.peek_input_key("!.c".as_bytes())?,
        };
        self.vm
            .memory
            .add_memory(0x1_0000, code.len() as u32)
            .unwrap();
        match self.vm.copy_into_memory(0x1_0000, &code) {
            Err(_) => {
                return Err(NeutronError::Unrecoverable(
                    UnrecoverableError::ErrorInitializingVM,
                ));
            }
            _ => {}
        }
        let data = match execution_type {
            ExecutionType::Call => {
                codata.ignore_permissions = true;
                let v = storage.private_load_state(codata, &[0x02, 0x10]);
                codata.ignore_permissions = false;
                v?
            }
            _ => codata.peek_input_key("!.d".as_bytes())?,
        };
        self.vm
            .memory
            .add_memory(0x8001_0000, data.len() as u32)
            .unwrap();
        match self.vm.copy_into_memory(0x8001_0000, &data) {
            Err(_) => {
                return Err(NeutronError::Unrecoverable(
                    UnrecoverableError::ErrorInitializingVM,
                ));
            }
            _ => {}
        }

        match execution_type {
            ExecutionType::Deploy => {
                storage.private_store_state(codata, &[0x02, 0x00], &code)?;
                storage.private_store_state(codata, &[0x02, 0x10], &data)?;
            }
            _ => {}
        };
        self.vm.memory.add_memory(0x8100_0000, 0xFFFF).unwrap();

        //do init stuff
        self.vm.set_thumb_pc_address(0x1_0000);
        Ok(())
    }
    /// Called when exiting the VM, should commit state etc
    fn exit_state(
        &mut self,
        codata: &mut CoData,
        callsystem: &CallSystem,
    ) -> Result<(), NeutronError> {
        let mut storage = callsystem.global_storage.as_ref().unwrap().borrow_mut();
        if self.errored {
            storage.revert_checkpoint(codata)?;
        } else {
            storage.commit_checkpoint(codata)?;
        }
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
    fn test_adding_vm() {
        let f = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        let mut vmm = VMManager::default();
        vmm.vm_builders.insert(2, f);
    }
}
