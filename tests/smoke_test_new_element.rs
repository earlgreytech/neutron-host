mod common;

extern crate num;
#[macro_use]
extern crate num_derive;

use neutron_common::RecoverableError;
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::harness::*;
use neutron_host::interface::*;
use neutron_host::neutronerror::*;
use std::cell::RefCell;

use common::*;

const CONTRACT_NAME: &'static str = "smoke_test_new_element";

// Test that basic smart contract execution doesn't throw an error
#[test]
fn test_new_element() {
    for target in vec!["debug", "release"] {
        let mut harness = TestHarness::default();
        let context = ExecutionContext::create_default_random_context();
        harness.db.checkpoint().unwrap();
        let mut cs = CallSystem::default();
        cs.global_storage = Some(RefCell::new(&mut harness.db));
        cs.logging = Some(RefCell::new(&mut harness.logger));
        let mut file_element = FileElement {};
        cs.add_call(FILE_ELEMENT_ID, &mut file_element).unwrap();
        let result = harness
            .instance
            .execute_binary(&get_contract_path_target(CONTRACT_NAME, target), &cs, context);
        harness.db.commit().unwrap();
        assert_eq!(result.status, 0);
    }
}

const FILE_ELEMENT_ID: u32 = 0x8000_0001;

#[derive(FromPrimitive)]
pub enum FileFunctions {
    /// returns 1
    Available = 0,
    /// ReadFile(file_name: stack [u8]) -> (length: u32, contents: stack [u8])
    ReadFile = 1,
    /// FileExists(file_name: stack [u8]) -> exists: u32
    FileExists = 2,
}
struct FileElement {}

impl ElementAPI for FileElement {
    fn system_call(
        &mut self,
        _callsystem: &CallSystem,
        codata: &mut CoData,
        feature: u32,
        function: u32,
    ) -> Result<ElementResult, NeutronError> {
        if feature != FILE_ELEMENT_ID {
            return Ok(ElementResult::Result(0));
        }
        let f = num::FromPrimitive::from_u32(function);
        if f.is_none() {
            return Err(NeutronError::Recoverable(RecoverableError::InvalidSystemFunction));
        }
        let function = f.unwrap();
        let result = match function {
            FileFunctions::Available => Ok(ElementResult::Result(1)),
            FileFunctions::ReadFile => {
                let data = codata.pop_input_stack()?;
                let filename = match std::str::from_utf8(&data) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("file: {}", filename);
                let contents = std::fs::read(filename).unwrap();
                codata.push_output_stack(&contents)?;
                Ok(ElementResult::Result(contents.len() as u64))
            }
            FileFunctions::FileExists => {
                let data = codata.pop_input_stack()?;
                let filename = match std::str::from_utf8(&data) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                if std::path::Path::new(filename).exists() {
                    Ok(ElementResult::Result(1))
                } else {
                    Ok(ElementResult::Result(0))
                }
            }
        };
        result
    }
}
