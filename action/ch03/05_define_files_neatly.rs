#[derive(Debug)]
struct File {
	name: String,
	data: Vec<u8>
}

impl File {
	fn new(name: &str) -> File {
		File {
			name: String::from(name),
			data: Vec::new()
		}
	}

	fn new_with_data(name: &str, data: &Vec<u8>) -> File {
		let mut f = File::new(name);
		f.data = data.clone();
		f
	}

	fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
		let mut tmp = self.data.clone();
		let read_length = tmp.len();
		save_to.reserve(read_length);
		save_to.append(&mut tmp);

		read_length
	}
}

fn open(_f: &mut File) -> bool {
	true
}

fn close(_f: &mut File) -> bool {
	return true;
}

fn main() {
	let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
	let mut f = File::new_with_data("file1.txt", &f_data);

	let mut buffer: Vec<u8> = vec![];
	open(&mut f);
	let l_length = f.read(&mut buffer);
	close(&mut f);

	let text_transfrom_vec = String::from_utf8_lossy(&buffer);
	println!("{:?}", f);
	println!("{} is {} bytes long", &f.name, l_length);
	println!("{}", text_transfrom_vec);
}