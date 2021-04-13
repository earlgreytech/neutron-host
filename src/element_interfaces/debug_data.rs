use crate::callsystem::*;
use crate::codata::*;
use crate::neutronerror::NeutronError::*;
use crate::neutronerror::*;
use core::mem::transmute;
use neutron_common::RecoverableError;
use std::convert::TryInto;
use std::str;

/*
## Debug Data Injector ElementAPI

ID: 0x4000_0001

This API is used in a testing environment to inject data into a test instance of the Neutron stack
This file also contains some data structures that simplifies construction of this data

Data structures:

* mock_input_stack          - An initial input stack state that the testing smart contract can load
* expected_output_stack     - An expected state for the output stack that the testing smart contract can asserted against,
                              along with debug information to make a failed assertion more informative than "bytes in stacks didn't match"

functions:

* [0] Available()           - Check if API is available in current instance (standard function)
* [1] PushInputStack()      - Pushes the provided mock input stack to the current instance
* [2] AssertOutputStack()   - Asserts the provided expected output stack against the current instance

*/

// IDs >= 0x8000_0000 are meant for "internal" usage within the project, e.g. for unit/integration testing.
pub const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[derive(FromPrimitive)]
pub enum DebugDataFunctions {
    Available = 0, //reserved??
    PushInputStack = 1,
    AssertOutputStack = 2,
    GetInputStackLen = 10,
    ReverseInputStack = 11,
}

#[derive(Default)]
pub struct DebugDataInjector {
    pub mock_input_stack: DebugCoDataStack,
    pub expected_output_stack: DebugCoData,
}

impl ElementAPI for DebugDataInjector {
    fn system_call(
        &mut self,
        _callsystem: &CallSystem,
        mut codata: &mut CoData,
        _feature: u32,
        function: u32,
    ) -> Result<ElementResult, NeutronError> {
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none() {
            return Err(Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let f = f.unwrap();

        let result = match f {
            DebugDataFunctions::Available => {
                println!("[DebugDataInjector] Called with function: 0 (Available)");
                println!("    (This function simply informs the contract that this ElementAPI is available in its context)");
                Ok(())
            }
            DebugDataFunctions::PushInputStack => {
                println!("[DebugDataInjector] Called with function: 1 (PushInputStack)");

                for i in 0..self.mock_input_stack.len() {
                    println!(
                        "    Pushing item {}/{} to CoData stack...",
                        i + 1,
                        self.mock_input_stack.len()
                    );
                    codata.push_output_stack(self.mock_input_stack.stack[i].as_slice())?;
                }
                println!("    Done pushing to CoData stack!");

                Ok(())
            }
            DebugDataFunctions::AssertOutputStack => {
                println!("[DebugDataInjector] Called with function: 2 (AssertOutputStack)");
                println!("    Asserting state of CoData stack against expected state: ");
                self.expected_output_stack.assert_output_eq(&mut codata);

                Ok(())
            }
            DebugDataFunctions::GetInputStackLen => {
                println!("[DebugDataInjector] Called with function: 10 (GetInputStackLen)");

                println!("    Pushing length of provided input stack to CoData stack...");
                codata.push_output_stack(&[self.mock_input_stack.stack.len() as u8])?;

                Ok(())
            }
            DebugDataFunctions::ReverseInputStack => {
                println!("[DebugDataInjector] Called with function: 11 (ReverseInputStack)");

                println!("    Reverse order in provided input stack (first item become last item and so forth)...");
                self.mock_input_stack.reverse();

                Ok(())
            }
        };

        if result.is_err() {
            Err(result.unwrap_err())
        } else {
            return Ok(ElementResult::Result(0));
        }
    }
}

// Wrapper for a byte vector that represents a neutron codata stack
// Contains copies of internal ecosystem functions that normally manipulate the stacks
#[derive(Default)]
pub struct DebugCoDataStack {
    pub stack: Vec<Vec<u8>>,
}

impl DebugCoDataStack {
    // The following functions are adapted from neutron-star/src/syscall.rs

    pub fn push_u64(&mut self, value: u64) {
        const SIZE: usize = 8;
        let t = unsafe { transmute::<u64, [u8; SIZE]>(value) };
        self.stack.push(t.to_vec());
    }

    pub fn push_u32(&mut self, value: u32) {
        const SIZE: usize = 4;
        let t = unsafe { transmute::<u32, [u8; SIZE]>(value) };
        self.stack.push(t.to_vec());
    }

    pub fn push_u16(&mut self, value: u16) {
        const SIZE: usize = 2;
        let t = unsafe { transmute::<u16, [u8; SIZE]>(value) };
        self.stack.push(t.to_vec());
    }

    pub fn push_u8(&mut self, value: u8) {
        const SIZE: usize = 1;
        let t = unsafe { transmute::<u8, [u8; SIZE]>(value) };
        self.stack.push(t.to_vec());
    }

    pub fn push_bytes(&mut self, value: &[u8]) {
        self.stack.push(value.to_vec());
    }

    pub fn push_str(&mut self, value: &str) {
        self.stack.push(value.as_bytes().to_vec());
    }

    // These functions simply mirrors regular vector behavior for convenience

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.stack.pop()
    }

    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

    pub fn reverse(&mut self) {
        self.stack.reverse();
    }
}

// TODO: Make private?
pub enum DebugDataType {
    U64,
    U32,
    U16,
    U8,
    BYTES,
    STR,
}

// Used to easily construct an expected codata state, along with extra debugging information
#[derive(Default)]
pub struct DebugCoData {
    pub output_stack: DebugCoDataStack,
    pub variable_names: Vec<String>,
    pub variable_types: Vec<DebugDataType>,
}

impl DebugCoData {
    pub fn push_u64(&mut self, value: u64, name: &str) {
        self.output_stack.push_u64(value);
        self.push_debug_data(name, DebugDataType::U64);
    }

    pub fn push_u32(&mut self, value: u32, name: &str) {
        self.output_stack.push_u32(value);
        self.push_debug_data(name, DebugDataType::U32);
    }

    pub fn push_u16(&mut self, value: u16, name: &str) {
        self.output_stack.push_u16(value);
        self.push_debug_data(name, DebugDataType::U16);
    }

    pub fn push_u8(&mut self, value: u8, name: &str) {
        self.output_stack.push_u8(value);
        self.push_debug_data(name, DebugDataType::U8);
    }

    pub fn push_bytes(&mut self, value: &[u8], name: &str) {
        self.output_stack.push_bytes(value);
        self.push_debug_data(name, DebugDataType::BYTES);
    }

    pub fn push_str(&mut self, value: &str, name: &str) {
        self.output_stack.push_str(value);
        self.push_debug_data(name, DebugDataType::STR);
    }

    // Check contract output stack against expected state
    pub fn assert_output_eq(&mut self, codata: &mut CoData) {
        while self.variable_names.len() > 0 {
            let expected_data = self.output_stack.pop().unwrap();
            let actual_data = match codata.pop_input_stack() {
                Ok(v) => v,
                Err(_e) => panic!("\n\n[DebugCoData] Assertion failed: Output stack was exhausted before expected stack\n\n"),
            };

            let name = self.variable_names.pop().unwrap();
            let data_type = self.variable_types.pop().unwrap();

            match data_type {
                DebugDataType::U64 => assert_eq!(
                    self.to_u64(&expected_data),
                    self.to_u64(&actual_data),
                    "\n\n[DebugCoData] Assertion failed for u64 named {}\n\n",
                    name
                ),
                DebugDataType::U32 => assert_eq!(
                    self.to_u32(&expected_data),
                    self.to_u32(&actual_data),
                    "\n\n[DebugCoData] Assertion failed for u32 named {}\n\n",
                    name
                ),
                DebugDataType::U16 => assert_eq!(
                    self.to_u16(&expected_data),
                    self.to_u16(&actual_data),
                    "\n\n[DebugCoData] Assertion failed for u16 named {}\n\n",
                    name
                ),
                DebugDataType::U8 => assert_eq!(
                    self.to_u8(&expected_data),
                    self.to_u8(&actual_data),
                    "\n\n[DebugCoData] Assertion failed for u8 named {}\n\n",
                    name
                ),
                DebugDataType::BYTES => assert_eq!(
                    expected_data, actual_data,
                    "\n\n[DebugCoData] Assertion failed for byte sequence named {}\n\n",
                    name
                ),
                DebugDataType::STR => assert_eq!(
                    str::from_utf8(&expected_data).unwrap(),
                    str::from_utf8(&actual_data).unwrap(),
                    "\n\n[DebugCoData] Assertion failed for str named {}\n\n",
                    name
                ),
            };
        }

        // Check that there is no unexpected data left on the codata stack
        match codata.pop_input_stack() {
            Ok(_v) => panic!("\n\n[DebugCoData] Assertion failed: Additional data found on output stack after expected stack was exhausted\n\n"),
            Err(_e) => {},
        }
    }

    // Push debug data to vectors
    fn push_debug_data(&mut self, name: &str, type_enum: DebugDataType) {
        self.variable_names.push(String::from(name));
        self.variable_types.push(type_enum);
    }

    // Transmute byte slices from the stacks to values
    fn to_u64(&mut self, value: &[u8]) -> u64 {
        let array: [u8; 8] = value[0..8]
            .try_into()
            .expect("to_u64: Slice was of incorrect length");
        unsafe { return transmute::<[u8; 8], u64>(array) };
    }
    fn to_u32(&mut self, value: &[u8]) -> u32 {
        let array: [u8; 4] = value[0..4]
            .try_into()
            .expect("to_u32: Slice was of incorrect length");
        unsafe { return transmute::<[u8; 4], u32>(array) };
    }
    fn to_u16(&mut self, value: &[u8]) -> u16 {
        let array: [u8; 2] = value[0..2]
            .try_into()
            .expect("to_u16: Slice was of incorrect length");
        unsafe { return transmute::<[u8; 2], u16>(array) };
    }
    fn to_u8(&mut self, value: &[u8]) -> u8 {
        let array: [u8; 1] = value[0..1]
            .try_into()
            .expect("to_u8: Slice was of incorrect length");
        unsafe { return transmute::<[u8; 1], u8>(array) };
    }
}
