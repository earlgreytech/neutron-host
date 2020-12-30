use crate::codata::*;
use crate::neutronerror::*;
use crate::vmmanager::*;
use crate::callsystem::*;

struct Manager{

}

impl Manager{
    fn execute_neutron(&mut self, codata: &mut CoData, callsystem: &dyn CallSystem, vmm: &VMManager) -> Result<(), NeutronError>{
        Ok(())
    }
}