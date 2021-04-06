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

use std::cell::RefCell;
use std::env;
use std::path::PathBuf;

pub const MAX_GAS: u64 = 10000;

/*
Harness for Neutron stack integration testing

Currently very basic functionality:
* Load and run a smart contract
* Output stream from execution context to the testing function
* Inject debug data into the execution context to push an initial input stack and/or assert state of output stack

*/

pub struct TestHarness<'a> {
    pub manager: Manager,
    pub codata: CoData,

    pub callsystem: CallSystem<'a>,
    pub db: MemoryGlobalState,
    pub logger: StdoutLogger,
    pub debugdata: DebugDataInjector,

    pub vmm: VMManager,
    pub context: ExecutionContext,

    pub contract: Option<elf::File>,
}

impl<'a> Default for TestHarness<'a> {
    fn default() -> Self {
        TestHarness {
            manager: Manager::default(),
            codata: CoData::new(),

            callsystem: CallSystem::default(),
            db: MemoryGlobalState::default(),
            logger: StdoutLogger {},
            debugdata: DebugDataInjector::default(),

            vmm: VMManager::default(),
            context: ExecutionContext::default(),

            contract: None,
        }
    }
}

impl<'a> TestHarness<'a> {
    // Currently only supports a single contract binary
    pub fn load_contract_binary(&mut self, path_str: &str) {
        let path = PathBuf::from(path_str);
        self.contract = Some(elf::File::open_path(path).unwrap());
    }

    // Neater function based on the default folder setup
    pub fn load_contract_binary_default_path(&mut self, test_dir: &str, contract_dir: &str) {
        let path_str = &format!(
            "./tests/{}/{}/target/thumbv6m-none-eabi/debug/contract-binary",
            test_dir, contract_dir
        );
        self.load_contract_binary(path_str);
    }
}

// Does all initialization that passes references around (aka lifetime hell) runs the VM, all in one swoop
// Splitting this up into neat functions is a later concern
#[macro_export]
macro_rules! initiateAndRun {
    ( $test_setup_ident:ident ) => {
        //setup mandatory storage and logging elements
        $test_setup_ident.db.checkpoint().unwrap();
        $test_setup_ident.callsystem.global_storage = Some(RefCell::new(&mut $test_setup_ident.db));
        $test_setup_ident.callsystem.logging = Some(RefCell::new(&mut $test_setup_ident.logger));
        $test_setup_ident
            .callsystem
            .add_call(0x8000_0001, &mut $test_setup_ident.debugdata)
            .unwrap();
        //todo, setup other ElementAPIs here

        //Add NARM as #2 VM
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        $test_setup_ident.vmm.vm_builders.insert(2, narm);

        //Setup execution context
        $test_setup_ident.codata.gas_remaining = MAX_GAS;
        neutron_host::reset_to_random_address(&mut $test_setup_ident.context.self_address);
        $test_setup_ident.context.self_address.version = 2; //to match NARM VM number
        $test_setup_ident.context.execution_type = ExecutionType::BareExecution;

        let contract = $test_setup_ident.contract.unwrap();
        let text_scn = contract.get_section(".text").unwrap();
        assert!(text_scn.shdr.addr == 0x10000);

        //Push contract bytecode into Neutron from ELF file
        $test_setup_ident
            .codata
            .push_context($test_setup_ident.context)
            .unwrap();
        $test_setup_ident
            .codata
            .push_input_key("!.c".as_bytes(), &text_scn.data)
            .unwrap();
        $test_setup_ident
            .codata
            .push_input_key("!.d".as_bytes(), &[0])
            .unwrap();

        println!("Beginning contract execution");
        let result = $test_setup_ident
            .manager
            .execute(
                &mut $test_setup_ident.codata,
                &$test_setup_ident.callsystem,
                &$test_setup_ident.vmm,
            )
            .unwrap();

        println!("Contract executed successfully!");
        println!("Gas used: {}", result.gas_used);
        println!("Status code: {:x}", result.status);
    };
}
