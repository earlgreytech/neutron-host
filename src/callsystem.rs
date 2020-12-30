use crate::neutronerror::*;
use std::collections::hash_map::*;
use crate::codata::*;
use std::cell::*;

pub enum ElementResult{
    Result(u32),
    NewCall
}

pub trait CallSystem{
    fn call(&self, manager: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>;
}

pub trait ElementAPI{
    fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>;
}

/// Manages ElementAPIs. This structure is only provided for convenience and not necessarily a required structure
/// Using it disallows any reentrancy within the call stack of elements (giving a runtime error)
#[derive(Default)]
pub struct RefCallSystem{
    elements: HashMap<u32, RefCell<Box<dyn ElementAPI>>>
}

impl RefCallSystem{
    pub fn add_call(&mut self, number: u32, element: Box<dyn ElementAPI>){
        self.elements.insert(number, RefCell::from(element));
    }
}
impl CallSystem for RefCallSystem{
    fn call(&self, manager: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        let mut t = self.elements.get(&feature).unwrap().borrow_mut();
        t.system_call(self, manager, feature, function)
    }
}


#[cfg(test)]
mod tests {
    #[derive(Default)]
    struct TestElementA{
    }
    impl ElementAPI for TestElementA{
        fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 2, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementB{
    }
    impl ElementAPI for TestElementB{
        fn system_call(&mut self, _callsystem: &dyn CallSystem, _manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFail{
    }
    impl ElementAPI for TestElementFail{
        fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 3, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFailA{
    }
    impl ElementAPI for TestElementFailA{
        fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 5, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementFailB{
    }
    impl ElementAPI for TestElementFailB{
        fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, _feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            callsystem.call(manager, 4, 0);
            Ok(ElementResult::Result(0))
        }
    }
    #[derive(Default)]
    struct TestElementC{
        test: u32
    }
    impl ElementAPI for TestElementC{
        fn system_call(&mut self, callsystem: &dyn CallSystem, manager: &mut CoData, feature: u32, _function: u32) -> Result<ElementResult, NeutronError>{
            self.test = feature;
            callsystem.call(manager, 1, 0);
            callsystem.call(manager, 1, 0);
            Ok(ElementResult::Result(0))
        }
    }
    use super::*;
    #[test]
    fn test_borrowing(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let mut cs = RefCallSystem::default();
        cs.add_call(1, t1);
        cs.add_call(2, t2);
        let mut codata = CoData::default();
        cs.call(&mut codata, 1, 0);
    }
    #[test]
    fn test_borrowing_back_and_forth(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementC::default());
        let mut cs = RefCallSystem::default();
        cs.add_call(1, t1);
        cs.add_call(2, t2);
        cs.add_call(3, t3);
        let mut codata = CoData::default();
        cs.call(&mut codata, 3, 0);
    }
    #[test]
    #[should_panic]
    fn test_borrowing_should_fail(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementFail::default());
        let mut cs = RefCallSystem::default();
        cs.add_call(1, t1);
        cs.add_call(2, t2);
        cs.add_call(3, t3);
        let mut codata = CoData::default();
        cs.call(&mut codata, 3, 0);
    }
    #[test]
    #[should_panic]
    fn test_borrowing_should_fail_extended(){
        let t1 = Box::new(TestElementA::default());
        let t2 = Box::new(TestElementB::default());
        let t3 = Box::new(TestElementFail::default());
        let t4 = Box::new(TestElementFailA::default());
        let t5 = Box::new(TestElementFailB::default());
        let mut cs = RefCallSystem::default();
        cs.add_call(1, t1);
        cs.add_call(2, t2);
        cs.add_call(3, t3);
        cs.add_call(4, t4);
        cs.add_call(5, t5);
        let mut codata = CoData::default();
        cs.call(&mut codata, 5, 0);
    }
}

