#[derive(Debug)]
struct File {
	name: String,
	data: Vec<u8>
}

fn main() {
	let f1 = File {
		name: String::from("f1.txt"),
		data: Vec::new()
	};

	let f1_name = &f1.name;
	let f1_length = &f1.data.len();
	
	println!("{:?}", f1);
	println!("{} is {} bytes long", f1_name, f1_length);

	println!("==========================");
	let f2 = File {
		name: String::from("f2.txt"),
		data: vec![114, 117, 115, 116, 33]
	};
	let f2_name = &f2.name;
	let f2_length = &f2.data.len();
	println!("{:?}", f2);
	println!("{} is {} bytes long", f2_name, f2_length);
}