use crate::interface::*;
use crate::addressing::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
use std::collections::HashMap;
use std::convert::*;
use std::mem;

#[derive(Default)]
pub struct GasSchedule{
    //vm_operation -> gas cost
    pub vm_costs: HashMap<u32, u64>,
    //element -> function -> [gas_cost_parameters]
    pub element_costs: HashMap<u32, HashMap<u32, Vec<u64>>>
}

pub struct CoData{
    context_stack: Vec<ExecutionContext>,
    stacks: [Vec<Vec<u8>>; 2],
    maps: Vec<HashMap<Vec<u8>, Vec<u8>>>,
    input_stack_index: usize,
    output_stack_index: usize,
    top_input_map_index: usize,
    top_output_map_index: usize,
    top_result_map_index: usize,
    
    //various fields that aren't really CoData, but are most convenient to track here
    pub gas_remaining: u64,
    pub vm_writeable_memory: u32,
    pub vm_read_only_memory: u32,
    pub gas_schedule: GasSchedule,

    /// Used for certain internal operations, such as loading bytecode, 
    /// where a "pure" call should be allowed to ignore otherwise restrictive permissions for special and determined-safe purposes
    pub ignore_permissions: bool
}

impl Default for CoData{
    fn default() -> CoData{
        let mut c = CoData{
            gas_remaining: 0,
            vm_writeable_memory: 0,
            vm_read_only_memory: 0,
            gas_schedule: GasSchedule::default(),
            ignore_permissions: false,
            context_stack: vec![],
            stacks: [vec![], vec![]],
            top_input_map_index: 1,
            top_output_map_index: 0,
            top_result_map_index: 1,
            input_stack_index: 0,
            output_stack_index: 1,
            maps: vec![]
        };
        c.maps.push(HashMap::<Vec<u8>, Vec<u8>>::default()); //add output map (note: this is flipped when context is pushed)
        c.maps.push(HashMap::<Vec<u8>, Vec<u8>>::default()); //add input map
        c
    }
}

impl CoData{
    pub fn new() -> CoData{
        CoData::default()
    }
    pub fn permissions(&self) -> ContextPermissions{
        if self.ignore_permissions{
            ContextPermissions::mutable_call()
        }else{
            self.current_context().permissions
        }
    }
    pub fn push_output_stack(&mut self, data: &[u8]) -> Result<(), NeutronError>{
        self.stacks[self.output_stack_index].push(data.to_vec());
        Ok(())
    }
	pub fn pop_input_stack(&mut self) -> Result<Vec<u8>, NeutronError>{
        match self.stacks[self.input_stack_index].pop(){
            None => {
                return Err(Recoverable(RecoverableError::ItemDoesntExist));
            },
            Some(v) => {
                return Ok(v);
            }
        }
    }
    pub fn clear_input_stack(&mut self){
        self.stacks[self.input_stack_index].clear();
    }
	pub fn drop_input_stack(&mut self) -> Result<(), NeutronError>{
        match self.stacks[self.input_stack_index].pop(){
            None => {
                return Err(Recoverable(RecoverableError::ItemDoesntExist));
            },
            Some(_) => {
                return Ok(());
            }
        }
    }
	pub fn peek_input_stack(&self, index: u32) -> Result<Vec<u8>, NeutronError>{
        let stack = &self.stacks[self.input_stack_index];
        let i = (stack.len() as isize - 1) - index as isize;
        if i < 0{
            return Err(Recoverable(RecoverableError::ItemDoesntExist));
        }
        match stack.get(i as usize){
            None => {
                return Err(Recoverable(RecoverableError::ItemDoesntExist));
            },
            Some(v) => {
                return Ok(v.to_vec());
            }
        }
    }

    pub fn push_output_key(&mut self, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{
        if key[0] == 0{
            return Err(NeutronError::Recoverable(RecoverableError::InvalidCoMapAccess));
        }
        self.maps.get_mut(self.top_output_map_index).unwrap().insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    pub fn push_input_key(&mut self, key: &[u8], value: &[u8]) -> Result<(), NeutronError>{
        if key[0] == 0{
            return Err(NeutronError::Recoverable(RecoverableError::InvalidCoMapAccess));
        }
        self.maps.get_mut(self.top_input_map_index).unwrap().insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    /* should this be allowed?
    pub fn pop_key(&mut self, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        match self.maps[self.top_input_map].remove(key){
            Some(v) => {
                Ok(v)
            },
            None => {
                Err(Recoverable(RecoverableError::ItemDoesntExist))
            }
        }
    }
    */
    pub fn peek_input_key(&self, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        if key[0] == 0{
            return Err(NeutronError::Recoverable(RecoverableError::InvalidCoMapAccess));
        }
        match self.maps[self.top_input_map_index].get(key){
            Some(v) => {
                Ok(v.to_vec())
            },
            None => {
                Err(Recoverable(RecoverableError::ItemDoesntExist))
            }
        }
    }
    pub fn peek_result_key(&self, key: &[u8]) -> Result<Vec<u8>, NeutronError>{
        if key[0] == 0{
            return Err(NeutronError::Recoverable(RecoverableError::InvalidCoMapAccess));
        }
        match self.maps[self.top_result_map_index].get(key){
            Some(v) => {
                Ok(v.to_vec())
            },
            None => {
                Err(Recoverable(RecoverableError::ItemDoesntExist))
            }
        }
    }
    fn build_transfer_key(&self, token_owner: NeutronAddress, id: u64) -> Vec<u8>{
        use crate::AddressDecoding;
        let mut key = Vec::with_capacity(1 + 20 + 1 + 8);
        key.push(0);
        key.extend(token_owner.decode());
        key.push(95); //ASCII for _
        key.extend(&id.to_le_bytes());
        key
    }

    pub fn push_output_transfer(&mut self, token_owner: NeutronAddress, id: u64, value: u64){
        let c = self.context_stack.last().unwrap();
        let key = self.build_transfer_key(token_owner, id);
        self.maps.get_mut(c.output_map).unwrap().insert(key.to_vec(), value.to_le_bytes().to_vec());
    }

    pub fn peek_input_transfer(&self, token_owner: NeutronAddress, id: u64) -> Result<u64, NeutronError>{
        let c = self.context_stack.last().unwrap();
        let key = self.build_transfer_key(token_owner, id);
        match self.maps[c.input_map].get(&key){
            Some(v) => {
                Ok(u64::from_le_bytes(v.clone().try_into().unwrap()))
            },
            None => {
                Err(Recoverable(RecoverableError::ItemDoesntExist))
            }
        }
    }

    pub fn element_pop_transfer(&mut self, token_owner: NeutronAddress, id: u64) -> Result<u64, NeutronError>{
        let c = self.context_stack.last().unwrap();
        let key = self.build_transfer_key(token_owner, id);
        match self.maps[c.input_map].remove(&key){
            Some(v) => {
                Ok(u64::from_le_bytes(v.try_into().unwrap()))
            },
            None => {
                Err(Recoverable(RecoverableError::ItemDoesntExist))
            }
        }
    }
    pub fn compute_outgoing_transfer_value(&self, token_owner: NeutronAddress, id: u64, _address: NeutronAddress) -> Result<u64, NeutronError>{
        let address = self.context_stack.last().unwrap().self_address;
        let key = self.build_transfer_key(token_owner, id);
        let mut value = 0;
        for context in &self.context_stack{
            if context.self_address == address{
                match self.maps[context.output_map].get(&key){
                    Some(v) => {
                        value += u64::from_le_bytes(v.clone().try_into().unwrap());
                    },
                    None => {}
                }
            }
        }
        Ok(value)
    }

    /// Should only be used by hypervisor ops that utilize costack arguments. Flip stacks once when entering the op and once more before leaving. 
    /// Used so that ops can actually read its inputs, and so the calling contract can read its outputs. 
    pub fn flip_stacks(&mut self){
        let tmp = self.input_stack_index;
        self.input_stack_index = self.output_stack_index;
        self.output_stack_index = tmp;
    }

    /// Should only be used by Element APIs. Flip stacks once when entering an Element API and once more when leaving and returning to a contract.
    /// Used so that contract outputs become Element inputs at first, then so that Element outputs becomes contract inputs
    /// This function clears the old input/new output stack on each flipping, which means an Element API will always leave the stacks empty save for its outputs (if any)
    fn flip_stacks_clear_output(&mut self){
        self.flip_stacks();
        self.stacks[self.output_stack_index].clear(); //outputs are cleared with each flipping (clears caller's outputs on entry, then callers inputs upon exit)
    }

    /// Used only by namesake hypervisor op to efficiently overwrite the output stack with a copy of the input stack
    /// This operation is meant to streamline the process of passing the current context's input/result as input to a new call
    /// TODO: Evaluate if a destructive move is the best approach?
    /// TODO: Allow moving only parts of the stack?
    pub fn move_input_to_output_costack(&mut self){
        self.stacks[self.output_stack_index] = mem::replace(&mut self.stacks[self.input_stack_index], vec![]);
    }

    /*
    Map Management
    new state, 1: 3 maps added: inputA1, outputA2, resultA3
    Call initiated, 2: 1 map added, resultB4. input2 points to outputA2, output2 points to resultA3
    Call initiated, 3: 1 map added, resultC5. input3 points to ResultA3, output3 points to ResultB4 
    Call initiated, 4: 1 map added, ResultD5. input4 points to ResultB3, output4 points to ResultC5
    Call ends, 4 destroyed: ResultD destroyed, top input(3) equivalent to (output2)ResultA. top output(3) equivalent to (input4)ResultB, is cleared. top result(3) equivalent to (output4)ResultC
    End result per call: input = len-1, output = len, result = len+1
    */

    /// Pushes a new execution context into the stack
    pub fn push_context(&mut self, context: ExecutionContext) -> Result<(), NeutronError>{
        let mut c = context;
        self.top_input_map_index = self.top_output_map_index; //one below top of stack
        self.top_output_map_index = self.top_result_map_index; //top of stack
        self.top_result_map_index = self.top_result_map_index + 1; //new (temporary) map

        c.input_map = self.top_input_map_index;
        c.output_map = self.top_output_map_index;
        c.result_map = self.top_result_map_index;
        self.maps.get_mut(self.top_output_map_index).unwrap().clear(); //clear what is now the new result map (which can go on to become the next call's output map)
        self.maps.push(HashMap::<Vec<u8>, Vec<u8>>::new()); //push new result map
        self.context_stack.push(c);
        //begin execution???
        Ok(())
    }

    /// Note for Element functions which result in a context push (ie, a call), 
    /// enter and exit functions should both be used before pushing the context and starting the call (and beginning execution),
    /// and then both again used after popping the context and exiting the call (returning execution to caller)
    pub fn enter_element(&mut self){
        self.top_input_map_index = self.top_output_map_index; //one below top of stack
        self.top_input_map_index = self.top_output_map_index; //one below top of stack
        self.top_output_map_index = self.top_result_map_index; //top of stack
        self.flip_stacks_clear_output();
        //note: elements can not access result map 
    }
    pub fn exit_element(&mut self){
        let c = self.context_stack.last().unwrap();
        self.top_output_map_index = c.output_map;
        self.top_input_map_index = c.input_map;
        self.flip_stacks_clear_output();
    }
    /// Removes the top execution context from the stack
    pub fn pop_context(&mut self) -> Result<(), NeutronError>{
        match self.context_stack.pop(){
            None => {
                return Err(Unrecoverable(UnrecoverableError::ContextIndexEmpty));
            },
            Some(_) => {}
        }
        let c = match self.context_stack.last(){
            None => {
                //no more contexts, so set to transaction level behavior
                self.top_input_map_index = 1;
                self.top_output_map_index = 0;
                self.top_result_map_index = 1;
                return Ok(());
            },
            Some(v) => {v}
        };
        self.maps.pop().unwrap(); //result map of caller is destroyed
        self.top_input_map_index = c.input_map;
        self.top_output_map_index = c.output_map;
        self.top_result_map_index = c.result_map;
        Ok(())
    }
    /// Peeks information from the execution context stack without modifying it
    pub fn peek_context(&self, index: usize) -> Result<&ExecutionContext, NeutronError>{
        let i = (self.context_stack.len() as isize - 1) - index as isize;
        if i < 0{
            return Err(Recoverable(RecoverableError::ItemDoesntExist));
        }
        match self.context_stack.get(i as usize){
            None => {
                return Err(Recoverable(RecoverableError::ItemDoesntExist));
            },
            Some(v) => {
                return Ok(v);
            }
        }
    }
    /// The total number of smart contract contexts currently involved in the overall execution
    pub fn context_count(&self) -> usize{
        self.context_stack.len()
    }

	/// Retrieves the context information of the current smart contract execution
	pub fn current_context(&self) -> &ExecutionContext{
        //this should never error, so just unwrap
        self.peek_context(0).unwrap()
    }

    /// Creates a top level context for calling an existing contract. The context stack MUST be empty
    pub fn create_top_level_call(&mut self, address: NeutronAddress, sender: NeutronAddress, gas_limit: u64, value: u64){
        assert!(self.context_stack.len() == 0);
        let mut c = ExecutionContext::default();
        c.self_address = address.clone();
        c.gas_limit = gas_limit;
        c.value_sent = value;
        c.sender = sender.clone();
        c.origin = sender.clone();
        c.execution_type = ExecutionType::Call;
        self.push_context(c).unwrap();
    }
    /// Creates a top level context for deploying a new contract. The context stack MUST be empty
    pub fn create_top_level_deploy(&mut self, address: NeutronAddress, sender: NeutronAddress, gas_limit: u64, value: u64){
        assert!(self.context_stack.len() == 0);
        //todo: dedupicate
        let mut c = ExecutionContext::default();
        c.self_address = address.clone();
        c.gas_limit = gas_limit;
        c.value_sent = value;
        c.sender = sender.clone();
        c.origin = sender.clone();
        c.execution_type = ExecutionType::Deploy;
        self.push_context(c).unwrap();
    }
    /// Creates a new nested context for calling an existing contract. The context stack MUST NOT be empty
    pub fn create_call(&mut self, address: NeutronAddress, gas_limit: u64, value: u64){
        assert!(self.context_stack.len() > 0);
        let mut c = ExecutionContext::default();
        c.self_address = address.clone();
        c.gas_limit = gas_limit;
        c.value_sent = value;
        c.sender = self.peek_context(0).unwrap().self_address.clone();
        c.origin = self.context_stack.get(0).unwrap().sender.clone();
        c.execution_type = ExecutionType::Call;
        self.push_context(c).unwrap();
    }
    /// Creates a new nested context for deploying a contract. The context stack MUST NOT be empty
    pub fn create_deploy(&mut self, address: NeutronAddress, gas_limit: u64, value: u64){
        assert!(self.context_stack.len() > 0);
        let mut c = ExecutionContext::default();
        c.self_address = address.clone();
        c.gas_limit = gas_limit;
        c.value_sent = value;
        c.sender = self.peek_context(0).unwrap().self_address.clone();
        c.origin = self.context_stack.get(0).unwrap().sender.clone();
        c.execution_type = ExecutionType::Deploy;
        self.push_context(c).unwrap();
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_call_map_flow(){
        let mut manager = CoData::new();
        let c1 = ExecutionContext::default();
        let c2 = ExecutionContext::default();
        let c3 = ExecutionContext::default();
        let key = [1];
        //ABI data
        manager.push_output_key(&key, &[1]).unwrap();
        manager.push_output_key(&[2], &[2]).unwrap();
        //call from transaction
        {
            manager.push_context(c1).unwrap();
            manager.push_output_key(&key, &[2]).unwrap();
            assert_eq!(manager.peek_input_key(&key).unwrap()[0], 1);
            //call into sub contract
            {
                manager.enter_element(); // as if the contract had called an element to make a call
                manager.exit_element(); // element must be exited before execution can be transferred
                manager.push_context(c2).unwrap();
                manager.push_output_key(&key, &[3]).unwrap();
                assert_eq!(manager.peek_input_key(&key).unwrap()[0], 2);
                //call into sub sub contract
                {
                    manager.enter_element();
                    manager.exit_element();
                    manager.push_context(c3).unwrap();
                    manager.push_output_key(&key, &[4]).unwrap();
                    assert_eq!(manager.peek_input_key(&key).unwrap()[0], 3);
                    manager.pop_context().unwrap();
                    manager.enter_element();
                    manager.exit_element();
                }
                assert_eq!(manager.peek_result_key(&key).unwrap()[0], 4);
                assert_eq!(manager.peek_input_key(&key).unwrap()[0], 2);
                manager.pop_context().unwrap();
                manager.enter_element(); // return to element context (ie to check results and push relevant data)
                manager.exit_element(); // finally, exit element to return execution to contract
            }
            assert_eq!(manager.peek_result_key(&key).unwrap()[0], 3);
            assert_eq!(manager.peek_input_key(&key).unwrap()[0], 1);
            manager.pop_context().unwrap();
        }
        assert!(manager.peek_input_key(&[2]).is_err());
        assert_eq!(manager.peek_input_key(&key).unwrap()[0], 2);
        assert_eq!(manager.peek_result_key(&key).unwrap()[0], 2);
    }
    #[test]
    fn test_element_stack_flow(){
        let mut manager = CoData::new();
        let c1 = ExecutionContext::default();        
        let key = [1];
        //ABI data
        manager.push_output_key(&key, &[1]).unwrap();
        //element data
        manager.push_context(c1).unwrap();
        manager.push_output_stack(&[2]).unwrap();
        manager.push_output_key(&key, &[5]).unwrap();
        {
            manager.enter_element();
            manager.push_output_stack(&[3]).unwrap();
            manager.push_output_key(&key, &[4]).unwrap();
            assert_eq!(manager.peek_input_key(&key).unwrap()[0], 5);
            assert_eq!(manager.pop_input_stack().unwrap()[0], 2);
            manager.exit_element();
        }
        assert_eq!(manager.peek_result_key(&key).unwrap()[0], 4);
        assert_eq!(manager.pop_input_stack().unwrap()[0], 3);
        manager.pop_context().unwrap();
        assert_eq!(manager.peek_result_key(&key).unwrap()[0], 5); //note: unsure if this behavior is correct
    }
}