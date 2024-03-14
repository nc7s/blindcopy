# blindcopy

Copy data to clipboard without it appearing in clipboard history or being
synced, by setting platform-specific options.

Currently, only copying text is supported, for the initial intended usage is
passwords. Support for more data types may be added.

For reading (and general writing), use another library.

Supported platforms and their options are:

- macOS, setting [`[NSPasteboard setString: forType:]`][forType] to
  [`org.nspasteboard.ConcealedType`][nspb]
- Windows, writing something of format
  [`ExcludeClipboardContentFromMonitorProcessing`][winformat] after the actual
  data
- KDE (Klipper) on Wayland, copying a source of MIME type
  `x-kde-passwordManagerHint` with value `secret`; there's no single source of
  truth for this, but can be searched at multiple places

I hope to also add support for:

- The `CLIPBOARD_STATE` protocol from wl-clipboard ([proposal][wlproposal] and
  [man page of wl-clipboard][wlcman])
- Other, more general protocols on Linux

Instructions and code to support more modern platforms and data types are
welcome.

[forType]: https://developer.apple.com/documentation/appkit/nspasteboard/1530774-setpropertylist?language=objc
[nspb]: http://nspasteboard.org
[winformat]: https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats
[wlproposal]: https://github.com/bugaevc/wl-clipboard/pull/93#issuecomment-774689076
[wlcman]: https://man.archlinux.org/man/wl-paste.1.en#CLIPBOARD_STATE

Thanks to:

- KeePassXC, for the initial [inspiration][kpxc]
- The arboard and wden crates, for directions on Windows handling
- The cacao crate, for macOS handling

[kpxc]: https://github.com/keepassxreboot/keepassxc/blob/267928d6de/src/gui/Clipboard.cpp#L54-L62
