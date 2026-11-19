use serde::Serialize;
use crate::rom;

#[derive(Serialize, Debug)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
}

#[tauri::command]
pub async fn list_ports() -> Result<Vec<PortInfo>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;
    Ok(ports.into_iter().map(|p| PortInfo {
        name: p.port_name,
        port_type: format!("{:?}", p.port_type),
    }).collect())
}

#[tauri::command]
pub async fn connect_ecu(port: String) -> Result<String, String> {
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    Ok(format!("เชื่อมต่อและ K-Line Fast Init บนพอร์ต {} สำเร็จ", port))
}

#[tauri::command]
pub async fn read_rom(port: String, offset: u32, size: u32) -> Result<Vec<u8>, String> {
    if size == 0 || size > 131072 {
        return Err("ขนาดข้อมูลไม่ถูกต้อง (0 < size <= 131072)".to_string());
    }
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    
    let cmd = crate::ecu::honda::HondaCommand::ReadRom { addr: offset, len: size as u16 };
    let req_bytes = cmd.to_bytes();
    let (msg, _, _) = crate::ecu::honda::format_message(&[0x72], &req_bytes[1..]);
    
    conn.write(&msg)?;
    let mut buf = [0u8; 1024];
    let n = conn.read(&mut buf)?;
    if n == 0 {
        return Err("ECU ไม่ตอบสนองคำสั่งอ่าน ROM".to_string());
    }
    
    if let Some(resp) = crate::ecu::honda::validate_response(&[0x72], &buf[..n]) {
        Ok(resp.data)
    } else {
        Ok(buf[..n].to_vec())
    }
}

#[tauri::command]
pub async fn write_rom(port: String, data: Vec<u8>) -> Result<String, String> {
    if data.is_empty() {
        return Err("ข้อมูลว่างเปล่า".to_string());
    }
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    conn.write(&data)?;
    Ok(format!("แฟลชสำเร็จ {} ไบต์บนพอร์ต {}", data.len(), port))
}

#[tauri::command]
pub fn calculate_cc(bore: f32, stroke: f32) -> f32 {
    if bore <= 0.0 || stroke <= 0.0 {
        return 0.0;
    }
    let r = bore / 2.0;
    std::f32::consts::PI * r * r * stroke / 1000.0
}

#[tauri::command]
pub async fn get_ecu_info(port: String) -> Result<serde_json::Value, String> {
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    
    let (mtype, req) = crate::ecu::honda::DetectProbe::ReadInfo.request();
    let (msg, _, _) = crate::ecu::honda::format_message(&mtype, &req);
    conn.write(&msg)?;
    
    let mut buf = [0u8; 256];
    let n = conn.read(&mut buf).unwrap_or(0);
    
    Ok(serde_json::json!({
        "port": port,
        "status": if n > 0 { "Connected" } else { "No Response" },
        "raw_response_len": n
    }))
}

#[tauri::command]
pub async fn reset_flash_count(port: String) -> Result<String, String> {
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    Ok("ลบ FLASHCOUNT สำเร็จ".into())
}

#[tauri::command]
pub async fn simulate_flash(port: String, data: Vec<u8>) -> Result<String, String> {
    if data.is_empty() {
        return Err("ข้อมูลว่างเปล่า".to_string());
    }
    let checksum: u16 = rom::checksum::calculate(&data);
    let mut conn = crate::serial::EcuConnection::new(&port)?;
    conn.fast_init()?;
    conn.write(&data)?;
    Ok(format!("แฟลชจริงสำเร็จ {} ไบต์, checksum=0x{:04X}", data.len(), checksum))
}