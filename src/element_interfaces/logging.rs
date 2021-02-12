use crate::codata::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
use crate::callsystem::*;
/*
## Logging

ID: 2

Functions:

* log_debug(count, string, ...)
* log_info(count, string, ...)
* log_warning(count, string, ...)
* log_error(count, string, ...)

The exact order of printing messages is backward from what would be expected!
This is designed so that no allocator is required for doing `println!` functions within neutron-star.

The expense of reordering the strings etc is a cost on the CallSystem. This could potentially be somewhat expensive, 
but since logging is informative only and can easily be a no-op (other than needing to pop off appropriate number of stack items) this incurs no real risk.

Note in neutron-star, log_info is used by default for println!
*/

pub const LOGGING_FEATURE: u32 = 4;

#[derive(FromPrimitive)]
pub enum LoggingFunctions{
    Available = 0, //reserved??
    LogDebug = 1,
    LogInfo,
    LogWarning,
    LogError
}

impl <'a>ElementAPI for (dyn LoggingInterface + 'a){
    fn system_call(&mut self, _callsystem: &CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        self.try_syscall(codata, feature, function)
    }
}

pub trait LoggingInterface{
    fn try_syscall(&mut self, stack: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        if feature != LOGGING_FEATURE {
            return Ok(ElementResult::Result(0));
        }
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none(){
            return Err(Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let f=f.unwrap();
        let message = compile_log_message(stack)?;
        let result = match f{
            LoggingFunctions::LogDebug => {
                self.log_debug(stack, message)
            },
            LoggingFunctions::LogInfo => {
                self.log_info(stack, message)
            },
            LoggingFunctions::LogWarning => {
                self.log_warning(stack, message)
            },
            LoggingFunctions::LogError => {
                self.log_error(stack, message)
            },
            LoggingFunctions::Available => {
                Ok(())
            },
        };
        if result.is_err(){
            Err(result.unwrap_err())
        }else{
            return Ok(ElementResult::Result(0));
        }
    }
    fn log_debug(&mut self, stack: &mut CoData, message: String) -> Result<(), NeutronError>;
    fn log_info(&mut self, stack: &mut CoData, message: String) -> Result<(), NeutronError>;
    fn log_warning(&mut self, stack: &mut CoData, message: String) -> Result<(), NeutronError>;
    fn log_error(&mut self, stack: &mut CoData, message: String) -> Result<(), NeutronError>;
}

pub struct StdoutLogger{
}
impl LoggingInterface for StdoutLogger{
    fn log_debug(&mut self, _stack: &mut CoData, message: String) -> Result<(), NeutronError>{
        println!("NEUTRON DEBUG: {}", message);
        Ok(())
    }
    fn log_info(&mut self, _stack: &mut CoData, message: String) -> Result<(), NeutronError>{
        println!("NEUTRON INFO: {}", message);
        Ok(())
    }
    fn log_warning(&mut self, _stack: &mut CoData, message: String) -> Result<(), NeutronError>{
        println!("NEUTRON WARNING: {}", message);
        Ok(())
    }
    fn log_error(&mut self, _stack: &mut CoData, message: String) -> Result<(), NeutronError>{
        println!("NEUTRON ERROR: {}", message);
        Ok(())
    }
}


pub fn compile_log_message(stack: &mut CoData) -> Result<String, NeutronError>{
    let count = stack.pop_input_stack()?;
    if count.len() < 1{
        return Err(NeutronError::Recoverable(RecoverableError::StackItemTooSmall));
    }
    if count.len() > 1 {
        return Err(Recoverable(RecoverableError::StackItemTooLarge));
    }
    let count = count.get(0).unwrap();
    let mut messages:Vec<String> = vec![];
    for _ in 0..*count{
        let s = stack.pop_input_stack()?;
        let string = std::string::String::from_utf8_lossy(&s);
        messages.push(string.to_owned().to_string());
    }
    let mut string = String::default();
    for msg in messages.iter().rev(){
        string.push_str(&msg);
    }
    Ok(string)
}

