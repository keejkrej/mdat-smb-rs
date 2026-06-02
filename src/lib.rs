//! Seekable read-only SMB I/O for large microscopy files (ND2/CZI).
//!
//! ## Session contract
//!
//! - Connect with [`parse_smb_url`] (`//host/share/...` or `smb://host/share/...`).
//! - Address files as virtual paths via [`format_smb_path`] / [`parse_smb_path`]
//!   (`smb:{sessionId}/relative/path.nd2`).
//! - Register a host [`SmbSessionProvider`], then [`open_path`] for seekable reads.

mod path;
mod provider;
mod reader;
mod url;

pub use path::{format_smb_path, is_smb_path, parse_smb_path, ParsedSmbPath};
pub use url::{parse_smb_url, ParsedSmbUrl};
pub use provider::{register_provider, provider, SmbHandle, SmbSessionProvider};
pub use reader::SmbSeekReader;

/// Open a seekable reader for a virtual `smb:{sessionId}/relative/path` URL.
pub fn open_path(path: &str) -> Result<SmbSeekReader, String> {
    SmbSeekReader::open_path(path)
}
