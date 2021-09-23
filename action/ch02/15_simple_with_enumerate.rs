fn main() {
	let search_term = "picture";
	let quote = "\
	Every face, every shop, bedroom window, public-house, and
	dark square is a picture feverishly turned--in search of what?
	It is the same with books. What do we seek through millions of pages?
	";

	for (i, v) in quote.lines().enumerate() {
		if v.contains(search_term) {
			let at_line = i + 1;
			println!("Line {}: {}", at_line, v);
		}
	}
}