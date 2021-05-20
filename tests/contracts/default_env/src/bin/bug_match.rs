//! Directly plug a `main` symbol instead of using `#[entry]`

#![deny(warnings)]
#![no_main]
#![no_std]

use neutron_star_rt::*;
use neutron_star::*;
//use neutron_star::syscalls::*;
extern crate panic_halt;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    
    
    for i in 0..=5 {
        
        match i {
            1 => println!("i = {}, match = 1", i),
            2 => println!("i = {}, match = 2", i),
            3 => println!("i = {}, match = 3", i),
            4 => println!("i = {}, match = 4", i),
            _ => println!("ERROR: {} IS OUT OF BOUNDS!!!!!!", i)
        }
        
        /*
        match i {
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            _ => {}
        }
        */
        /*
        match i {
            1 => {},
            _ => {}
        }
        */
    }
    
    /*
    match i {
        1 => {
            println!("i = {}, match = 1", i);
            __exit(1);
        },
        2 => {
            println!("i = {}, match = 2", i);
            __exit(2);
        },
        3 => {
            println!("i = {}, match = 3", i);
            __exit(3);
        },
        4 => {
            println!("i = {}, match = 4", i);
            __exit(4);
        },
        _ => {
            println!("i = {}, match = NONE!!!", i);
            __exit(42);
        }
    }
    */
    
    /*
    let i = 17;
    
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        6 => println!("6"),
        7 => println!("7"),
        8 => println!("8"),
        9 => println!("9"),
        10 => println!("10"),
        11 => println!("11"),
        12 => println!("12"),
        13 => println!("13"),
        14 => println!("14"),
        15 => println!("15"),
        16 => println!("16"),
        17 => println!("17"),
        18 => println!("18"),
        19 => println!("19"),
        20 => println!("20"),
        21 => println!("21"),
        22 => println!("22"),
        23 => println!("23"),
        _ => println!("42")
    }
    */
    __exit(5);
}
