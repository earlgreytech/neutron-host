use crate::callsystem::*;
use crate::codata::*;
use crate::comap_abi_decoder::*;
use crate::neutronerror::NeutronError::*;
use crate::neutronerror::*;
use neutron_common::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;

/*
## Debug Data Injector ElementAPI

ID: 0x4000_0001

This API is used in a testing environment to inject data into a test instance of the Neutron stack
This file also contains some data structures that simplifies construction of this data

Data structures:

* inject_stack              - An initial input stack state that the testing smart contract can load

* expect_stack              - An expected state for the output stack that the testing smart contract can asserted against,
                              along with debug information to make a failed assertion more informative than "bytes in stacks didn't match"

* inject_map                - An initial result stack state that the testing smart contract can load

* expect_map                - An expected state for the output map that the testing smart contract can asserted against

functions:

* [0]  Available()          - Check if API is available in current instance (standard function)

* [1]  PushInputStack()     - Pushes the provided mock input stack to the current instance
                              (Actually pushes to output stack, but it becomes input stack on return to contract)

* [2]  AssertOutputStack()  - Asserts the provided expected output stack against the current instance
                              (Actually asserts against input stack, but it was the output stack before entering this function)

* [3]  PushResultMap()      - Pushes the provided mock result map to the current instance
                              (Actually pushes to output map, but it becomes result map on return to contract)

* [4]  AssertOutputMap()    - Asserts the provided expected output map against the current instance
                             (Actually asserts against input map, but it was the output stack before entering this function)

* [10] GetInputStackLen()   - Push length of provided mock input stack to the current instance

* [11] ReverseInputStack()  - Reverse item order of provided mock input stack

*/

// IDs >= 0x8000_0000 are meant for "internal" usage within the project, e.g. for unit/integration testing.
pub const DEBUG_DATA_FEATURE: u32 = 0x4000_0001;

#[derive(FromPrimitive)]
pub enum DebugDataFunctions {
    Available = 0, //reserved??
    PushInputStack = 1,
    AssertOutputStack = 2,
    PushResultMap = 3,
    AssertOutputMap = 4,
    GetInputStackLen = 10,
    ReverseInputStack = 11,
}

#[derive(Default)]
pub struct DebugDataInjector {
    pub inject_stack: DebugCoStack,
    pub expect_stack: WrappedDebugCoStack,
    pub inject_map: DebugCoMap,
    pub expect_map: DebugCoMap,
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

                println!("    Pushing provided stack to codata stack...");
                self.inject_stack.push_to_codata(&mut codata);
                println!("    Done!");

                Ok(())
            }
            DebugDataFunctions::AssertOutputStack => {
                println!("[DebugDataInjector] Called with function: 2 (AssertOutputStack)");

                println!("    Asserting state of CoData stack against expected state... ");
                self.expect_stack.assert_eq(&mut codata);
                println!("    Done!");

                Ok(())
            }
            DebugDataFunctions::PushResultMap => {
                println!("[DebugDataInjector] Called with function: 3 (PushResultMap)");

                println!("    Pushing provided map to codata map...");
                self.inject_map.push_to_codata(&mut codata);
                println!("    Done!");

                Ok(())
            }
            DebugDataFunctions::AssertOutputMap => {
                println!("[DebugDataInjector] Called with function: 4 (AssertOutputMap)");

                println!("    Asserting state of CoData map against expected state... ");
                self.expect_map.assert_eq(&mut codata);
                println!("    Done!");

                Ok(())
            }
            DebugDataFunctions::GetInputStackLen => {
                println!("[DebugDataInjector] Called with function: 10 (GetInputStackLen)");

                println!("    Pushing length of provided input stack to CoData stack...");
                self.inject_stack.push_len_to_codata(&mut codata);
                println!("    Done!");

                Ok(())
            }
            DebugDataFunctions::ReverseInputStack => {
                println!("[DebugDataInjector] Called with function: 11 (ReverseInputStack)");

                println!("    Reverse order in provided input stack (first item become last item and so forth)...");
                self.inject_stack.reverse();
                println!("    Done!");

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

// A vector of byte vectors that represents a neutron codata stack
// Contains copies of internal ecosystem functions that normally manipulate the stacks
#[derive(Default)]
pub struct DebugCoStack {
    pub stack: Vec<Vec<u8>>,
}

impl DebugCoStack {
    // These functions mostly mirror codata stack behavior
    // (adapted from neutron-star/src/syscall.rs)

    // Single values

    pub fn push_u8(&mut self, value: u8) {
        self.stack.push([value].to_vec());
    }

    pub fn push_u16(&mut self, value: u16) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_u32(&mut self, value: u32) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_u64(&mut self, value: u64) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_i8(&mut self, value: i8) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_i16(&mut self, value: i16) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_i32(&mut self, value: i32) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_i64(&mut self, value: i64) {
        self.stack.push(value.to_le_bytes().to_vec());
    }

    pub fn push_address(&mut self, value: NeutronAddress) {
        let mut bytes = value.version.to_le_bytes().to_vec();
        bytes.append(&mut value.data.to_vec());
        self.stack.push(bytes);
    }

    // Array values

    pub fn push_array_u8(&mut self, slice: &[u8]) {
        self.stack.push(slice.to_vec()); // No need to cast byte to byte
    }

    pub fn push_array_u16(&mut self, slice: &[u16]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_u32(&mut self, slice: &[u32]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_u64(&mut self, slice: &[u64]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_i8(&mut self, slice: &[i8]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_i16(&mut self, slice: &[i16]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_i32(&mut self, slice: &[i32]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_i64(&mut self, slice: &[i64]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].to_le_bytes());
        }
        self.stack.push(bytes);
    }

    pub fn push_array_address(&mut self, slice: &[NeutronAddress]) {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..slice.len() {
            bytes.extend_from_slice(&slice[i].version.to_le_bytes());
            bytes.extend_from_slice(&slice[i].data);
        }
        self.stack.push(bytes);
    }

    // Misc type values

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

    // Push to codata output stack (Will become input stack on return to contract)
    pub fn push_to_codata(&mut self, codata: &mut CoData) {
        for i in 0..self.len() {
            codata.push_output_stack(self.stack[i].as_slice()).unwrap();
        }
    }

    // Push length to codata output stack
    pub fn push_len_to_codata(&mut self, codata: &mut CoData) {
        codata.push_output_stack(&[self.stack.len() as u8]).unwrap();
    }
}

// TODO: Make private?
pub enum DebugDataType {
    U64,
    U32,
    U16,
    U8,
    I64,
    I32,
    I16,
    I8,
    ADDRESS,
    BYTES,
    STR,
}

// Used to easily construct an expected output CoStack state, along with extra debugging information
#[derive(Default)]
pub struct WrappedDebugCoStack {
    pub output_stack: DebugCoStack,
    pub variable_names: Vec<String>,
    pub variable_types: Vec<DebugDataType>,
}

// Cast bytes to integer without error chacking
// Accepts both slices and slice-able types like vectors
// Expected behavior:
// * Too many input bytes -> Extra bytes ignored
// * Too few input bytes -> Index out of bounds error
macro_rules! bytes_to_integer {
    ($VALUE:ident, $TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();
        let array: [u8; SIZE] = $VALUE[0..SIZE].try_into().unwrap();
        $TYPE::from_le_bytes(array)
    }};
}

impl WrappedDebugCoStack {
    // CoStack functions

    pub fn push_u8(&mut self, value: u8, name: &str) {
        self.output_stack.push_u8(value);
        self.push_debug_data(name, DebugDataType::U8);
    }

    pub fn push_u16(&mut self, value: u16, name: &str) {
        self.output_stack.push_u16(value);
        self.push_debug_data(name, DebugDataType::U16);
    }

    pub fn push_u32(&mut self, value: u32, name: &str) {
        self.output_stack.push_u32(value);
        self.push_debug_data(name, DebugDataType::U32);
    }

    pub fn push_u64(&mut self, value: u64, name: &str) {
        self.output_stack.push_u64(value);
        self.push_debug_data(name, DebugDataType::U64);
    }

    pub fn push_i8(&mut self, value: i8, name: &str) {
        self.output_stack.push_i8(value);
        self.push_debug_data(name, DebugDataType::I8);
    }

    pub fn push_i16(&mut self, value: i16, name: &str) {
        self.output_stack.push_i16(value);
        self.push_debug_data(name, DebugDataType::I16);
    }

    pub fn push_i32(&mut self, value: i32, name: &str) {
        self.output_stack.push_i32(value);
        self.push_debug_data(name, DebugDataType::I32);
    }

    pub fn push_i64(&mut self, value: i64, name: &str) {
        self.output_stack.push_i64(value);
        self.push_debug_data(name, DebugDataType::I64);
    }

    pub fn push_address(&mut self, value: NeutronAddress, name: &str) {
        self.output_stack.push_address(value);
        self.push_debug_data(name, DebugDataType::ADDRESS);
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
    pub fn assert_eq(&mut self, codata: &mut CoData) {
        while self.variable_names.len() > 0 {
            let expected_data = self.output_stack.pop().unwrap();
            let actual_data = match codata.pop_input_stack() {
                Ok(v) => v,
                Err(_e) => panic!("\n\n[DebugCoData] Assertion failed: Output stack was exhausted before expected stack\n\n"),
            };

            let name = self.variable_names.pop().unwrap();
            let data_type = self.variable_types.pop().unwrap();

            match data_type {
                DebugDataType::U8 => assert_eq!(
                    bytes_to_integer!(expected_data, u8),
                    bytes_to_integer!(actual_data, u8),
                    "\n\n[DebugCoData] Assertion failed for u8 named '{}'\n\n",
                    name
                ),
                DebugDataType::U16 => assert_eq!(
                    bytes_to_integer!(expected_data, u16),
                    bytes_to_integer!(actual_data, u16),
                    "\n\n[DebugCoData] Assertion failed for u16 named '{}'\n\n",
                    name
                ),
                DebugDataType::U32 => assert_eq!(
                    bytes_to_integer!(expected_data, u32),
                    bytes_to_integer!(actual_data, u32),
                    "\n\n[DebugCoData] Assertion failed for u32 named '{}'\n\n",
                    name
                ),
                DebugDataType::U64 => assert_eq!(
                    bytes_to_integer!(expected_data, u64),
                    bytes_to_integer!(actual_data, u64),
                    "\n\n[DebugCoData] Assertion failed for u64 named {}\n\n",
                    name
                ),
                DebugDataType::I8 => assert_eq!(
                    bytes_to_integer!(expected_data, i8),
                    bytes_to_integer!(actual_data, i8),
                    "\n\n[DebugCoData] Assertion failed for i8 named '{}'\n\n",
                    name
                ),
                DebugDataType::I16 => assert_eq!(
                    bytes_to_integer!(expected_data, i16),
                    bytes_to_integer!(actual_data, i16),
                    "\n\n[DebugCoData] Assertion failed for i16 named '{}'\n\n",
                    name
                ),
                DebugDataType::I32 => assert_eq!(
                    bytes_to_integer!(expected_data, i32),
                    bytes_to_integer!(actual_data, i32),
                    "\n\n[DebugCoData] Assertion failed for i32 named '{}'\n\n",
                    name
                ),
                DebugDataType::I64 => assert_eq!(
                    bytes_to_integer!(expected_data, i64),
                    bytes_to_integer!(actual_data, i64),
                    "\n\n[DebugCoData] Assertion failed for i64 named {}\n\n",
                    name
                ),
                DebugDataType::ADDRESS => assert_eq!(
                    NeutronAddress::from_data(&expected_data),
                    NeutronAddress::from_data(&actual_data),
                    "\n\n[DebugCoData] Assertion failed for NeutronAddress named '{}'\n\n",
                    name
                ),
                DebugDataType::BYTES => assert_eq!(
                    expected_data, actual_data,
                    "\n\n[DebugCoData] Assertion failed for byte sequence named '{}'\n\n",
                    name
                ),
                DebugDataType::STR => assert_eq!(
                    str::from_utf8(&expected_data).unwrap(),
                    str::from_utf8(&actual_data).unwrap(),
                    "\n\n[DebugCoData] Assertion failed for str named '{}'\n\n",
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
}

// Wrapper for a key/value hashmap that represents a neutron codata map
// Contains copies of internal ecosystem functions that normally manipulate the maps
#[derive(Default)]
pub struct DebugCoMap {
    pub map: HashMap<Vec<u8>, Vec<u8>>,
}

// TODO: Add typed push functions, and maybe a wrapped version with extra debug info? (like WrappedDebugCoStack)
impl DebugCoMap {
    // These functions mirror codata map behavior
    pub fn push_key(&mut self, key: &[u8], value: &[u8]) -> Result<(), NeutronError> {
        if key[0] == 0 {
            return Err(NeutronError::Recoverable(
                RecoverableError::InvalidCoMapAccess,
            ));
        }
        self.map.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    // For u32 ABI header
    pub fn push_key_abi(
        &mut self,
        key: &[u8],
        value: &[u8],
        abi_data: u32,
    ) -> Result<(), NeutronError> {
        let (header_size, header_bytes) = comap_abi_header_from_u32(abi_data);
        let mut full_value = vec![];
        full_value.extend_from_slice(&header_bytes[0..header_size]);
        full_value.extend_from_slice(&value);
        self.push_key(key, &full_value)
    }

    pub fn peek_key(&mut self, key: &[u8]) -> Result<Vec<u8>, NeutronError> {
        if key[0] == 0 {
            return Err(NeutronError::Recoverable(
                RecoverableError::InvalidCoMapAccess,
            ));
        }
        match self.map.get(key) {
            Some(v) => Ok(v.to_vec()),
            None => Err(Recoverable(RecoverableError::ItemDoesntExist)),
        }
    }

    // Check contract output map against expected state
    pub fn assert_eq(&mut self, codata: &mut CoData) {
        println!("[DebugCoData] Asserting expected CoMap values against actual output CoMap...");
        for key in self.map.keys() {
            let key_str = String::from_utf8_lossy(key);

            let expected_data = self.map.get(key).unwrap();
            let actual_data = match codata.peek_input_key(key) {
                Ok(v) => v,
                Err(_e) => panic!(
                    "\n\n    Assertion failed: Actual output map lacked entry for key '{}'\n\n",
                    key_str
                ),
            };
            let expected_data_str = String::from_utf8_lossy(expected_data);
            let actual_data_str = String::from_utf8_lossy(&actual_data);

            assert_eq!(
                expected_data, &actual_data,
                "\n\n    Assertion failed for codata entry with key '{}' and string values: \nExpected: '{}' \nActual:'{}' \n\n",
                key_str,
                expected_data_str,
                actual_data_str
            );
            println!("    CoMap entry with key '{}' matched!", key_str);
        }
    }

    // Push to codata output map (Will become result map on return to contract)
    // There is no function to push to result map, because it would be discarded on return from ElemAPI anyway
    pub fn push_to_codata(&mut self, codata: &mut CoData) {
        for key in self.map.keys() {
            let data = self.map.get(key).unwrap();
            codata.push_output_key(&key, data).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DebugCoStack::push_u8(u8)
    #[test]
    fn test_debugcostack_push_u8() {
        let mut stack = DebugCoStack::default();
        stack.push_u8(0x11 as u8);
        let expected_bytes: Vec<u8> = vec![0x11];
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_u8(&[u8])
    #[test]
    fn test_debugcostack_push_array_u8() {
        let mut stack = DebugCoStack::default();
        let array: [u8; 3] = [1, 2, 3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array);

        stack.push_array_u8(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_u16(u16)
    #[test]
    fn test_debugcostack_push_u16() {
        let mut stack = DebugCoStack::default();
        stack.push_u16(0x1122 as u16);
        let expected_bytes: Vec<u8> = vec![0x22, 0x11];
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_u16(&[u16])
    #[test]
    fn test_debugcostack_push_array_u16() {
        let mut stack = DebugCoStack::default();
        let array: [u16; 3] = [1, 2, 3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_u16(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_u32(u32)
    #[test]
    fn test_debugcostack_push_u32() {
        let mut stack = DebugCoStack::default();
        stack.push_u32(0x1122_3344 as u32);
        let expected_bytes: Vec<u8> = vec![0x44, 0x33, 0x22, 0x11];
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_u32(&[u32])
    #[test]
    fn test_debugcostack_push_array_u32() {
        let mut stack = DebugCoStack::default();
        let array: [u32; 3] = [1, 2, 3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_u32(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_u64(u64)
    #[test]
    fn test_debugcostack_push_u64() {
        let mut stack = DebugCoStack::default();
        stack.push_u64(0x1122_3344_5566_7788 as u64);
        let expected_bytes: Vec<u8> = vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_u64(&[u64])
    #[test]
    fn test_debugcostack_push_array_u64() {
        let mut stack = DebugCoStack::default();
        let array: [u64; 3] = [1, 2, 3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_u64(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_i8(i8)
    #[test]
    fn test_debugcostack_push_i8() {
        let mut stack = DebugCoStack::default();
        let num: i8 = i8::MIN / 2;
        stack.push_i8(num);
        let expected_bytes: Vec<u8> = num.to_le_bytes().to_vec();
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_i8(&[i8])
    #[test]
    fn test_debugcostack_push_array_i8() {
        let mut stack = DebugCoStack::default();
        let array: [i8; 3] = [-1, -2, -3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_i8(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_i16(i16)
    #[test]
    fn test_debugcostack_push_i16() {
        let mut stack = DebugCoStack::default();
        let num: i16 = i16::MIN / 2;
        stack.push_i16(num);
        let expected_bytes: Vec<u8> = num.to_le_bytes().to_vec();
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_i16(&[i16])
    #[test]
    fn test_debugcostack_push_array_i16() {
        let mut stack = DebugCoStack::default();
        let array: [i16; 3] = [-1, -2, -3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_i16(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_i32(i32)
    #[test]
    fn test_debugcostack_push_i32() {
        let mut stack = DebugCoStack::default();
        let num: i32 = i32::MIN / 2;
        stack.push_i32(num);
        let expected_bytes: Vec<u8> = num.to_le_bytes().to_vec();
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_i32(&[i32])
    #[test]
    fn test_debugcostack_push_array_i32() {
        let mut stack = DebugCoStack::default();
        let array: [i32; 3] = [-1, -2, -3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_i32(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_i64(i64)
    #[test]
    fn test_debugcostack_push_i64() {
        let mut stack = DebugCoStack::default();
        let num: i64 = i64::MIN / 2;
        stack.push_i64(num);
        let expected_bytes: Vec<u8> = num.to_le_bytes().to_vec();
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_array_i64(&[i64])
    #[test]
    fn test_debugcostack_push_array_i64() {
        let mut stack = DebugCoStack::default();
        let array: [i64; 3] = [-1, -2, -3];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&array[0].to_le_bytes());
        expected_bytes.extend_from_slice(&array[1].to_le_bytes());
        expected_bytes.extend_from_slice(&array[2].to_le_bytes());

        stack.push_array_i64(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_address(NeutronAddress)
    #[test]
    fn test_debugcostack_push_address() {
        let mut stack = DebugCoStack::default();

        let version: u32 = 123456789;
        let data: [u8; 20] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        let mut bytes = version.to_le_bytes().to_vec();
        bytes.append(&mut data.to_vec());
        stack.push_address(NeutronAddress::from_data(&bytes));
        assert_eq!(stack.stack[0], bytes);
    }

    // DebugCoStack::push_array_address(&[NeutronAddress])
    #[test]
    fn test_debugcostack_push_array_address() {
        let mut stack = DebugCoStack::default();

        // For simplicity we just create a byte sequence with the same length as the address struct
        let mut version_and_data: [[u8; 24]; 3] = [[0; 24], [0; 24], [0; 24]];
        for i in 0..24 {
            version_and_data[0][i] = (i + 1) as u8;
            version_and_data[1][i] = (i + 11) as u8;
            version_and_data[2][i] = (i + 101) as u8;
        }

        let array: [NeutronAddress; 3] = [
            NeutronAddress::from_data(&version_and_data[0]),
            NeutronAddress::from_data(&version_and_data[1]),
            NeutronAddress::from_data(&version_and_data[2]),
        ];

        let mut expected_bytes: Vec<u8> = vec![];
        expected_bytes.extend_from_slice(&version_and_data[0]);
        expected_bytes.extend_from_slice(&version_and_data[1]);
        expected_bytes.extend_from_slice(&version_and_data[2]);

        stack.push_array_address(&array);
        assert_eq!(stack.stack[0], expected_bytes);
    }

    // DebugCoStack::push_bytes(&[u8])
    #[test]
    fn test_debugcostack_push_bytes() {
        let bytes: Vec<u8> = vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];

        let mut stack = DebugCoStack::default();
        stack.push_bytes(&bytes);
        assert_eq!(stack.stack[0], bytes);
    }

    // DebugCoStack::push_str(&str)
    #[test]
    fn test_debugcostack_push_str() {
        let string = "This is a testing string!";

        let mut stack = DebugCoStack::default();
        stack.push_str(string);
        assert_eq!(&stack.stack[0], string.as_bytes());
    }

    #[test]
    fn test_bytes_to_integer_macro() {
        let bytes: &[u8] = &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
        let result = bytes_to_integer!(bytes, u64);
        assert_eq!(result, 0x1122_3344_5566_7788 as u64);
    }
    #[test]
    fn test_bytes_to_integer_macro_too_long() {
        let bytes: &[u8] = &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0xFF];
        let result = bytes_to_integer!(bytes, u64);
        assert_eq!(result, 0x1122_3344_5566_7788 as u64);
    }
    #[test]
    #[should_panic]
    fn negtest_bytes_to_integer_macro_too_short() {
        let bytes: &[u8] = &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22];
        let result = bytes_to_integer!(bytes, u64);
        assert_eq!(result, 0x1122_3344_5566_7788 as u64);
    }

    // TODO: Tests for debug comap functionality
}
