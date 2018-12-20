use std::env;
use std::process;
use fsmulator;
use fsmulator::Config;
fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(args)
		.unwrap_or_else(|err| {
			eprintln!("Error parsing arguments : {}", err);
			process::exit(1);
		});

	//println!("{:#?}", config);
	if fsmulator::run(&config).unwrap() {
		println!("Accepted");
	} else {
		println!("Rejected");
	}
}