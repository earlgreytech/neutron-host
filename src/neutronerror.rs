use std::fmt;
use std::error;



#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnrecoverableError{
    NotImplemented,
    StateOutOfRent,
    ContextIndexEmpty,
    DatabaseCommitError,
    DatabaseWritingError,
    ErrorInitializingVM,
    OutOfGas,
    TopLevelError(RecoverableError),
    InvalidElementOperation,
    DeveloperError //used for things that should only happen by Neutron developer error
}

//TODO: this later needs to be moved/copied to neutron-constants for sharing with neutron-star
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoverableError{
    ItemDoesntExist = 0x8000_0001,
    StackItemTooLarge,
    InvalidSystemFunction,
    InvalidSystemFeature,
    ErrorCopyingIntoVM,
    ErrorCopyingFromVM,
    ContractExecutionError,
    InvalidHypervisorInterrupt,
    StackItemTooSmall,
    InvalidVM,
    ContractRevertedExecution,
    InvalidCoMapAccess,
    LowTokenBalance,
    RequiresPermissionSelfAccess,
    RequiresPermissionSelfMod,
    RequiresPermissionExternalAccess,
    RequiresPermissionExternalMod,
    PureCallOfImpureContract
}

//TODO: add error codes for recoverable failures
/// The primary error structure of NeutronAPI calls
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeutronError{
	/// An error has occured, but if the VM implements an error handling system, it is appropriate to allow this error
    /// to be handled by the smart contract and for execution to continue
	Recoverable(RecoverableError),
    /// An error has occured and the VM should immediately terminate, not allowing the smart contract to detect or handle this error in any capacity
    Unrecoverable(UnrecoverableError)
}

impl fmt::Display for NeutronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NeutronError::Recoverable(e) => {
                write!(f, "Recoverable Failure! {:?}", e)
            },
            NeutronError::Unrecoverable(e) => {
                write!(f, "Unrecoverable Failure! {:?}", e)
            }
        }
    }
}

impl error::Error for NeutronError{
}



