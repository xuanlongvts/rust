fn main() {
	let penguin_data ="\
	common name,length (cm) 
	Little penguin,33
	Yellow-eyed penguin,65
	Fiordland penguin,60
	Invalid,data
	";

	let records = penguin_data.lines();

	for (i, v) in records.enumerate() {
		if i == 0 || v.trim().len() == 0 {
			continue;
		}

		let fields: Vec<_> = v.split(',').map(|field| field.trim()).collect();

		if cfg!(debug_assertions) {
			eprintln!("debug: record: {:?} ---> fields: {:?}", v, fields);
		}

		let name = fields[0];
		if let Ok(length) = fields[1].parse::<f32>() {
			println!("{}        | {}cm", name, length);
		}
	}
}

// cargo run -q --release will ignore debug
// cargo run --release will ignore debug