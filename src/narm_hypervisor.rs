use crate::narm::*;
use crate::codata::*;
use std::collections::hash_map::*;

pub trait ElementAPI{
    fn system_call(&mut self, manager: &mut CoData, feature: u32, function: u32);
}

#[derive(Default)]
pub struct TestElement{
    pub tmp: Vec<u32>
}

impl ElementAPI for TestElement{
    fn system_call(&mut self, manager: &mut CoData, feature: u32, function: u32){
        
    }
}

#[derive(Default)]
pub struct NarmHypervisor{
    pub tmp: HashMap<u32, Box<dyn ElementAPI>>
}

impl NarmHypervisor{

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_borrowing(){
        let t = Box::new(TestElement::default());
        let mut hv = NarmHypervisor::default();
        hv.tmp.insert(1, t);
    }
}



