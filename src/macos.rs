#![cfg(target_os = "macos")]

use {
	objc::{class, msg_send, runtime::Object, sel, sel_impl, MessageError},
	objc_id::{Id, ShareId},
};

/* See http://nspasteboard.org */
const TYPE_STRING: &str = "org.nspasteboard.ConcealedType";

pub fn text<S: AsRef<str>>(s: S) -> Result<(), MessageError> {
	unsafe { text_impl(s.as_ref()) }
}

/* Mostly copied from cacao::pasteboard */
#[inline]
unsafe fn text_impl(s: &str) -> Result<(), MessageError> {
	let clipboard_ptr: ShareId<Object> =
		ShareId::from_ptr(msg_send![class!(NSPasteboard), generalPasteboard]);
	let nsstring: *mut Object = msg_send![class!(NSString), alloc];
	let nss_ptr: Id<Object> = Id::from_ptr(msg_send![nsstring, initWithUTF8String: s]);
	let type_ptr: Id<Object> =
		Id::from_ptr(msg_send![class!(NSString), initWithUTF8String: TYPE_STRING]);
	let _: () = msg_send![clipboard_ptr, setString: &*nss_ptr forType: &*type_ptr];
	Ok(())
}
