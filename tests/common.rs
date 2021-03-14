#![allow(dead_code)] // Stuff used only in/by the macros generate warnings otherwise

extern crate elf;

use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;
use neutron_host::{db::MemoryGlobalState, element_interfaces::logging::StdoutLogger, manager::*};
use std::path::PathBuf;
use std::{cell::RefCell, env};

const MAX_GAS: u64 = 10000;

// TODO: #[cfg(test)]???

// TODO: What does the lifetime parameter actually DO?
pub struct TestSetup<'a> {
    pub manager: Manager,
    pub codata: CoData,

    pub callsystem: CallSystem<'a>,
    pub db: MemoryGlobalState, // TODO: Make private, only used in CallSystem
    pub logger: StdoutLogger,  // TODO: Make private, only used in CallSystem

    pub vmm: VMManager,
    pub context: ExecutionContext,
}

impl<'a> Default for TestSetup<'a> {
    fn default() -> Self {
        TestSetup {
            manager: Manager::default(),
            codata: CoData::new(),

            callsystem: CallSystem::default(),
            db: MemoryGlobalState::default(),
            logger: StdoutLogger {},

            vmm: VMManager::default(),
            context: ExecutionContext::default(),
        }
    }
}

// A minimalistic setup that allows executing a single smart contract
// TODO: Figure out how to do this with implemented functions? Lifetimes are harsh, man
#[macro_export]
macro_rules! initiate {
    ( $test_setup_ident:ident, $path_ident:ident ) => {
        let file = elf::File::open_path(&$path_ident).unwrap();

        let text_scn = file.get_section(".text").unwrap();
        assert!(text_scn.shdr.addr == 0x10000);

        //setup mandatory storage and logging elements
        $test_setup_ident.db.checkpoint().unwrap();
        $test_setup_ident.callsystem.global_storage = Some(RefCell::new(&mut $test_setup_ident.db));
        $test_setup_ident.callsystem.logging = Some(RefCell::new(&mut $test_setup_ident.logger));
        //todo, setup other ElementAPIs here

        //Add NARM as #2 VM
        let narm = || -> Box<dyn VMHypervisor> { Box::from(NarmHypervisor::default()) };
        $test_setup_ident.vmm.vm_builders.insert(2, narm);

        //Setup execution context
        $test_setup_ident.codata.gas_remaining = MAX_GAS;
        neutron_host::reset_to_random_address(&mut $test_setup_ident.context.self_address);
        $test_setup_ident.context.self_address.version = 2; //to match NARM VM number
        $test_setup_ident.context.execution_type = ExecutionType::BareExecution;

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
    };
}

/*
impl<'a> TestSetup<'a> {
    // TODO: Can this be done trough Default trait?
    pub fn initiate(&self) {
        self.db.checkpoint().unwrap();
        self.callsystem.global_storage=Some(RefCell::new(&mut self.db));
        self.callsystem.logging = Some(RefCell::new(&mut self.logger));
    }
}
*/
