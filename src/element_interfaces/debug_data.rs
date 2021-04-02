use crate::codata::*;
use crate::neutronerror::*;
use crate::neutronerror::NeutronError::*;
use neutron_common::RecoverableError;
use crate::callsystem::*;
use std::str;
use core::mem::transmute;
/*
## NAME

ID: 0x8000_0001

Functions:

* FUNCTIONS
 
 TODO: This entire comment block

*/

// IDs >= 0x8000_0000 are meant for "internal" usage within the project, e.g. for unit/integration testing. 
pub const DEBUG_DATA_FEATURE: u32 = 0x8000_0001;

#[derive(FromPrimitive)]
pub enum DebugDataFunctions{
    Available = 0, //reserved??
    PushInputStack = 1,
    AssertOutputStack = 2,
}

#[derive(Default)]
pub struct DebugDataInjector{
    pub input_stack: DebugCoDataStack,
    pub debug_codata: DebugCoData,
}

impl ElementAPI for DebugDataInjector{
    fn system_call(&mut self, _callsystem: &CallSystem, codata: &mut CoData, feature: u32, function: u32) -> Result<ElementResult, NeutronError>{
        
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none(){
            return Err(Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let f=f.unwrap();
        
        let result = match f {
            DebugDataFunctions::Available => {
                println!("[DebugData] Called with feature: 0 (Available)");
                Ok(())
            }, 
            DebugDataFunctions::PushInputStack => {
                println!("[DebugData] Called with feature: 1 (PushInputStack)");

                codata.push_output_stack(&self.input_stack.stack.as_slice());
                Ok(())
            }, 
            DebugDataFunctions::AssertOutputStack => {
                println!("[DebugData] Called with feature: 2 (AssertOutputStack)");
                
                let mut exec_output_stack = codata.pop_input_stack().unwrap();

                println!("    Popped {} elements from codata", exec_output_stack.len());

                self.debug_codata.assert_output_eq(&mut exec_output_stack);

                Ok(())
            }, 
        };
        
        if result.is_err(){
            Err(result.unwrap_err())
        }else{
            return Ok(ElementResult::Result(0));
        }
    }
}

// Used to easily construct an expected codata state, along with some extra debugging information
#[derive(Default)]
pub struct DebugCoDataStack {
    pub stack: Vec<u8>,
}

impl DebugCoDataStack {

    // The following functions are adapted from neutron-star/src/syscall.rs
    
    /// Pushes a u64 value to the stack. 
    pub fn push_u64(&mut self, value: u64){
        const SIZE:usize = 8;
        let t = unsafe{
            transmute::<u64, [u8; SIZE]>(value)
        };
        for byte in &t {
            self.stack.push(*byte);
        }
    }
    /// Pushes a u32 value to the stack. 
    pub fn push_u32(&mut self, value: u32){
        const SIZE:usize = 4;
        let t = unsafe{
            transmute::<u32, [u8; SIZE]>(value)
        };
        for byte in &t {
            self.stack.push(*byte);
        }
    }

    /// Pushes a u16 value to the stack. 
    pub fn push_u16(&mut self, value: u16){
        const SIZE:usize = 2;
        let t = unsafe{
            transmute::<u16, [u8; SIZE]>(value)
        };
        for byte in &t {
            self.stack.push(*byte);
        }
    }

    /// Pushes a u8 value to the stack. 
    pub fn push_u8(&mut self, value: u8){
        const SIZE:usize = 1;
        let t = unsafe{
            transmute::<u8, [u8; SIZE]>(value)
        };
        for byte in &t {
            self.stack.push(*byte);
        }
    }
    
    /// Pushes a byte sequence to the stack. 
    pub fn push_bytes(&mut self, value: &[u8]){
        for byte in value {
            self.stack.push(*byte);
        }
    }
    
    // These functions simply mirrors regular vector behavior for convenience
    
    pub fn pop(&mut self) -> Option<u8>{
        self.stack.pop()
    }
    
    pub fn len(&mut self) -> usize{
        self.stack.len()
    }
}


// Used to easily construct an expected codata state, along with extra debugging information
#[derive(Default)]
pub struct DebugCoData {
    pub output_stack: DebugCoDataStack,
    pub variable_names: Vec<String>,
    pub variable_types: Vec<String>,
    pub variable_sizes: Vec<usize>,
}

impl DebugCoData {
    /// Pushes a u64 value to the stack. 
    pub fn push_u64(&mut self, value: u64, name: &str){
        const SIZE:usize = 8;
        self.output_stack.push_u64(value);
        
        self.variable_names.push(String::from(name));
        self.variable_types.push(String::from("u64"));
        self.variable_sizes.push(SIZE);
    }
    /// Pushes a u32 value to the stack. 
    pub fn push_u32(&mut self, value: u32, name: &str){
        const SIZE:usize = 4;
        self.output_stack.push_u32(value);
        
        self.variable_names.push(String::from(name));
        self.variable_types.push(String::from("u32"));
        self.variable_sizes.push(SIZE);
    }

    /// Pushes a u16 value to the stack. 
    pub fn push_u16(&mut self, value: u16, name: &str){
        const SIZE:usize = 2;
        self.output_stack.push_u16(value);
        
        self.variable_names.push(String::from(name));
        self.variable_types.push(String::from("u16"));
        self.variable_sizes.push(SIZE);
    }

    /// Pushes a u8 value to the stack. 
    pub fn push_u8(&mut self, value: u8, name: &str){
        const SIZE:usize = 1;
        self.output_stack.push_u8(value);
        
        self.variable_names.push(String::from(name));
        self.variable_types.push(String::from("u8"));
        self.variable_sizes.push(SIZE);
    }
    
    /// Pushes a byte sequence to the stack. 
    pub fn push_bytes(&mut self, value: &[u8], name: &str){
        let SIZE:usize = value.len();
        self.output_stack.push_bytes(value);
        
        self.variable_names.push(String::from(name));
        self.variable_types.push(String::from("byte sequence"));
        self.variable_sizes.push(SIZE);
    }
    
    
    // Checks stack
    pub fn assert_output_eq(&mut self, stack_to_compare: &mut Vec<u8>){
        let expected_stack_length = self.output_stack.stack.len();
        let actual_stack_length = stack_to_compare.len();
        // If length isn't equal it means there's a mismatch of variable count and/or types
        assert!(expected_stack_length == actual_stack_length, "\n\n[DebugCoData] assert_output_eq: Expected output stack size {}, was {}. \n\n", expected_stack_length, actual_stack_length);
        
        // If we expected zero-length output and got zero-length output we are done
        if expected_stack_length == 0 {
            return ();
        }
        
        while self.variable_sizes.len() > 0 {
            let name = self.variable_names.pop().unwrap();
            let data_type = self.variable_types.pop().unwrap();
            let size = self.variable_sizes.pop().unwrap();
            for _i in 0..size {
                let byte_expected = self.output_stack.pop();
                let byte_comparison = stack_to_compare.pop();
                assert!(byte_expected == byte_comparison, "\n\n[DebugCoData] assert_output_eq: Wrong value found in {} named {}. \n\n", data_type, name);
            }
            println!("[DebugCoData] assert_output_eq: Correct value found in {} named {}", data_type, name);
        }
    }
}