use serialport::{SerialPort, ClearBuffer};
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use crate::ecu::transport::EcuTransport;

/// Production mode with real hardware connection
pub const MOCK_MODE: bool = false;

pub struct EcuConnection {
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
}

impl EcuConnection {
    pub fn new(port_name: &str) -> Result<Self, String> {
        if MOCK_MODE {
            // Mock mode - no actual port needed
            return Ok(Self { 
                port: None, 
                port_name: port_name.to_string() 
            });
        }
        
        let port = serialport::new(port_name, 10400)
            .timeout(Duration::from_millis(1000))
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()
            .map_err(|e| format!("เปิดพอร์ตไม่ได้: {}", e))?;
        
        Ok(Self { 
            port: Some(port), 
            port_name: port_name.to_string() 
        })
    }

    /// Honda K-Line Fast Init (maps to HondaECU.init in eculib/honda.py).
    /// eculib drives TX low via FTDI bitbang; here we use the UART break
    /// signal which is electrically identical (TX held low):
    /// 1. set_break()  -> TX low for 70ms  (bitmode + write 0x00)
    /// 2. clear_break() -> TX high, flush input, wait 130ms for ECU
    pub fn fast_init(&mut self) -> Result<(), String> {
        if MOCK_MODE {
            // Mock successful init
            println!("[MOCK] K-Line Fast Init สำเร็จบน {}", self.port_name);
            return Ok(());
        }

        // Real hardware implementation
        if let Some(ref mut port) = self.port {
            port.set_break().map_err(|e| format!("ตั้ง break ไม่ได้: {}", e))?;
            std::thread::sleep(Duration::from_millis(70));
            port.clear_break().map_err(|e| format!("เคลียร์ break ไม่ได้: {}", e))?;
            port.clear(ClearBuffer::Input).map_err(|e| format!("flush ไม่ได้: {}", e))?;
            std::thread::sleep(Duration::from_millis(130));
            Ok(())
        } else {
            Err("ไม่มีการเชื่อมต่อ".to_string())
        }
    }

    /// อ่านข้อมูลจาก ECU
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        if MOCK_MODE {
            // Return mock data
            for byte in buf.iter_mut() {
                *byte = 0xFF;
            }
            return Ok(buf.len());
        }

        if let Some(ref mut port) = self.port {
            port.read(buf).map_err(|e| e.to_string())
        } else {
            Err("ไม่มีการเชื่อมต่อ".to_string())
        }
    }

    /// เขียนข้อมูลไปยัง ECU
    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        if MOCK_MODE {
            println!("[MOCK] เขียน {} ไบต์ไปยับ ECU", data.len());
            return Ok(data.len());
        }

        if let Some(ref mut port) = self.port {
            port.write(data).map_err(|e| e.to_string())
        } else {
            Err("ไม่มีการเชื่อมต่อ".to_string())
        }
    }

    pub fn close(self) -> Result<(), String> {
        // Port is automatically closed when dropped; flush remaining data
        if let Some(mut port) = self.port {
            let _ = port.flush();
        }
        Ok(())
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }
}

impl EcuTransport for EcuConnection {
    fn endpoint(&self) -> &str {
        &self.port_name
    }

    fn fast_init(&mut self) -> Result<(), String> {
        EcuConnection::fast_init(self)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        EcuConnection::read(self, buf)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        EcuConnection::write(self, data)
    }
}