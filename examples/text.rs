use std::time::Instant;

const SECRET: &str = "Test Secret Please Ignore";

fn main() {
	let start = Instant::now();
	let mut rand = 0;
	for _ in 0..1024 {
		rand += start.elapsed().as_micros() % 1024;
	}

	let mut buffer = String::new();

	let secret = format!("{SECRET}: {}", start.elapsed().as_micros() + rand);

	blindcopy::text(&secret).unwrap();
	println!("Your super secret string is: {secret:?}");
	println!("It should have been copied. Paste back below to verify: ");
	std::io::stdin().read_line(&mut buffer).unwrap();
	buffer = buffer.trim().to_string();
	if buffer == secret {
		println!("It's the same! Now check your clipboard manager.");
	} else {
		println!("It's not the same. You pasted: {buffer:?}");
	}
}
