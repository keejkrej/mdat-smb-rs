# mdat-smb-rs

Seekable read-only SMB I/O for large microscopy files (ND2, CZI, and other [mdat](https://github.com/keejkrej) drivers).

Hosts implement [`SmbSessionProvider`](src/provider.rs) (open file, read at offset, close). Drivers call [`open_path`](src/lib.rs) on virtual URLs such as `smb:{sessionId}/share/path/file.nd2` to obtain a [`SmbSeekReader`](src/reader.rs) that implements `Read + Seek` without downloading the whole file.

## Usage

```rust
use std::sync::Arc;
use mdat_smb_rs::{register_provider, open_path, SmbSessionProvider};

// Your app registers once at startup (e.g. lisca’s SMB session store).
register_provider(Arc::new(MySmbProvider))?;

let mut reader = open_path("smb:session-id/project/data.nd2")?;
// Pass `reader` to nd2-rs / czi-rs via `open_reader` / `open_smb`.
```

## Related crates

- [nd2-rs](https://github.com/keejkrej/nd2-rs) — enable feature `smb`
- [czi-rs](https://github.com/keejkrej/czi-rs) — enable feature `smb`
- [lisca](https://github.com/keejkrej/lisca) — reference `SmbSessionProvider` implementation

## License

MIT OR Apache-2.0
