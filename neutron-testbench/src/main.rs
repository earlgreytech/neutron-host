extern crate elf;

use std::{cell::RefCell, env};
use std::path::PathBuf;
use neutron_host::{db::MemoryGlobalState, element_interfaces::logging::StdoutLogger, manager::*};
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::interface::*;
use neutron_host::narm_hypervisor::*;
use neutron_testbench::test_interface::*;
use neutron_host::vmmanager::*;

const MAX_GAS:u64 = 10000000;

fn main() {
    let args: Vec<String> = env::args().collect();

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
    context.gas_limit = 1000000;
    neutron_host::reset_to_random_address(&mut context.self_address);
    context.self_address.version = 2; //to match NARM VM number
    context.execution_type = ExecutionType::BareExecution;

    //Push contract bytecode into Neutron from ELF file
    codata.push_context(context).unwrap();
    codata.element_push_key("!.c".as_bytes(), &text_scn.data).unwrap();
    codata.element_push_key("!.d".as_bytes(), &[0]).unwrap();

    manager.execute(&mut codata, &callsystem, &vmm).unwrap();

/*
    let data_scn = file.get_section(".data").unwrap();
    assert!(data_scn.shdr.addr == 0x80020000);

    let mut api = TestbenchAPI::default();
    setup_api(&mut api, &text_scn.data, &data_scn.data);
    let mut vm:VM = VM::default();
    vm.charger = GasCharger::test_schedule();
    let mut hypervisor = NeutronHypervisor{context: api.get_context(), api: Box::new(api.clone)};
    hypervisor.init_cpu(&mut vm).unwrap();
    hypervisor.create_contract_from_sccs(&mut vm).unwrap();
    let x = vm.execute(&mut hypervisor);
    vm.print_diagnostics();
    println!("Used gas: {}", MAX_GAS - vm.gas_remaining);
    x.unwrap();
    */
}

    /*
fn setup_api(api: &mut TestbenchAPI, code: &Vec<u8>, data: &Vec<u8>){
    api.push_sccs(&vec![]).unwrap(); //extra data
    api.push_sccs(data).unwrap();
    api.push_sccs(&vec![1]).unwrap(); //data section count
    api.push_sccs(code).unwrap();
    api.push_sccs(&vec![1]).unwrap(); //code section count
    api.push_sccs(&vec![0, 0, 0, 0]).unwrap(); //vmversion (fill in properly later)
    api.context.exec.gas_limit = MAX_GAS;
}
    */

 