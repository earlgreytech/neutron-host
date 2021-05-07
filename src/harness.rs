//! The harness module contains a series of methods and structures for more easily using Neutron for various testing purposes

extern crate elf;

use crate::callsystem::*;
use crate::codata::*;
use crate::db::MemoryGlobalState;
use crate::element_interfaces::debug_data::*;
use crate::element_interfaces::logging::StdoutLogger;
use crate::interface::*;
use crate::manager::*;
use crate::narm_hypervisor::*;
use crate::vmmanager::*;

use std::cell::RefCell;
use std::path::PathBuf;

pub const DEFAULT_TEST_GAS: u64 = 10000;

/// TestHarness contains a NeutronInstance and test versions of "mandatory" Elements, plus the optional DebugDataInjector Element
#[derive(Default)]
pub struct TestHarness {
    pub instance: NeutronInstance,
    pub db: MemoryGlobalState,
    pub logger: StdoutLogger,
    pub debugdata: DebugDataInjector,
}

/// Contains the execution state data needed to run Neutron which is also likely to be interacted with through test code
#[derive(Default)]
pub struct NeutronInstance {
    pub manager: Manager,
    pub codata: CoData,
}

impl NeutronInstance {
    /// Loads the given binary at `path_str` and loads it for a "use once" execution using the given CallSystem and Context
    pub fn execute_binary(
        &mut self,
        path_str: &str,
        callsystem: &CallSystem,
        mut context: ExecutionContext,
    ) -> NeutronResult {
        self.prepare_execute(path_str, &mut context);
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        let result = self
            .manager
            .execute(&mut self.codata, &callsystem, &vmm)
            .unwrap();
        NeutronInstance::print_results(&result);
        result
    }

    fn prepare_execute(&mut self, path_str: &str, context: &mut ExecutionContext) {
        let path = PathBuf::from(path_str);
        let binary = elf::File::open_path(path).unwrap();

        let text_scn = binary.get_section(".text").unwrap();
        assert!(text_scn.shdr.addr == 0x10000);

        if context.gas_limit == 0 {
            context.gas_limit = DEFAULT_TEST_GAS;
        }
        self.codata.gas_remaining = context.gas_limit;

        context.permissions = ContextPermissions::mutable_call();
        context.execution_type = ExecutionType::BareExecution;
        //self.codata.gas_remaining = MAX_GAS;

        self.codata.push_context(context.clone()).unwrap();
        self.codata
            .push_input_key("!.c".as_bytes(), &text_scn.data)
            .unwrap();
        self.codata.push_input_key("!.d".as_bytes(), &[0]).unwrap();
    }

    fn prepare_deploy(&mut self, path_str: &str, context: &mut ExecutionContext) {
        let path = PathBuf::from(path_str);
        let binary = elf::File::open_path(path).unwrap();

        let text_scn = binary.get_section(".text").unwrap();
        assert!(text_scn.shdr.addr == 0x10000);

        if context.gas_limit == 0 {
            context.gas_limit = DEFAULT_TEST_GAS;
        }
        self.codata.gas_remaining = context.gas_limit;

        context.permissions = ContextPermissions::mutable_call();
        context.execution_type = ExecutionType::Deploy;
        //self.codata.gas_remaining = MAX_GAS;

        self.codata.push_context(context.clone()).unwrap();
        self.codata
            .push_input_key("!.c".as_bytes(), &text_scn.data)
            .unwrap();
        self.codata.push_input_key("!.d".as_bytes(), &[0]).unwrap();
    }

    fn print_results(result: &NeutronResult) {
        println!("Contract executed successfully!");
        println!("Gas used: {}", result.gas_used);
        println!("Status code: {:x}", result.status);
    }
}

impl TestHarness {
    /// Uses the default test CallSystem to "use once" execute the given smart contract binary
    pub fn execute_binary_using_default_callsystem(
        &mut self,
        path_str: &str,
        mut context: ExecutionContext,
    ) -> NeutronResult {
        self.instance.prepare_execute(path_str, &mut context);
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        self.db.checkpoint().unwrap();
        let mut cs = CallSystem::default();
        cs.global_storage = Some(RefCell::new(&mut self.db));
        cs.logging = Some(RefCell::new(&mut self.logger));
        cs.add_call(DEBUG_DATA_FEATURE, &mut self.debugdata)
            .unwrap();

        let result = self
            .instance
            .manager
            .execute(&mut self.instance.codata, &cs, &vmm)
            .unwrap();
        NeutronInstance::print_results(&result);

        self.db.commit().unwrap();
        result
    }

    /// Loads the given smart contract binary and deploys it for multiple uses with the default test CallSystem
    pub fn deploy_binary_using_default_callsystem(
        &mut self,
        path_str: &str,
        mut context: ExecutionContext,
    ) -> NeutronResult {
        self.instance.prepare_deploy(path_str, &mut context);
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        self.db.checkpoint().unwrap();
        let mut cs = CallSystem::default();
        cs.global_storage = Some(RefCell::new(&mut self.db));
        cs.logging = Some(RefCell::new(&mut self.logger));
        cs.add_call(DEBUG_DATA_FEATURE, &mut self.debugdata)
            .unwrap();

        let result = self
            .instance
            .manager
            .execute(&mut self.instance.codata, &cs, &vmm)
            .unwrap();
        NeutronInstance::print_results(&result);

        self.db.commit().unwrap();
        result
    }

    /// Executes a previously deployed smart contract using the default test CallSystem
    pub fn call_using_default_callsystem(
        &mut self,
        mut context: ExecutionContext,
    ) -> NeutronResult {
        if context.gas_limit == 0 {
            context.gas_limit = DEFAULT_TEST_GAS;
        }
        self.instance.codata.gas_remaining = context.gas_limit;

        context.permissions = ContextPermissions::mutable_call();
        context.execution_type = ExecutionType::Call;
        self.instance.codata.push_context(context.clone()).unwrap();
        let mut vmm = VMManager::default();
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        vmm.vm_builders.insert(2, narm);

        self.db.checkpoint().unwrap();
        let mut cs = CallSystem::default();
        cs.global_storage = Some(RefCell::new(&mut self.db));
        cs.logging = Some(RefCell::new(&mut self.logger));
        cs.add_call(DEBUG_DATA_FEATURE, &mut self.debugdata)
            .unwrap();

        let result = self
            .instance
            .manager
            .execute(&mut self.instance.codata, &cs, &vmm)
            .unwrap();
        NeutronInstance::print_results(&result);

        self.db.commit().unwrap();
        result
    }
}
