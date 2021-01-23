use crate::codata::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
use crate::callsystem::*;
use neutron_star_constants::*;
use std::convert::*;
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
    fn system_call(&mut self, _callsystem: & CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
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
            // _ => {
            //     Ok(ElementResult::Result(0))
            // } //todo
        }
    }
    fn store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn key_exists(&mut self, codata: &mut CoData, key: &[u8]) -> Result<bool, NeutronError>;

    fn private_store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    fn private_load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn private_load_state_external(&mut self, codata: &mut CoData, address: NeutronAddress, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn private_store_state_external(&mut self, codata: &mut CoData, address: NeutronAddress, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;

    fn create_token_transfer(&mut self, codata: &mut CoData, owner: NeutronAddress, id: u64, value: u64) -> Result<u64, NeutronError>{
        let c = codata.peek_context(0)?.clone();
        let balance = self.get_token_balance(codata, owner, id, c.self_address).unwrap_or(0);
        if balance < value{
            return Err(NeutronError::Recoverable(RecoverableError::LowTokenBalance));
        }
        codata.element_push_transfer(owner, id, value);
        Ok(balance)
    }
    fn claim_token_transfer(&mut self, codata: &mut CoData, owner: NeutronAddress, id: u64) -> Result<u64, NeutronError>{
        let c = codata.peek_context(0)?.clone();
        let self_key = build_token_key(c.self_address, id);
        let sender_key = build_token_key(c.sender, id);
        let self_balance = self.get_token_balance(codata, owner, id, c.self_address).unwrap_or(0);
        let value = codata.element_pop_transfer(owner, id).unwrap_or(0);
        let sender_balance = self.get_token_balance(codata, owner, id, c.sender)?;
        if sender_balance < value{
            return Err(Unrecoverable(UnrecoverableError::DeveloperError));
        }
        self.private_store_state_external(codata, owner, &self_key, &(self_balance + value).to_le_bytes())?;
        self.private_store_state_external(codata, owner, &sender_key, &(sender_balance - value).to_le_bytes())?;
        Ok(0)
    }
    
    fn get_token_balance(&mut self, codata: &mut CoData, owner: NeutronAddress, id: u64, address: NeutronAddress) -> Result<u64, NeutronError>{
        let key = build_token_key(address, id);
        match self.private_load_state_external(codata, owner, &key){
            Ok(v) => {
                let mut balance = u64::from_le_bytes(v.clone().try_into().unwrap());
                balance -= codata.compute_outgoing_transfer_value(owner, id, address).unwrap_or(0);
                Ok(balance)
            },
            Err(_) => {
                Ok(0)
            }
        }
    }
    
    fn create_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
    fn revert_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
    fn commit_checkpoint(&mut self, codata: &mut CoData) -> Result<(), NeutronError>;
}

pub fn build_token_key(token_owner: NeutronAddress, id: u64) -> Vec<u8>{
    use crate::AddressDecoding;
    let mut key = Vec::with_capacity(1 + 20 + 1 + 8);
    key.push(0);
    key.push(0xFF);
    key.extend(token_owner.decode());
    key.push(95); //ASCII for _
    key.extend(&id.to_le_bytes());
    key
}