#![cfg(target_os = "macos")]

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
	static NSPasteboardTypeString: *const Object;
}

/** Read a pasteboard string for the given type. */
unsafe fn read_pasteboard_string(type_obj: *const Object) -> Option<String> {
	#[allow(non_snake_case)]
	let NSPasteboard = class!(NSPasteboard);
	let pasteboard: Id<Object> = msg_send![NSPasteboard, generalPasteboard];
	let value: *const Object = msg_send![pasteboard, stringForType: type_obj];
	if value.is_null() {
		return None;
	}
	let ns: &NSString = &*(value as *const NSString);
	Some(ns.as_str().to_owned())
}

#[test]
#[ignore = "requires a GUI session"]
fn text_roundtrip() {
	let payload = format!("blindcopy-test-{}", std::process::id());

	blindcopy::text(&payload).unwrap();

	let got = unsafe { read_pasteboard_string(NSPasteboardTypeString) }
		.expect("clipboard should contain text");
	assert_eq!(got, payload);
}

#[test]
#[ignore = "requires a GUI session"]
fn concealment_metadata() {
	let concealed_type = "org.nspasteboard.ConcealedType";

	blindcopy::text("concealment-check").unwrap();

	let type_nsstring: Id<NSString> = NSString::from_str(concealed_type);
	let got =
		unsafe { read_pasteboard_string(&*type_nsstring as *const NSString as *const Object) }
			.expect("clipboard should contain concealed type");
	assert_eq!(got, "secret");
}
