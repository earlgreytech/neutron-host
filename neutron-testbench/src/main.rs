extern crate elf;

use std::{cell::RefCell, env};
use std::path::PathBuf;
use neutron_host::{db::MemoryGlobalState, element_interfaces::logging::StdoutLogger, manager::*};
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;

const MAX_GAS:u64 = 10000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("Syntax: neutron-testbench smart-contract-file");
        println!("Expected smart contract file is an ARM architecture executable compiled as an ELF file");
        return;
    }
    let path = PathBuf::from(&args[1]);
    let file = elf::File::open_path(&path).unwrap();

    let text_scn = file.get_section(".text").unwrap();
    assert!(text_scn.shdr.addr == 0x10000);

    //setup Neutron
    let mut manager = Manager::default();
    let mut codata = CoData::new();
    let mut callsystem = CallSystem::default();

    let mut db = MemoryGlobalState::default();
    let mut logger = StdoutLogger{};

    //setup mandatory storage and logging elements
    db.checkpoint().unwrap();
    callsystem.global_storage=Some(RefCell::new(&mut db));
    callsystem.logging = Some(RefCell::new(&mut logger));
    //todo, setup other ElementAPIs here

    //Add NARM as #2 VM
    let narm = || -> Box<dyn VMHypervisor>{
        Box::from(NarmHypervisor::default())
    };
    let mut vmm = VMManager::default();
    vmm.vm_builders.insert(2, narm);

    //Setup execution context
    let mut context = ExecutionContext::default();
    codata.gas_remaining = MAX_GAS;
    neutron_host::reset_to_random_address(&mut context.self_address);
    context.self_address.version = 2; //to match NARM VM number
    context.execution_type = ExecutionType::BareExecution;

    //Push contract bytecode into Neutron from ELF file
    codata.push_context(context).unwrap();
    codata.push_input_key("!.c".as_bytes(), &text_scn.data).unwrap();
    codata.push_input_key("!.d".as_bytes(), &[0]).unwrap();

    println!("Beginning contract execution");
    let result = manager.execute(&mut codata, &callsystem, &vmm).unwrap();
    println!("Contract executed successfully!");
    println!("Gas used: {}", result.gas_used);
    println!("Status code: {:x}", result.status);
}
 