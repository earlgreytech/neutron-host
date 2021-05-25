use crate::addressing::*;
use crate::neutronerror::*;

/// The result of a smart contract execution
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct NeutronVMResult{
	/// The total amount of gas used by the execution
	pub gas_used: u64,
	/// If set to true, then no state effects should've occured from this execution and any state effects should be reverted
	pub should_revert: bool,
	/// The error code specifying how this contract ended
	pub error_code: u32,
	/// An undefined ID of the location of the contract error (for x86 this is the 'EIP' register)
	pub error_location: u64,
	/// Extra data which a smart contract VM is free to use. This is not exposed to smart contracts
    pub extra_data: u64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum ExecutionType{
    Call = 0,
    Deploy = 1,
    BareExecution = 2
}

impl Default for ExecutionType{
    fn default() -> ExecutionType{
        ExecutionType::Call
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Copy)]
pub struct ContextPermissions{
    /// Program can modify its own state and status
    pub modify_self: bool,
    /// program can modify (or trigger modification of) external state and status
    pub modify_external: bool,
    /// Program can access external state
    pub access_external: bool,
    /// Program can access its own data and storage (NOTE bytecode is excluded from this; only valid to be false when contract code can not be upgraded or modified)
    pub access_self: bool,
}
/*
Implications of permission sets:

Pure -- can always be executed in parallel safely and also called from any other context (including isolated, since "pure" is regarded as stateless)
Isolated -- Can be executed in parallel with other isolated calls which are not using the same address
Immutable -- Can be executed in parallel with other immutable and pure calls in between "barriers" of mutable/isolated calls 
Mutable -- Can never be executed in parallel

Pure can only make pure calls
Isolated can make all calls which do not touch external state or other addresses, and are capable of making pure calls (including to other external addresses)
Immutable can only make other immutable and pure calls
Mutable can make all calls and all types of calls can be utilized within a mutable context
*/
impl ContextPermissions{
    pub fn pure_call() -> ContextPermissions{
        ContextPermissions{
            modify_self: false,
            modify_external: false,
            access_external: false,
            access_self: false,
        }
    }
    pub fn isolated_call() -> ContextPermissions{
        ContextPermissions{
            modify_external: false,
            modify_self: true,
            access_self: true,
            access_external: false
        }
    }
    pub fn immutable_call() -> ContextPermissions{
        ContextPermissions{
            modify_external: false,
            modify_self: false,
            access_self: true,
            access_external: true
        }
    }
    pub fn mutable_call() -> ContextPermissions{
        ContextPermissions{
            modify_external: true,
            modify_self: true,
            access_self: true,
            access_external: true
        }
    }
    pub fn assert_has_self_access(&self) -> Result<(), NeutronError>{
        if !self.access_self{
            Err(NeutronError::Recoverable(RecoverableError::RequiresPermissionSelfAccess))
        }else{
            Ok(())
        }
    }
    pub fn assert_has_self_modification(&self) -> Result<(), NeutronError>{
        if !self.modify_self{
            Err(NeutronError::Recoverable(RecoverableError::RequiresPermissionSelfMod))
        }else{
            Ok(())
        }
    }
    pub fn assert_has_external_access(&self) -> Result<(), NeutronError>{
        if !self.access_external{
            Err(NeutronError::Recoverable(RecoverableError::RequiresPermissionExternalAccess))
        }else{
            Ok(())
        }
    }
    pub fn assert_has_external_modification(&self) -> Result<(), NeutronError>{
        if !self.modify_external{
            Err(NeutronError::Recoverable(RecoverableError::RequiresPermissionExternalMod))
        }else{
            Ok(())
        }
    }
}


/// The execution context of the current smart contract
/// Multiple ExecContext structs are expected, a new one for each smart contract call performed. 
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ExecutionContext{
	/// TBD
	pub flags: u64,
    /// The address which caused this execution to occur.
    /// This may be the sender of the transaction, or the smart contract which caused this execution to occur via a call.
	pub sender: NeutronAddress,
    /// The total amount of gas allowed to be consumed in this execution
	pub gas_limit: u64,
	/// The number of coins which were sent with this execution
	pub value_sent: u64,
	/// The address which caused this chain of execution to occur.
    /// This is the sender of the transaction which caused this execution.
	pub origin: NeutronAddress,
	/// The current address of the executing smart contract
    pub self_address: NeutronAddress,
    pub execution_type: ExecutionType,
    pub input_stack: usize,
    pub output_stack: usize,
    pub input_map: usize,
    pub output_map: usize,
    pub result_map: usize,
    pub permissions: ContextPermissions
}

impl ExecutionContext{
    pub fn create_default_random_context() -> ExecutionContext{
        let mut c = ExecutionContext::default();
        crate::reset_to_random_address(&mut c.self_address);
        c.self_address.version = 2; //to match NARM VM number
        c
    }
}



/// The transaction information in which the current contract execution is located
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionContext{
	/// The spent UTXOs which make up this transaction
	pub inputs: Vec<TxItem>,
	/// The created UTXOs, contract executions, and other misc data which make up this transaction
	pub outputs: Vec<TxItem>,
    /// The total amount of coins spent by gas fees
    /// Note that this only counts for gas_limit, as it can not be known how much actual gas will be consumed until the transaction is complete
    pub total_gas_fees: u64,
    /// The total fee in coins sent with the transaction. This includes the above total_gas_fees and also any other transaction fees. 
    pub total_fees: u64
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TxItem{
	/// The owner of this UTXO (or spent UTXO)
	pub sender: NeutronAddress,
	/// The total value sent with this UTXO (or spent by it)
    pub value: u64,
    /// The state sent with this UTXO
    pub state: Vec<u8>
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BlockContext{
	/// The creator of the current block
	pub creator: NeutronAddress,
	/// The total gas limit for the entire block
	pub gas_limit: u64,
	/// The difficulty of the current block (the meaning of this varies by blockchain)
	pub difficulty: u64,
	/// The block height of the current block
	pub height: u32,
    /// The time recorded in the block just before this one (the current time can not be revealed by all blockchains due to determinism problems)
	pub previous_time: u64,
	/// The previous block hashes leading up to this block.
    /// previous_hashes[0] is the previous block, previous_hashes[1] is the block before that, and so on
    /// Not all blockchains will reveal an entire list of block hashes in this field.
	pub previous_hashes: Vec<[u8; 32]>
}



/*
typedef struct{
    uint8_t format;
    uint8_t rootVM;
    uint8_t vmVersion;
    uint16_t flagOptions;
    uint32_t qtumVersion;
} NeutronVersion;
*/
#[derive(Debug, Eq, PartialEq, Default)]
pub struct NeutronVersion{
    pub format: u8,
    pub root_vm: u8,
    pub vm_version: u8,
    pub flags: u16,
}




    /*
    leftovers from NeutronAPI that need to be implemented in system contracts
	/// Loads user accessible state from the smart contract database
    fn load_state(&mut self, address: NeutronAddress, key: &[u8], data: &mut Vec<u8>) -> Result<usize, NeutronError>;
    /// Writes user accessible state to the smart contract database
    fn store_state(&mut self, address: NeutronAddress, key: &[u8], data: &[u8]) -> Result<(), NeutronError>;
    /// Loads "protected" state from the smart contract database. Protected state can include bytecode, VM configuration options, etc. 
    /// Protected state should not be freely exposed to smart contracts 
    fn load_protected_state(&mut self, address: NeutronAddress, key: &[u8], data: &mut Vec<u8>) -> Result<usize, NeutronError>;
    /// Writes "protected" state to the smart contract database. Protected state can include bytecode, VM configuration options, etc. 
    /// Protected state should not be freely exposed to smart contracts 
    fn store_protected_state(&mut self, address: NeutronAddress, key: &[u8], data: &[u8]) -> Result<(), NeutronError>;
    /// Loads user accessible state from another smart contract's "namespace" in the smart contract database.  
    fn load_external_state(&mut self, address: &NeutronAddress, key: &[u8], data: &mut Vec<u8>) -> Result<usize, NeutronError>;
    /// Loads "protected" state from the smart contract database which is from another smart contract's namespace. 
    /// Protected state can include bytecode, VM configuration options, etc. Protected state should not be freely exposed to smart contracts 
    fn load_external_protected_state(&mut self, address: &NeutronAddress, key: &[u8], data: &mut Vec<u8>) -> Result<usize, NeutronError>;

    /// Transfers coins from the currently executing smart contract to the specified address
    fn transfer(&mut self, address: &NeutronAddress, value: u64) -> Result<(), NeutronError>;
    /// Transfers coins from the currently executing smart contract to the specified address
    /// This can only be used for valid short addresses where the amount of data in a full address exactly matches the size of a short address
    fn transfer_short(&mut self, address: &NeutronAddress, value: u64) -> Result<(), NeutronError>;
    /// Returns the balance of the currently executing smart contract
    fn balance(&mut self) -> Result<u64, NeutronError>;
    /// Checks the balance of an external smart contract. This can not be used for checking the balance of non-contract addresses.
    fn balance_of_external(&mut self, address: &NeutronAddress) -> Result<u64, NeutronError>;

    /// Gets the block hash of the specified block
    fn get_block_hash(&mut self, number: u64, hash: &mut[u8]) -> Result<(), NeutronError>;

    /// Calculates the difference in gas cost produced by changing the amount of allocated memory.
    /// Note this does not actually allocate any memory, this is left to the specific VM and hypervisor.
    /// This is only for charging an appropriate gas cost to the smart contract for allocating/freeing memory.
    fn calculate_memory_cost(&self, existing_size: u64, new_size: u64) -> Result<i64, NeutronError>;
    /// Calculates the difference in gas cost produced by changing the amount of allocated read-only memory.
    /// Note this does not actually allocate any memory nor charge the smart contract for the gas, this is left to the specific VM and hypervisor.
    /// This is only for charging an appropriate gas cost to the smart contract for allocating/freeing memory.
    fn calculate_readonly_memory_cost(&self, existing_size: u64, new_size: u64) -> Result<i64, NeutronError>;

    /// This is used for charging (or refunding) the smart contract for a specific gas cost, such as memory allocation
    fn add_gas_cost(&mut self, gas_difference: i64) -> Result<u64, NeutronError>;



    /// Logs an error message. Only for diagnostic purposes, does not have any consensus effect and may effectively be a no-op
    fn log_error(&mut self, msg: &str);
    /// Logs an informational message. Only for diagnostic purposes, does not have any consensus effect and may effectively be a no-op
    fn log_info(&mut self, msg: &str);
    /// Logs a debug message. Only for diagnostic purposes, does not have any consensus effect and may effectively be a no-op
    fn log_debug(&mut self, msg: &str);
    */