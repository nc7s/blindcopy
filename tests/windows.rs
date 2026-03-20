#![cfg(target_os = "windows")]

use clipboard_win::{raw, register_format, Clipboard, Getter, Unicode};

#[test]
#[ignore = "requires a desktop session"]
fn text_roundtrip() {
	let payload = format!("blindcopy-test-{}", std::process::id());

	blindcopy::text(&payload).unwrap();

	let _clipboard = Clipboard::new().unwrap();
	let mut buf = String::new();
	Unicode.read_clipboard(&mut buf).unwrap();
	assert_eq!(buf, payload);
}

#[test]
#[ignore = "requires a desktop session"]
fn concealment_metadata() {
	let exclude_format = "ExcludeClipboardContentFromMonitorProcessing";

	blindcopy::text("concealment-check").unwrap();

	let format = register_format(exclude_format).expect("format should be registered");
	assert!(
		raw::is_format_avail(format.get()),
		"clipboard should contain {exclude_format}"
	);
}
