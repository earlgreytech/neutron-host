extern crate elf;

// These will throw lots of unused import warnings because some are only used in macros
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::db::MemoryGlobalState;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::element_interfaces::logging::StdoutLogger;
use neutron_host::interface::*;
use neutron_host::manager::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;

use std::{cell::RefCell, char::MAX};
use std::path::PathBuf;

pub const MAX_GAS: u64 = 10000;

/*
Harness for Neutron stack integration testing

Currently very basic functionality:
* Load and run a smart contract
* Output stream from execution context to the testing function
* Inject debug data into the execution context to push an initial input stack and/or assert state of output stack

*/

#[derive(Default)]
pub struct TestHarness {
    pub instance: NeutronInstance,
    pub db: MemoryGlobalState,
    pub logger: StdoutLogger,
    pub debugdata: DebugDataInjector,
}
#[derive(Default)]
pub struct NeutronInstance{
    pub manager: Manager,
    pub codata: CoData,
}

impl NeutronInstance{
    pub fn execute_binary(&mut self, path_str: &str, callsystem: &CallSystem, mut context: ExecutionContext) -> NeutronResult{
        self.prepare_execute(path_str, &mut context);
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        let result = self.manager.execute(
            &mut self.codata,
            &callsystem,
            &vmm,
            )
            .unwrap();
        NeutronInstance::print_results(&result);
        result
    }
    fn prepare_execute(&mut self, path_str: &str, context: &mut ExecutionContext){
        let path = PathBuf::from(path_str);
        let binary = elf::File::open_path(path).unwrap();

        let text_scn = binary.get_section(".text").unwrap();
        assert!(text_scn.shdr.addr == 0x10000);

        if context.gas_limit == 0{
            context.gas_limit = MAX_GAS;
        }
        self.codata.gas_remaining = context.gas_limit;
        
        context.permissions = ContextPermissions::mutable_call();
        context.execution_type = ExecutionType::BareExecution;
        //self.codata.gas_remaining = MAX_GAS;

        self
            .codata
            .push_context(context.clone())
            .unwrap();
        self
            .codata
            .push_input_key("!.c".as_bytes(), &text_scn.data)
            .unwrap();
        self
            .codata
            .push_input_key("!.d".as_bytes(), &[0])
            .unwrap();
    }
    fn print_results(result: &NeutronResult){
        println!("Contract executed successfully!");
        println!("Gas used: {}", result.gas_used);
        println!("Status code: {:x}", result.status); 
    }
}

impl TestHarness {
    // Neater function based on the default folder setup
    pub fn execute_debug_path_binary_using_default_callsystem(&mut self, test_dir: &str, contract_dir: &str, context: ExecutionContext) -> NeutronResult {
        self.execute_binary_using_default_callsystem(
            &TestHarness::get_debug_binary_path(test_dir, contract_dir),
            context
        )
    }

    pub fn get_debug_binary_path(test_dir: &str, contract_dir: &str) -> String{
        let path_str = &format!(
            "./tests/{}/{}/target/thumbv6m-none-eabi/debug/contract-binary",
            test_dir, contract_dir
        );
        path_str.to_string()
    }



    pub fn execute_binary_using_default_callsystem(&mut self, path_str: &str, mut context: ExecutionContext) -> NeutronResult{
        self.instance.prepare_execute(path_str, &mut context);
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        self.db.checkpoint().unwrap();
        let mut cs = CallSystem::default();
        cs.global_storage = Some(RefCell::new(&mut self.db));
        cs.logging = Some(RefCell::new(&mut self.logger));
        cs.add_call(DEBUG_DATA_FEATURE, &mut self.debugdata).unwrap();

        let result = self.instance.manager.execute(
            &mut self.instance.codata,
            &cs,
            &vmm,
            )
            .unwrap();
        NeutronInstance::print_results(&result);

        self.db.commit().unwrap();
        result
    }

}
