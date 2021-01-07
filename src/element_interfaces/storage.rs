use crate::codata::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
use crate::callsystem::*;
use neutron_star_constants::*;
/*
## Global Storage

ID: 2

Functions:

* store_state(key, value) -> ()
* load_state(key) -> (value)
* key_exists(key) -> (bool)
*/

pub const GLOBAL_STORAGE_FEATURE: u32 = 2;

#[derive(FromPrimitive)]
pub enum GlobalStateFunctions{
    Available = 0, //reserved??
    StoreState = 1,
    LoadState,
    KeyExists,
}

impl <'a>ElementAPI for (dyn GlobalState +'a){
    fn system_call(&mut self, callsystem: & CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        self.try_syscall(codata, feature, function)
    }
}

pub trait GlobalState{
    fn try_syscall(&mut self, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        if feature != GLOBAL_STORAGE_FEATURE{
            return Ok(ElementResult::Result(0));
        }
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none(){
            return Err(Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let f=f.unwrap();
        match f{
            GlobalStateFunctions::KeyExists => {
                let key = codata.pop_stack()?;
                let result = if self.key_exists(codata, &key)?{
                    1
                } else{
                    0
                };
                codata.push_stack(&[result])?;
                Ok(ElementResult::Result(0))
            },
            GlobalStateFunctions::LoadState => {
                let key = codata.pop_stack()?;
                let value = self.load_state(codata, &key)?;
                codata.push_stack(&value)?;
                Ok(ElementResult::Result(0))
            },
            GlobalStateFunctions::StoreState => {
                let key = codata.pop_stack()?;
                let value = codata.pop_stack()?;
                self.store_state(codata, &key, &value)?;
                Ok(ElementResult::Result(0))
            }
            GlobalStateFunctions::Available => {
                Ok(ElementResult::Result(0))
            },
            _ => {
                Ok(ElementResult::Result(0))
            } //todo
        }
    }
    fn store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn key_exists(&mut self, codata: &mut CoData, key: &[u8]) -> Result<bool, NeutronError>;

    fn private_store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn private_load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;

    //do these belong here? They could be done by using a single struct, but impl on two traits. However, this could bring refcell problems
    fn transfer_balance(&mut self, codata: &mut CoData, address: NeutronFullAddress, value: u64) -> Result<u64, NeutronError>;
    fn get_balance(&mut self, codata: &mut CoData) -> Result<u64, NeutronError>;
    
    fn create_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
    fn revert_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
    fn commit_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
}

