//! Transport abstraction for ECU links.
//!
//! Two backends implement this trait:
//! - [`crate::serial::EcuConnection`] - VCP serial via the `serialport` crate
//! - [`crate::ecu::ftdi::FtdiKline`] (feature `ftdi`) - direct FTDI access via
//!   libftdi1, mirroring eculib's `pylibftdi` usage (bitbang Fast Init).

/// Common operations every ECU link must support.
pub trait EcuTransport {
    /// Human-readable endpoint name (COM port or FTDI serial).
    fn endpoint(&self) -> &str;
    /// K-Line Fast Init handshake (TX low 70ms, release, wait 130ms).
    fn fast_init(&mut self) -> Result<(), String>;
    /// Read up to `buf.len()` bytes.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
    /// Write all bytes.
    fn write(&mut self, data: &[u8]) -> Result<usize, String>;
}
