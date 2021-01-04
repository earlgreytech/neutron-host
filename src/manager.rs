use crate::codata::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;

#[derive(Default)]
pub struct Manager{
}

impl Manager{
    /// Starts execution of a new Neutron instance, creating a new VM from the top context
    fn start_execution(&mut self, codata: &mut CoData, vmm: &VMManager) -> Result<Box<dyn VMHypervisor>, NeutronError>{
        assert!(codata.context_count() > 0);
        assert!(vmm.vm_builders.len() > 0);
        let context = codata.peek_context(0)?;
        let hypervisor = match vmm.vm_builders.get(&context.self_address.version){
            Some(v) => v,
            None => {
                return Err(NeutronError::Recoverable(RecoverableError::InvalidVM));
            }
        }(); //should this be in a box?
        Ok(Box::from(hypervisor))
    }
    /// Ends execution, pushing relevant execution results and destroying the top context
    fn end_execution(&mut self, codata: &mut CoData, _error: u32) -> Result<(), NeutronError>{
        codata.enter_element();
        //gather and push call/execution results etc?
        codata.exit_element();
        codata.pop_context()?;
        Ok(())
    }
    /// The main loop for handling a Neutron execution
    /// The VM is continually executed. Upon VM return, element calls will be handled, if a sub-contract is called, it'll cause a recursive execute() call. 
    /// The main loop will be exited either upon an unrecoverable error or upon the VM returning an "ended" result
    fn neutron_main_loop(&mut self, hypervisor: &mut Box<dyn VMHypervisor>, codata: &mut CoData, callsystem: & CallSystem, vmm: &VMManager) -> Result<VMResult, NeutronError>{
        callsystem.global_storage.as_ref().unwrap().borrow_mut().create_checkpoint(codata);
        loop{
            let result = match hypervisor.execute(codata){
                Ok(v) => v,
                Err(e) => {
                    dbg!(&e);
                    return Err(e);
                }
            };
            match result{
                VMResult::Ended => {
                    return Ok(VMResult::Ended);
                },
                VMResult::ElementCall(element, function) => {
                    match callsystem.call(codata, element, function){
                        Ok(v) => {
                            match v{
                                ElementResult::Result(result) => {
                                    hypervisor.set_result(result);
                                },
                                ElementResult::NewCall => {
                                    match self.execute(codata, callsystem, vmm){
                                        Err(NeutronError::Recoverable(e)) => {
                                            callsystem.global_storage.as_ref().unwrap().borrow_mut().revert_checkpoint(codata);
                                            dbg!(&e);
                                            hypervisor.set_error(e as u64);
                                        },
                                        Err(NeutronError::Unrecoverable(e)) => {
                                            dbg!(&e);
                                            return Err(NeutronError::Unrecoverable(e));
                                        },
                                        Ok(_) =>{
                                            callsystem.global_storage.as_ref().unwrap().borrow_mut().commit_checkpoint(codata);
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            match e{
                                NeutronError::Recoverable(v) => {
                                    dbg!(&v);
                                    hypervisor.set_error(v as u64);
                                },
                                NeutronError::Unrecoverable(e) => {
                                    dbg!(&e);
                                    return Err(NeutronError::Unrecoverable(e));
                                }
                            }
                        }
                    }
                }
            };
        }
    }

    pub fn execute(&mut self, codata: &mut CoData, callsystem: & CallSystem, vmm: &VMManager) -> Result<(), NeutronError>{
        let mut hv = &mut self.start_execution(codata, vmm)?;
        hv.enter_state(codata, callsystem)?;
        let mut error = 0;
        match self.neutron_main_loop(hv, codata, callsystem, vmm){
            Ok(v) => {
                assert!(v == VMResult::Ended); //element call should never escape
            },
            Err(e) => {
                match e{
                    NeutronError::Recoverable(e) => {
                        if codata.context_count() == 1 {
                            return Err(NeutronError::Unrecoverable(UnrecoverableError::TopLevelError(e)));
                        }else{
                            error = e as u32;
                        }
                    },
                    NeutronError::Unrecoverable(e) => {
                        //this leaves the entire structure in-tact for inspection
                        return Err(NeutronError::Unrecoverable(e));
                    }
                };
            }
        };
        self.end_execution(codata, error)?;
        hv.exit_state(codata, callsystem)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neutron_star_constants::NeutronFullAddress;
    use std::cell::RefCell;

    #[derive(Default)]
    struct TestVM{
        self_calls: usize
    }
    impl VMHypervisor for TestVM{
        fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>{
            self.self_calls += 1;
            match codata.pop_stack().unwrap_or(vec![0])[0]{
                1 => {
                    codata.push_key(&[1], &[1])?;
                    return Ok(VMResult::Ended);
                },
                _ => {}
            }
            match codata.peek_key(&[0])?[0]{
                0 => {
                    assert_eq!(self.self_calls, 1);
                    return Ok(VMResult::ElementCall(1, 0));
                },
                1 => {
                    let result = codata.peek_result_key(&[2]).unwrap_or(vec![0])[0];
                    if result == 3{
                        //returns from call 2
                        codata.push_key(&[3], &[1])?;
                        return Ok(VMResult::Ended);
                    }else{
                        //will call 2
                        assert!(self.self_calls <= 10);
                        codata.push_key(&[0], &[2])?;
                        codata.push_key(&[1], &[2])?;
                        return Ok(VMResult::ElementCall(1, 2));
                    }
                },
                2 => {
                    assert_eq!(self.self_calls, 1);
                    //sub-contract call
                    assert!(codata.peek_key(&[1]).unwrap()[0] == 2);
                    codata.push_key(&[2], &[3])?;
                    return Ok(VMResult::Ended);
                },
                3 => {
                    //test sub-call error, will call 4
                    codata.push_key(&[0], &[4])?;
                    return Ok(VMResult::ElementCall(1, 2));
                },
                4 => {
                    assert_eq!(self.self_calls, 1);
                    //sub-contract call 2
                    codata.push_key(&[2], &[4])?;
                    return Err(NeutronError::Unrecoverable(UnrecoverableError::StateOutOfRent));
                }
                _ => {
                    assert!(false);
                }
            }
            assert!(false);
            //return Ok(VMResult::ElementCall(0, 0));
            return Err(NeutronError::Unrecoverable(UnrecoverableError::ErrorInitializingVM));
        }
        fn set_result(&mut self, code: u64){
    
        }
        fn set_error(&mut self, code: u64){
    
        }
        /// Creates the initial state, including potentially storing state to the database, decoding of bytecode, etc
        fn enter_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>{
            Ok(())
        }
        /// Called when exiting the VM, should commit state etc
        fn exit_state(&mut self, codata: &mut CoData, callsystem: & CallSystem) -> Result<(), NeutronError>{
            Ok(())
        }
    }
    #[derive(Default)]
    struct TestStorageElement{}
    impl crate::syscall_interfaces::storage::GlobalStorage for TestStorageElement{
        fn store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{Ok(())}
        fn load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>{Ok((vec![0]))}
        fn key_exists(&mut self, codata: &mut CoData, key: &[u8]) -> Result<bool, NeutronError>{Ok((false))}
    
        fn private_store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{Ok(())}
        fn private_load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>{Ok((vec![0]))}
    
        //do these belong here? They could be done by using a single struct, but impl on two traits. However, this could bring refcell problems
        fn transfer_balance(&mut self, codata: &mut CoData, address: NeutronFullAddress, value: u64) -> Result<u64, NeutronError>{Ok(0)}
        fn get_balance(&mut self, codata: &mut CoData) -> Result<u64, NeutronError>{Ok(0)}
        
        fn create_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>{Ok(())}
        fn revert_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>{Ok(())}
        fn commit_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>{Ok(())}
    }
    impl ElementAPI for TestStorageElement{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{Ok(ElementResult::Result(0))}
    }

    #[derive(Default)]
    struct TestElement{
    }
    impl ElementAPI for TestElement{
        fn system_call(&mut self, callsystem: & CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
            codata.enter_element();
            assert_eq!(feature, 1);
            match function{
                0 => {
                    codata.push_stack(&[1])?;
                },
                2 => {
                    let mut context = crate::interface::ExecutionContext::default();
                    context.self_address.version = 1;
                    codata.exit_element(); //TODO can this be cleaner?
                    codata.push_context(context)?;
                    codata.enter_element();
                    codata.exit_element();
                    return Ok(ElementResult::NewCall);
                },
                3 => {
                    let mut context = crate::interface::ExecutionContext::default();
                    context.self_address.version = 1;
                    codata.exit_element(); //TODO can this be cleaner?
                    codata.push_context(context)?;
                    codata.enter_element();
                    codata.exit_element();
                    return Ok(ElementResult::NewCall);
                }
                _ => {
                    assert!(false);
                }
            }
            codata.exit_element();
            Ok(ElementResult::Result(0))
        }
    }

    #[test]
    fn test_bare_behavior_correct(){
        let mut codata = CoData::new();
        codata.push_key(&[0], &[0]).unwrap();
        let mut callsystem = CallSystem::default();
        let element = TestElement::default();
        callsystem.add_call(1, Box::from(element));
        let storage = TestStorageElement::default();
        callsystem.global_storage = Some(RefCell::from(Box::from(storage)));

        let testvm = || -> Box<dyn VMHypervisor>{
            Box::from(TestVM::default())
        };
        let mut vmm = VMManager::default();
        vmm.vm_builders.insert(1, testvm);

        let mut manager = Manager::default();
        let mut context = crate::interface::ExecutionContext::default();
        context.self_address.version = 1;
        codata.push_context(context).unwrap();

        manager.execute(&mut codata, &callsystem, &vmm).unwrap();
        assert!(codata.peek_key(&[0]).is_err());
        assert!(codata.peek_key(&[1]).unwrap()[0] == 1);
        assert!(codata.peek_result_key(&[0]).is_err());
        assert!(codata.peek_result_key(&[1]).unwrap()[0] == 1);
    }
    
    #[test]
    fn test_single_call_behavior_correct(){
        let mut codata = CoData::new();
        codata.push_key(&[0], &[1]).unwrap();
        let mut callsystem = CallSystem::default();
        let element = TestElement::default();
        callsystem.add_call(1, Box::from(element));
        let storage = TestStorageElement::default();
        callsystem.global_storage = Some(RefCell::from(Box::from(storage)));

        let testvm = || -> Box<dyn VMHypervisor>{
            Box::from(TestVM::default())
        };
        let mut vmm = VMManager::default();
        vmm.vm_builders.insert(1, testvm);

        let mut manager = Manager::default();
        let mut context = crate::interface::ExecutionContext::default();
        context.self_address.version = 1;
        codata.push_context(context).unwrap();

        manager.execute(&mut codata, &callsystem, &vmm).unwrap();
        assert!(codata.peek_key(&[3]).unwrap()[0] == 1);
        assert!(codata.peek_result_key(&[3]).unwrap()[0] == 1);
        //assert!(codata.peek_result_key(&[0]).is_err()); //TODO is this correct?
    }
    
    #[test]
    fn test_single_call_recoverable_error_behavior_correct(){
        let mut codata = CoData::new();
        codata.push_key(&[0], &[3]).unwrap();
        let mut callsystem = CallSystem::default();
        let element = TestElement::default();
        callsystem.add_call(1, Box::from(element));
        let storage = TestStorageElement::default();
        callsystem.global_storage = Some(RefCell::from(Box::from(storage)));

        let testvm = || -> Box<dyn VMHypervisor>{
            Box::from(TestVM::default())
        };
        let mut vmm = VMManager::default();
        vmm.vm_builders.insert(1, testvm);

        let mut manager = Manager::default();
        let mut context = crate::interface::ExecutionContext::default();
        context.self_address.version = 1;
        codata.push_context(context).unwrap();

        assert!(manager.execute(&mut codata, &callsystem, &vmm).is_err());
    }
    
}







