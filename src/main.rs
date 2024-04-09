use clap::Parser;
use lazy_static::lazy_static;
use resource_simulation::mem::mem;
use resource_simulation::{Options, Simulation};
use sysinfo::System;

lazy_static! {
    static ref SYS_INFO: System = System::new_all();
}
fn main() {
    let cmd = Simulation::parse();
    match &cmd.sub_command {
        Options::Memory(opts) => {
            mem::process(opts, cmd.refresh);
        }
        Options::CPU => {}
        _ => {}
    }

    println!("{:#?}", &cmd);
}
