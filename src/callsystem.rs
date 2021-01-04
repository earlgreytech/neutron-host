use crate::neutronerror::*;
use std::collections::hash_map::*;
use crate::codata::*;
use std::cell::*;
use crate::element_interfaces::storage::*;
use crate::element_interfaces::logging::*;

pub enum ElementResult{
    Result(u64),
    NewCall
}


pub trait ElementAPI{
    fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>;
}

/// Manages ElementAPIs. This structure is only provided for convenience and not necessarily a required structure
/// Using it disallows any reentrancy within the call stack of elements (giving a runtime error)
#[derive(Default)]
pub struct CallSystem{
    elements: HashMap<u32, RefCell<Box<dyn ElementAPI>>>,
    pub global_storage: Option<RefCell<Box<dyn GlobalStorage>>>,
    pub logging: Option<RefCell<Box<dyn LoggingInterface>>>
}

impl CallSystem{
    pub fn add_call(&mut self, number: u32, element: Box<dyn ElementAPI>) -> Result<(), NeutronError>{
        match number{
            GLOBAL_STORAGE_FEATURE => {
                return Err(NeutronError::Unrecoverable(UnrecoverableError::InvalidElementOperation));
            },
            LOGGING_FEATURE => {
                return Err(NeutronError::Unrecoverable(UnrecoverableError::InvalidElementOperation));
            },
            _ => {
                self.elements.insert(number, RefCell::from(element));
            }
        }
        Ok(())
    }
    
    pub fn call(&self, codata: &mut CoData, element: u32, function: u32) -> Result<ElementResult, NeutronError>{
        let function = function & (!0x8000_0000); //public calls can not set the top bit, which is reserved for private functions
        self.private_call(codata, element, function)
    }
    pub fn private_call(&self, manager: &mut CoData, element: u32, function: u32) -> Result<ElementResult, NeutronError>{
        match element{
            GLOBAL_STORAGE_FEATURE => {
                self.global_storage.as_ref().unwrap().borrow_mut().system_call(self, manager, element, function)
            },
            LOGGING_FEATURE => {
                self.logging.as_ref().unwrap().borrow_mut().system_call(self, manager, element, function)
            },
            _ => {
                let mut t = self.elements.get(&element).unwrap().borrow_mut();
                t.system_call(self, manager, element, function)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[derive(Default)]
    struct TestElementA{
    }
    impl ElementAPI for TestElementA{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 12, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementB{
    }
    impl ElementAPI for TestElementB{
        fn system_call(&mut self, _callsystem: & CallSystem, _manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFail{
    }
    impl ElementAPI for TestElementFail{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 13, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFailA{
    }
    impl ElementAPI for TestElementFailA{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 15, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFailB{
    }
    impl ElementAPI for TestElementFailB{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 14, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementC{
        test: u32
    }
    impl ElementAPI for TestElementC{
        fn system_call(&mut self, callsystem: & CallSystem, manager: &mut CoData, feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            self.test = feature;
            callsystem.call(manager, 11, 0);
            callsystem.call(manager, 11, 0);
            Ok(ElementResult::Result(0))
        }
    }
    use super::*;
    #[test]
    fn test_borrowing(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let mut cs = CallSystem::default();
        cs.add_call(11, t1);
        cs.add_call(12, t2);
        let mut codata = CoData::default();
        cs.call(&mut codata, 11, 0);
    }
    #[test]
    fn test_borrowing_back_and_forth(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementC::default());
        let mut cs = CallSystem::default();
        cs.add_call(11, t1);
        cs.add_call(12, t2);
        cs.add_call(13, t3);
        let mut codata = CoData::default();
        cs.call(&mut codata, 13, 0);
    }
    #[test]
    #[should_panic]
    fn test_borrowing_should_fail(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementFail::default());
        let mut cs = CallSystem::default();
        cs.add_call(11, t1);
        cs.add_call(12, t2);
        cs.add_call(13, t3);
        let mut codata = CoData::default();
        cs.call(&mut codata, 13, 0);
    }
    #[test]
    #[should_panic]
    fn test_borrowing_should_fail_extended(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementFail::default());
        let t4 = Box::new(TestElementFailA::default());
        let t5 = Box::new(TestElementFailB::default());
        let mut cs = CallSystem::default();
        cs.add_call(11, t1);
        cs.add_call(12, t2);
        cs.add_call(13, t3);
        cs.add_call(14, t4);
        cs.add_call(15, t5);
        let mut codata = CoData::default();
        cs.call(&mut codata, 15, 0);
    }
}

