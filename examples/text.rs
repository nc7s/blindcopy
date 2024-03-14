const SECRET: &str = "Test Secret Please Ignore";

fn main() {
	let mut buffer = String::new();
	blindcopy::text(SECRET).unwrap();
	print!("Paste back here: ");
	std::io::stdin().read_line(&mut buffer).unwrap();
	println!();
	if buffer.trim() == SECRET {
		println!("It's the same! Now check your clipboard manager.");
	} else {
		println!("It's not the same. You pasted: {buffer:?}");
	}
}
