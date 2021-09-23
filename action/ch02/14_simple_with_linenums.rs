fn main() {
	let search_term = "picture";
	let quote = "\
	Every face, every shop, bedroom window, public-house, and
	5 dark square is a picture feverishly turned--in search of what?
	6 It is the same with books. What do we seek through millions of pages?
	";

	let mut count: usize = 1;
	for item in quote.lines() {
		if item.contains(search_term) {
			println!("Line {}: {}", count, item);
		}
		count += 1;
	}
}