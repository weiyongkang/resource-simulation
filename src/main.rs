use clap::Parser;
use lazy_static::lazy_static;
use resource_simulation::cmd::{Options, Simulation};
use resource_simulation::io;
use resource_simulation::mem::mem;
use sysinfo::System;

lazy_static! {
	static ref SYS_INFO: System = System::new_all();
}
fn main() {
	let cmd = Simulation::parse();
	match &cmd.sub_command {
		Options::Memory(opts) => {
			mem::process(opts, cmd.refresh);
		},
		Options::IO(opts) => {
			let _ = io::process(opts, cmd.refresh);
		},
		Options::File(opts) => {
			let _ = io::process_file(opts);
		},
		Options::CPU => {},
		_ => {},
	}

	println!("{:#?}", &cmd);
}
