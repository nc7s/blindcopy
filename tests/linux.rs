#![cfg(target_os = "linux")]

use std::io::Read;
use wl_clipboard_rs::paste;

/** Let the background data source register with the compositor. */
fn settle() {
	std::thread::sleep(std::time::Duration::from_millis(100));
}

#[test]
#[ignore = "requires a Wayland compositor"]
fn text_roundtrip() {
	let payload = format!("blindcopy-test-{}", std::process::id());

	blindcopy::text(&payload).unwrap();
	settle();

	let (mut reader, _mime) = paste::get_contents(
		paste::ClipboardType::Regular,
		paste::Seat::Unspecified,
		paste::MimeType::Text,
	)
	.unwrap();
	let mut buf = String::new();
	reader.read_to_string(&mut buf).unwrap();
	assert_eq!(buf, payload);
}

#[test]
#[ignore = "requires a Wayland compositor"]
fn concealment_metadata() {
	let kde_hint = "x-kde-passwordManagerHint";

	blindcopy::text("concealment-check").unwrap();
	settle();

	let types =
		paste::get_mime_types(paste::ClipboardType::Regular, paste::Seat::Unspecified).unwrap();
	assert!(
		types.contains(kde_hint),
		"clipboard should offer {kde_hint}, got: {types:?}"
	);

	let (mut reader, _mime) = paste::get_contents(
		paste::ClipboardType::Regular,
		paste::Seat::Unspecified,
		paste::MimeType::Specific(kde_hint),
	)
	.unwrap();
	let mut buf = Vec::new();
	reader.read_to_end(&mut buf).unwrap();
	assert_eq!(buf, b"secret");
}
