#![cfg(target_os = "windows")]

use clipboard_win::{raw, register_format, Clipboard, Setter, SysResult, Unicode};

const EXCLUDE_FORMAT: &str = "ExcludeClipboardContentFromMonitorProcessing";
const ATTEMPTS: usize = 5;
const WAIT: core::time::Duration = core::time::Duration::from_millis(5);

pub fn text<S: AsRef<str>>(data: S) -> SysResult<()> {
	/* Prior art:
	 * https://docs.rs/arboard/3.2.2/x86_64-pc-windows-msvc/src/arboard/platform/windows.rs.html#441-448
	 */
	let mut attempts = ATTEMPTS;
	let _clipboard = loop {
		match Clipboard::new() {
			Ok(v) => break v,
			Err(e) => match attempts {
				0 => return Err(e),
				_ => {
					std::thread::sleep(WAIT);
					attempts -= 1;
				}
			},
		}
	};

	Unicode.write_clipboard(&data)?;

	/* See:
	 * https://github.com/luryus/wden/blob/600f9637/wden/src/ui/clipboard/windows_clipboard.rs#L17-22
	 *
	 * Primarily the use of set_without_clear, since set would clear the data
	 * set on last call (obviously).
	 *
	 * NOTE(EXCL-FORMAT): Most seem to let it pass if the format is absent.
	 * Should we pass the result on?
	 */
	if let Some(format) = register_format(EXCLUDE_FORMAT) {
		raw::set_without_clear(format.get(), b"1")?;
	}

	Ok(())
}
