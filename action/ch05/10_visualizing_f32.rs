const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
	let n: f32 = 42.42;

	let (sign, exponent, fraction) = deconstruct_f32(n);
	println!("{} -> [sign: {:01b}, exponent: {:08b}, mantissa: {:023b}] -> tbc", n, sign, exponent, fraction);

	let (sign, exponent, mantissa) = decode_f32_parts(sign, exponent, fraction);

	let reconstituted_n = f32_from_parts(sign, exponent, mantissa);
	println!("{} -> [sign: {}, exponent: {}, mantissa: {:?}] -> {}", n, sign, exponent, mantissa, reconstituted_n);
}

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
	let bits: u32 = n.to_bits();

	let sign = (bits >> 31) & 1; // X & 1 = X
	let exponent = (bits >> 23) & 0xFF; // in 32 bit flow: 1 - 8 - 23 (sign, exponent, fraction)
	let fraction = bits & 0x7FFFFF;
	(sign, exponent, fraction)
}

fn decode_f32_parts(sign: u32, exponent: u32, frac: u32) -> (f32, f32, f32) {
	let signed = (-1.0_f32).powf(sign as f32);

	let exp = (exponent as i32) - BIAS;
	let exp = RADIX.powf(exp as f32);

	let mut mantissa: f32 = 1.0;

	for i in 0..23 {
		let mask = 1 << i;
		let one_at_bit_i = frac & mask;
		if one_at_bit_i != 0 {
			let i_ = i as f32;
			let weight = 2_f32.powf(i_ - 23.0);
			mantissa += weight;
		}
	}
	(signed, exp, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
	sign *  exponent * mantissa
}