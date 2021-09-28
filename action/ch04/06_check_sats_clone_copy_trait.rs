#[derive(Debug, Clone)]
struct CubeSat {
	id: u64
}

#[derive(Debug)]
enum StatusMessage {
	Ok
}

fn check_status(_sat_id: CubeSat) -> StatusMessage {
	StatusMessage::Ok
}

fn main() {
	let sat_a = CubeSat { id: 10 };
	
	let a_status = check_status(sat_a.clone());
	println!("a: {:?}", a_status);

	let a_status = check_status(sat_a);
	println!("a: {:?}", a_status);
}