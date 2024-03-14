#![cfg(target_os = "macos")]

use {
	objc::{class, msg_send, runtime::Object, sel, sel_impl, MessageError},
	objc_id::Id,
	objc_foundation::{INSString, NSString},
};

/* https://docs.rs/arboard/3.3.2/x86_64-apple-darwin/src/arboard/platform/osx.rs.html#31-38
 */
#[link(name = "AppKit", kind = "framework")]
extern "C" {
	static NSPasteboardTypeString: *const Object;
}

/* http://nspasteboard.org */
const TYPE_STRING: &str = "org.nspasteboard.ConcealedType";

pub fn text<S: AsRef<str>>(s: S) -> Result<(), MessageError> {
	unsafe { text_impl(s.as_ref()) }
}

#[inline]
unsafe fn text_impl(s: &str) -> Result<(), MessageError> {
	#[allow(non_snake_case)]
	let NSPasteboard = class!(NSPasteboard);
	let pasteboard: Id<Object> = msg_send![NSPasteboard, generalPasteboard];
	let nss_s: Id<NSString> = NSString::from_str(s);
	let nss_type: Id<NSString> = NSString::from_str(TYPE_STRING);

	let _: () = msg_send![pasteboard, clearContents];
	/* https://github.com/roosto/pbsecret/blob/9e91917de0/pbsecret.m#L100-L101
	 *
	 * Probably should be done with writeObjects but this is easier.
	 */
	let _: () = msg_send![pasteboard, setString: NSString::from_str("secret") forType: nss_type];
	let _: () = msg_send![pasteboard, setString: nss_s forType: NSPasteboardTypeString];
	Ok(())
}
