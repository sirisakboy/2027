//! Direct FTDI backend via libftdi1 (feature `ftdi`).
//!
//! Mirrors eculib's `pylibftdi` usage in `base.py`: open by USB serial,
//! 10400 baud 8N1, and bitbang-driven K-Line Fast Init. Needs the system
//! library (`libftdi1-dev` to build, `libftdi1-2` at runtime) plus udev
//! rules / `ftdi_sio` detached for the adapter. Only compiled with
//! `--features ftdi` so default builds stay dependency-free.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::time::Duration;

use super::transport::EcuTransport;

// --- libftdi1 C ABI (stable subset used by eculib) ---------------------------

#[repr(C)]
struct FtdiContext {
    _private: [u8; 0],
}

const BITMODE_RESET: c_uchar = 0x00;
const BITMODE_BITBANG: c_uchar = 0x01;
const BITS_8: c_int = 8;
const STOP_BIT_1: c_int = 0;
const PARITY_NONE: c_int = 0;

#[link(name = "ftdi1")]
extern "C" {
    fn ftdi_new() -> *mut FtdiContext;
    fn ftdi_free(ftdi: *mut FtdiContext);
    fn ftdi_usb_open_string(ftdi: *mut FtdiContext, description: *const c_char) -> c_int;
    fn ftdi_usb_close(ftdi: *mut FtdiContext) -> c_int;
    fn ftdi_usb_reset(ftdi: *mut FtdiContext) -> c_int;
    fn ftdi_usb_purge_buffers(ftdi: *mut FtdiContext) -> c_int;
    fn ftdi_set_baudrate(ftdi: *mut FtdiContext, baudrate: c_int) -> c_int;
    fn ftdi_set_line_property(
        ftdi: *mut FtdiContext,
        bits: c_int,
        sbit: c_int,
        parity: c_int,
    ) -> c_int;
    fn ftdi_set_bitmode(ftdi: *mut FtdiContext, bitmask: c_uchar, mode: c_uchar) -> c_int;
    fn ftdi_write_data(ftdi: *mut FtdiContext, buf: *const c_uchar, size: c_int) -> c_int;
    fn ftdi_read_data(ftdi: *mut FtdiContext, buf: *mut c_uchar, size: c_int) -> c_int;
    fn ftdi_get_error_string(ftdi: *mut FtdiContext) -> *const c_char;
}

// --- safe wrapper ------------------------------------------------------------

/// FTDI adapter opened by USB serial string (e.g. `"FT123456"`).
pub struct FtdiKline {
    ctx: *mut FtdiContext,
    serial: String,
}

impl FtdiKline {
    /// Open adapter `s:<serial>`, reset, set 8N1 + baudrate, purge buffers.
    /// Mirrors `KlineAdapter.__init__` in eculib/base.py.
    pub fn open(serial: &str, baudrate: u32) -> Result<Self, String> {
        let ctx = unsafe { ftdi_new() };
        if ctx.is_null() {
            return Err("ftdi_new ล้มเหลว (หน่วยความจำ?)".to_string());
        }
        let dev = Self { ctx, serial: serial.to_string() };
        // pylibftdi Device(device_id, ...) opens by serial string.
        let desc = CString::new(format!("s:{}", serial)).map_err(|e| e.to_string())?;
        dev.check(unsafe { ftdi_usb_open_string(dev.ctx, desc.as_ptr()) }, "เปิด FTDI")?;
        dev.check(unsafe { ftdi_usb_reset(dev.ctx) }, "reset")?;
        dev.check(
            unsafe { ftdi_set_line_property(dev.ctx, BITS_8, STOP_BIT_1, PARITY_NONE) },
            "ตั้ง 8N1",
        )?;
        dev.check(
            unsafe { ftdi_set_baudrate(dev.ctx, baudrate as c_int) },
            "ตั้ง baudrate",
        )?;
        dev.check(unsafe { ftdi_usb_purge_buffers(dev.ctx) }, "purge")?;
        Ok(dev)
    }

    fn check(&self, rc: c_int, op: &str) -> Result<(), String> {
        if rc < 0 {
            let msg = unsafe {
                let p = ftdi_get_error_string(self.ctx);
                if p.is_null() {
                    format!("{} ล้มเหลว (rc={})", op, rc)
                } else {
                    format!("{} ล้มเหลว: {}", op, CStr::from_ptr(p).to_string_lossy())
                }
            };
            return Err(msg);
        }
        Ok(())
    }

    fn write_all(&mut self, data: &[u8]) -> Result<usize, String> {
        let mut done = 0;
        let mut tries = 0;
        while done < data.len() && tries < 1000 {
            let rc = unsafe {
                ftdi_write_data(
                    self.ctx,
                    data[done..].as_ptr(),
                    (data.len() - done) as c_int,
                )
            };
            if rc < 0 {
                self.check(rc, "เขียน")?;
            }
            if rc == 0 {
                tries += 1;
                continue;
            }
            done += rc as usize;
        }
        if done < data.len() {
            return Err("เขียนไม่ครบ (timeout)".to_string());
        }
        Ok(done)
    }

    fn read_into(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let mut done = 0;
        let mut tries = 0;
        while done < buf.len() && tries < 2000 {
            let rc = unsafe {
                ftdi_read_data(self.ctx, buf[done..].as_mut_ptr(), (buf.len() - done) as c_int)
            };
            if rc < 0 {
                self.check(rc, "อ่าน")?;
            }
            if rc == 0 {
                tries += 1;
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            done += rc as usize;
        }
        Ok(done)
    }

    /// K-Line presence probe. Mirrors `KlineAdapter.kline()` in base.py:
    /// bitbang, TX 0x00, 2ms, expect echo 0x00 back.
    pub fn kline_present(&mut self) -> Result<bool, String> {
        self.check(unsafe { ftdi_set_bitmode(self.ctx, 1, BITMODE_RESET) }, "bitmode probe")?;
        self.write_all(&[0x00])?;
        std::thread::sleep(Duration::from_millis(2));
        let mut b = [0u8; 1];
        let n = self.read_into(&mut b)?;
        self.check(unsafe { ftdi_set_bitmode(self.ctx, 1, BITMODE_RESET) }, "bitmode restore")?;
        self.check(unsafe { ftdi_set_bitmode(self.ctx, 0, BITMODE_RESET) }, "uart restore")?;
        Ok(n == 1 && b[0] == 0x00)
    }
}

impl Drop for FtdiKline {
    fn drop(&mut self) {
        unsafe {
            ftdi_usb_close(self.ctx);
            ftdi_free(self.ctx);
        }
    }
}

impl EcuTransport for FtdiKline {
    fn endpoint(&self) -> &str {
        &self.serial
    }

    /// Fast Init exactly like `HondaECU.init`: bitbang TX 0x00 for 70ms,
    /// TX 0x01, back to UART, flush, wait 130ms.
    fn fast_init(&mut self) -> Result<(), String> {
        self.check(unsafe { ftdi_set_bitmode(self.ctx, 1, BITMODE_BITBANG) }, "bitmode")?;
        self.write_all(&[0x00])?;
        std::thread::sleep(Duration::from_millis(70));
        self.write_all(&[0x01])?;
        self.check(unsafe { ftdi_set_bitmode(self.ctx, 0, BITMODE_RESET) }, "uart restore")?;
        self.check(unsafe { ftdi_usb_purge_buffers(self.ctx) }, "flush")?;
        std::thread::sleep(Duration::from_millis(130));
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        self.read_into(buf)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        self.write_all(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_missing_device_fails_cleanly() {
        // No hardware needed: bogus serial must return Err, not crash.
        assert!(FtdiKline::open("NO-SUCH-FTDI-DEVICE", 10400).is_err());
    }
}
