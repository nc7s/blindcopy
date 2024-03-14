#![cfg(target_os = "linux")]

use wl_clipboard_rs::copy::{copy_multi, Error, MimeSource, MimeType, Options, Source};

const MIME_KDE: &str = "x-kde-passwordManagerHint";
const MIME_KDE_VALUE: &[u8] = b"secret";

pub fn text<S: AsRef<str>>(data: S) -> Result<(), Error> {
	copy_multi(
		Options::new(),
		vec![
			MimeSource {
				mime_type: MimeType::Autodetect,
				source: Source::Bytes(data.as_ref().as_bytes().into()),
			},
			MimeSource {
				mime_type: MimeType::Specific(MIME_KDE.to_string()),
				source: Source::Bytes(MIME_KDE_VALUE.into()),
			},
		],
	)
}
