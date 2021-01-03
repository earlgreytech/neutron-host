use crate::codata::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
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
pub enum GlobalStorageFunctions{
    Available = 0, //reserved??
    StoreState = 1,
    LoadState,
    KeyExists,
    //private functions
    LoadPrivateState = 0x8000_0001,
    StorePrivateState = 0x8000_0002,

}

pub trait GlobalStorage{
    fn try_syscall(&mut self, codata: &mut CoData, feature: u32, function: u32) -> Result<bool, NeutronError>{
        if feature != GLOBAL_STORAGE_FEATURE{
            return Ok(false);
        }
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none(){
            return Err(Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let f=f.unwrap();
        let result = match f{
            GlobalStorageFunctions::KeyExists => {
                let key = codata.pop_stack()?;
                let result = if self.key_exists(codata, &key)?{
                    1
                } else{
                    0
                };
                codata.push_stack(&[result])?;
                Ok(())
            },
            GlobalStorageFunctions::LoadState => {
                let key = codata.pop_stack()?;
                let value = self.load_state(codata, &key)?;
                codata.push_stack(&value)?;
                Ok(())
            },
            GlobalStorageFunctions::StoreState => {
                let key = codata.pop_stack()?;
                let value = codata.pop_stack()?;
                self.store_state(codata, &key, &value)?;
                Ok(())
            }
            GlobalStorageFunctions::Available => {
                Ok(())
            },
            _ => { Ok(()) } //todo
        };
        if result.is_err(){
            Err(result.unwrap_err())
        }else{
            Ok(true)
        }
    }
    fn store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn key_exists(&mut self, codata: &mut CoData, key: &[u8]) -> Result<bool, NeutronError>;

    fn private_store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn private_load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
}

