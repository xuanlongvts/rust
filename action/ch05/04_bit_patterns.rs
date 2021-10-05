fn main() {
	let zero: u16 = 0b0000_0000_0000_0000;
	let one: u16 = 0b_0000_0000_0000_0001;
	let two: u16 = 0b_0000_0000_0000_0010;

	let sixfivethousand_533: u16 = 0b1111_1111_1111_1101;
	let sixfivethousand_534: u16 = 0b1111_1111_1111_1110;
	let sixfivethousand_535: u16 = 0b1111_1111_1111_1111;

	print!("{}, {}, {}, ..., ", zero, one, two);
  	println!("{}, {}, {}", sixfivethousand_533, sixfivethousand_534, sixfivethousand_535);
}