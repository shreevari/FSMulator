extern crate csv;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Config {
	input: String,
	states: Vec<u32>,
	inputs: Vec<char>,
	transition_table: Vec<Vec<u32>>,
	start_state: u32,
	final_states: Vec<u32>,
}

impl Config {
	pub fn new (args: Vec<String>) -> Result<Config, Box<Error>> {

		let input = args[1].to_string();
		let mut inputs = Vec::new();
		let mut states = Vec::new();
		let start_state: u32 = args[2].parse()?;
		let mut final_states = Vec::new(); 
		let final_states_path = args[4].to_string();
		let mut transition_table: Vec<Vec<u32>> = Vec::new();
		let transition_table_path = args[3].to_string();
		
		//Read final_states from csv file
		let mut final_states_reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_path(final_states_path)?;
		for record in final_states_reader.records() {
			for final_state in record?.iter() {
				final_states.push(final_state.trim().parse()?);
			}
		}
		//println!("final_states : {:?}", final_states);

		let mut transition_table_reader = csv::ReaderBuilder::new()
			.has_headers(true)
			.from_path(transition_table_path)?;

		//Read inputs from transition_table csv file
		for field in transition_table_reader.headers()?.iter().skip(1) {
			let input: char = field.trim().parse()?;
			inputs.push(input)
		}
		//println!("inputs : {:?}", inputs);

		//Read states and transition_table from transition_table csv file
		for record in transition_table_reader.records() {
			let mut row = Vec::new();
			for delta in record?.iter() {
				row.push(delta.trim().parse()?)
			}
			states.push(row.remove(0));
			transition_table.push(row.clone());
		}

		//println!("states : {:?}", states);
		//println!("transition_table : {:?}", transition_table);

		Ok(Config{ input,states, inputs, transition_table, start_state, final_states })
	}
}

pub fn run(config: &Config) -> Result<bool, Box<Error>> {
	let mut present_state: &u32 = &config.start_state;
	for symbol in config.input.chars() {
		let input_index = find(symbol, &config.inputs).unwrap_or_else(|| { panic!("Wrong inputs") });
		let state_index = find(*present_state, &config.states).unwrap_or_else(|| { panic!("Wrong states"); });
		present_state = config.transition_table.get(state_index).unwrap().get(input_index).unwrap();
	}
	Ok(config.final_states.contains(present_state))
}

fn find<T: PartialEq+Debug>(key: T, vector: &Vec<T>) -> Option<usize> {
	for (i, element) in vector.iter().enumerate() {

		if key == *element {
			return Some(i);
		}
	}
	None
}