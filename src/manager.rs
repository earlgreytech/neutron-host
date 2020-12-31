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
    fn neutron_main_loop(&mut self, hypervisor: &mut Box<dyn VMHypervisor>, codata: &mut CoData, callsystem: &dyn CallSystem, vmm: &VMManager) -> Result<VMResult, NeutronError>{
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
                    codata.enter_element();
                    match callsystem.call(codata, element, function){
                        Ok(v) => {
                            codata.exit_element();
                            match v{
                                ElementResult::Result(result) => {
                                    hypervisor.set_result(result);
                                },
                                ElementResult::NewCall => {
                                    match self.execute(codata, callsystem, vmm){
                                        Err(NeutronError::Recoverable(e)) => {
                                            dbg!(&e);
                                            hypervisor.set_error(e as u32);
                                        },
                                        Err(NeutronError::Unrecoverable(e)) => {
                                            dbg!(&e);
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
                                    dbg!(&v);
                                    hypervisor.set_error(v as u32);
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

    pub fn execute(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem, vmm: &VMManager) -> Result<(), NeutronError>{
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

    #[derive(Default)]
    struct TestVM{
    }
    impl VMHypervisor for TestVM{
        fn execute(&mut self, codata: &mut CoData) -> Result<VMResult, NeutronError>{
            match codata.pop_stack().unwrap_or(vec![0])[0]{
                1 => {
                    codata.push_key(&[1], &[1])?;
                    return Ok(VMResult::Ended);
                },
                _ => {}
            }
            if codata.peek_key(&[0])?[0] == 0{
                return Ok(VMResult::ElementCall(1, 1));
            }
            return Ok(VMResult::ElementCall(0, 0));
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

    #[derive(Default)]
    struct TestElement{
    }
    impl ElementAPI for TestElement{
        fn system_call(&mut self, callsystem: &dyn CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
            match function{
                0 => {
                    codata.push_stack(&[1])?;
                },
                _ => {}
            }
            Ok(ElementResult::Result(0))
        }
    }

    #[test]
    fn test_bare_behavior_correct(){
        let mut codata = CoData::new();
        codata.push_key(&[0], &[0]).unwrap();
        let mut callsystem = RefCallSystem::default();
        let element = TestElement::default();
        callsystem.add_call(1, Box::from(element));

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
}







