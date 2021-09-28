use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
	radio_freq: u8
}

fn main() {
	let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation {
		radio_freq: 5
	}));
	println!("base: {:?}", base);
	{
		let mut base_2 = base.borrow_mut();
		base_2.radio_freq -= 1;
		println!("base 2: {:?}", base_2);
	}

	println!("base: {:?}", base);

	let mut base_3 = base.borrow_mut();
	base_3.radio_freq += 5;

	println!("base: {:?}", base);

	println!("base 3: {:?}", base_3);
}