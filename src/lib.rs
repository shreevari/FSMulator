use std::env;
use std::collections::HashMap;

#[derive(Debug)]
enum OptKind {
	Short,
	Long,
}

#[derive(Debug)]
enum Opt {
	Param { name: String, kind: OptKind, value: String },
	Flag { name: String, kind: OptKind, value: bool }, 
}

#[derive(Debug)]
struct Args<'a> {
	raw_args: Vec<String>,
	options: Vec<Opt>,
}

impl Opt {
	fn new (name: String, kind: OptKind, value: Option<>) -> Result<Opt, &'static str> {

	}
}

impl Args {

	fn new(args: Vec<String>, options: Vec<Opt>) -> Result<Args, &'static str> {
		
		let raw_args: Vec<String> = Vec::new();
		let mut items = args.iter().enumerate();
		
		raw_args.push(items.next().1.clone());

		for (i, item) in items {
			match item {
				long if long.starts_with("--") => {
					let option = raw_args.remove(i);
					find_option(option, options);
				},
				short if short.starts_with("-") => {

				},
				_ => {
					raw_args.push(item);
				},
			}
		}

	}
}

#[derive(Debug)]
struct Config {
	input: String,
	start_state: Opt,
	final_states: Opt,
	transition_table: Opt,
	interactive: bool,
}

impl Config {
	fn new (args: Vec<String>) -> Config {
		let input = String::new();
		let start_state = String::new();
		let final_states;
		let transition_table;
		let interactive = false;

		let mut items = args.iter().enumerate().skip(1);
		for (i, item) in items {
			match item {
				"-i" => {
					interactive = true;
				},
				"-f" => {
					let filename  = args[i+1];

				}
			}
		}
	}
}