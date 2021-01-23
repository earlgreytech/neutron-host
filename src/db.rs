extern crate neutron_star_constants;
use std::collections::HashMap;
//use std::collections::HashSet;
use neutron_star_constants::*;
use crate::neutronerror::*;
use crate::element_interfaces::storage::*;
use crate::codata::*;

const USER_SPACE: u8 = '_' as u8;

/*
pub trait NeutronDB{
    fn read_key(&mut self, address: &NeutronAddress, key: &[u8]) -> Result<Vec<u8>, NeutronError>;
    fn write_key(&mut self, address: &NeutronAddress, key: &[u8], value: &[u8]) -> Result<(), NeutronError>;
    /// Creates a new checkpoint which enables the ability to revert back to the current state
    /// Returns the number of current checkpoints within the database context
    fn checkpoint(&mut self) -> Result<u32, NeutronError>;
    /// Collapses all outstanding checkpoints into a single top level checkpoint
    fn collapse_checkpoints(&mut self) -> Result<(), NeutronError>;
    /// Reverts the current state to the previous checkpoint, discarding the modifications made since that checkpoint
    fn revert_checkpoint(&mut self) -> Result<u32, NeutronError>;
    fn clear_checkpoints(&mut self);
    /// Commits all state to the database 
    /// TBD: should this be left as a non-trait function??
    fn commit(&mut self) -> Result<(), NeutronError>;
    //fn compute_new_proofs(&mut self, )
    // Automatically will execute `collapse_checkpoints`. Returns the keys and values which were read in this context as well as the keys which were written to
    //fn compute_state_differences(&mut self, reads: HashMap<NeutronAddress, HashMap<Vec<u8>, Vec<u8>>>, writes: HashMap<NeutronAddress, HashMap<Vec<u8>, Vec<u8>>>)
    //    -> Result<(), NeutronDBError>;
}
*/
#[derive(Default,  Debug, Clone)]
pub struct MemoryGlobalState{
    storage: HashMap<NeutronAddress, HashMap<Vec<u8>, Vec<u8>>>,
    /// This only tracks keys which are read from storage, and ignores checkpoint-only data and reverts
    //touched: HashMap<NeutronAddress, Vec<u8>>,
    //rents: HashMap<Vec<u8>, u32>,
    checkpoints: Vec<HashMap<NeutronAddress, HashMap<Vec<u8>, Vec<u8>>>>
}

impl MemoryGlobalState{
    /*
    pub fn create_user_key(&self, address: NeutronAddress, space: u8, key: &[u8]) -> Vec<u8>{
        let bigkey = Vec::<u8>::default();
        for i in 0..4{
            bigkey.push(((address.version & (0xFF << i)) >> (i * 8)) as u8);
        }
        
        bigkey.append(&mut address.data.to_vec());
        bigkey.push(space);
        bigkey.append(&mut key.to_vec());
        bigkey
    }
    pub fn create_raw_key(&self, address: NeutronAddress, key: &[u8]) -> Vec<u8>{
        let bigkey = Vec::<u8>::default();
        for i in 0..4{
            bigkey.push(((address.version & (0xFF << i)) >> (i * 8)) as u8);
        }
        
        bigkey.append(&mut address.data.to_vec());
        bigkey.append(&mut key.to_vec());
        bigkey
    }
    */
    pub fn create_user_key(&self, key: &[u8]) -> Vec<u8>{
        let mut bigkey = Vec::<u8>::default();
        bigkey.push(USER_SPACE);
        bigkey.append(&mut key.to_vec());
        bigkey
    }
}

impl GlobalState for MemoryGlobalState{
    fn store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{
        self.write_key(&codata.peek_context(0).unwrap().self_address, &self.create_user_key(key), value)
    }
    fn load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        self.read_key(&codata.peek_context(0).unwrap().self_address, &self.create_user_key(key))
    }
    fn key_exists(&mut self, _codata: &mut CoData, _key: &[u8]) -> Result<bool, NeutronError>{
        Err(NeutronError::Unrecoverable(UnrecoverableError::NotImplemented))
    }

    fn private_store_state(&mut self, codata: &mut CoData, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{
        self.write_key(&codata.peek_context(0).unwrap().self_address,  key, value)
    }
    fn private_load_state(&mut self, codata: &mut CoData, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        self.read_key(&codata.peek_context(0).unwrap().self_address, key)
    }
    fn private_store_state_external(&mut self, codata: &mut CoData, _address: NeutronAddress, key: &[u8], value: &[u8]) -> Result<(), NeutronError> {
        self.write_key(&codata.peek_context(0).unwrap().self_address, &key, value)
    }
    fn private_load_state_external(&mut self, codata: &mut CoData, _address: NeutronAddress, key: &[u8]) -> Result<Vec<u8>, NeutronError> {
        self.read_key(&codata.peek_context(0).unwrap().self_address, &key)
    }

    fn create_checkpoint(&mut self, _codata: &mut CoData) -> Result<(), NeutronError>{
        self.checkpoint()?;
        Ok(())
    }
    fn revert_checkpoint(&mut self, _codata: &mut CoData) -> Result<(), NeutronError>{
        self.revert_single_checkpoint()?;
        Ok(())
    }
    fn commit_checkpoint(&mut self, _codata: &mut CoData) -> Result<(), NeutronError>{
        self.commit_single_checkpoint()
    }
}

impl MemoryGlobalState{
    pub fn read_key(&mut self, address: &NeutronAddress, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        for checkpoint in self.checkpoints.iter().rev(){
            match checkpoint.get(address){
                Some(kv) => {
                    match kv.get(key){
                        Some(v) => {
                            return Ok(v.to_vec());
                        },
                        None => {}
                    }
                },
                None => {
                }
            }
        }
        match self.storage.get(address){
            Some(kv) => {
                match kv.get(key){
                    Some(v) => {
                        return Ok(v.to_vec());
                    },
                    None => {
                    }
                }
            },
            None => {
            }
        }
        Err(NeutronError::Unrecoverable(UnrecoverableError::StateOutOfRent))
    }
    pub fn write_key(&mut self, address: &NeutronAddress, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{

        if self.checkpoints.len() == 0{
            return Err(NeutronError::Unrecoverable(UnrecoverableError::DeveloperError));
        }
        let c = self.checkpoints.last_mut().unwrap();
        match c.get_mut(address){
            Some(kv) => {
                kv.insert(key.to_vec(), value.to_vec());
            },
            None => {
                let mut t = HashMap::new();
                t.insert(key.to_vec(), value.to_vec());
                c.insert(*address, t);
            }
        }
        Ok(())
    }
    pub fn checkpoint(&mut self) -> Result<u32, NeutronError>{
        self.checkpoints.push(HashMap::new());
        Ok(self.checkpoints.len() as u32)
    }
    pub fn revert_single_checkpoint(&mut self) -> Result<u32, NeutronError>{
        if self.checkpoints.pop().is_none(){
            Err(NeutronError::Unrecoverable(UnrecoverableError::StateOutOfRent))
        }else{
            Ok(self.checkpoints.len() as u32)
        }
    }
    pub fn commit_single_checkpoint(&mut self) -> Result<(), NeutronError>{
        let mut collapsed = HashMap::new();
        let mut kv_top = self.checkpoints.pop().unwrap();
        let mut kv_bottom = self.checkpoints.pop().unwrap();
        for (key, value) in kv_bottom.drain(){
            collapsed.insert(key, value);
        }
        for (key, value) in kv_top.drain(){
            collapsed.insert(key, value);
        }
        self.checkpoints.push(collapsed);
        
        Ok(())
    }
    pub fn collapse_checkpoints(&mut self) -> Result<(), NeutronError>{
        let mut collapsed = HashMap::new();
        for kv in self.checkpoints.iter_mut(){
            for (key, value) in kv.drain(){
                collapsed.insert(key, value);
            }
        }
        self.checkpoints.clear();
        self.checkpoints.push(collapsed);
        
        Ok(())
    }
    pub fn commit(&mut self) -> Result<(), NeutronError>{
        self.collapse_checkpoints()?;
        for (key, value) in self.checkpoints.last_mut().unwrap().drain(){
            match self.storage.get_mut(&key){
                None => {
                    self.storage.insert(key, value);
                },
                Some(kv) => {
                    for(k2, v2) in value{
                        kv.insert(k2, v2);
                    }
                }
            }
        }
        self.clear_checkpoints();
        Ok(())
    }
    pub fn clear_checkpoints(&mut self){
        self.checkpoints.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic(){
        let mut a = NeutronAddress::default();
        a.version=100;
        a.data[5] = 20;
        let mut db = MemoryGlobalState::default();
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[1], &[8, 8, 8, 8]).is_ok());
        assert!(db.write_key(&a, &[1], &[9, 9, 9, 9]).is_ok());
        let v = db.read_key(&a, &[1]).unwrap();
        assert!(v == vec![9, 9, 9, 9]);
    }
    
    #[test]
    fn test_checkpoints(){
        let mut a = NeutronAddress::default();
        a.version=100;
        a.data[5] = 20;
        let mut db = MemoryGlobalState::default();
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[1], &[8, 8, 8, 8]).is_ok());
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[1], &[9, 9, 9, 9]).is_ok());
        assert!(db.revert_single_checkpoint().is_ok());
        let v = db.read_key(&a, &[1]).unwrap();
        assert!(v == vec![8, 8, 8, 8]);
    }
    
    #[test]
    fn test_storage(){
        let mut a = NeutronAddress::default();
        a.version=100;
        a.data[5] = 20;
        let mut db = MemoryGlobalState::default();
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[1], &[8, 8, 8, 8]).is_ok());
        assert!(db.commit().is_ok());
        assert!(db.revert_single_checkpoint().is_err());
        db.clear_checkpoints();
        let v = db.read_key(&a, &[1]).unwrap();
        assert!(v == vec![8, 8, 8, 8]);
        db.clear_checkpoints();
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[1, 2, 3], &[9, 9, 9, 9]).is_ok());
        assert!(db.commit().is_ok());
        assert!(db.revert_single_checkpoint().is_err());
        assert!(db.checkpoint().is_ok());
        let v = db.read_key(&a, &[1, 2, 3]).unwrap();
        assert!(v == vec![9, 9, 9, 9]);
    }
    #[test]
    fn replicate_checkpoint_bug(){
        let mut a = NeutronAddress::default();
        a.version=100;
        a.data[5] = 20;
        let mut db = MemoryGlobalState::default(); 
        //deploy
        assert!(db.checkpoint().is_ok());
        assert!(db.write_key(&a, &[2, 1, 0], &[10]).is_ok());
        assert!(db.commit().is_ok());
        //first call
        assert!(db.checkpoint().is_ok());
        let v = db.read_key(&a, &[2, 1, 0]).unwrap();
        assert!(v == vec![10]);
        db.write_key(&a, &[95, 0, 1, 2, 3], &[10, 20, 30, 40]).unwrap();
        db.commit().unwrap();
        //second call
        db.checkpoint().unwrap();
        let v = db.read_key(&a, &[2, 1, 0]).unwrap();
        assert!(v == vec![10]);
        
    }
    
}