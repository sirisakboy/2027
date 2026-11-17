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
    println!("Connecting to ECU on {}", port);
    // TODO: Implement K-Line fast init for real hardware
    // Mock for development
    Ok(format!("เชื่อมต่อ {} สำเร็จ - รอเปิดกุญแจ...", port))
}

#[tauri::command]
pub async fn read_rom(_port: String, offset: u32, size: u32) -> Result<Vec<u8>, String> {
    // TODO: Implement real Honda protocol
    // Mock for development - returns FF bytes
    if size == 0 || size > 131072 {
        return Err("ขนาดข้อมูลไม่ถูกต้อง (0 < size <= 131072)".to_string());
    }
    if offset > 0xFFFFF {
        return Err("ออฟเซ็ทเกินขีดจำกัด".to_string());
    }
    println!("Reading ROM: offset=0x{:X}, size={} bytes", offset, size);
    Ok(vec![0xFF; size as usize])
}

#[tauri::command]
pub async fn write_rom(port: String, data: Vec<u8>) -> Result<String, String> {
    if data.is_empty() {
        return Err("ข้อมูลเป็น空ว่าง".to_string());
    }
    if data.len() > 131072 {
        return Err("ขนาดข้อมูลเกินขีดจำกัด (131KB)".to_string());
    }
    println!("Writing ROM to {}: {} bytes", port, data.len());
    // Mock for development
    Ok(format!("แฟลชสำเร็จ {} ไบต์", data.len()))
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
pub async fn get_ecu_info(_port: String) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "id": "KEIHIN-XYZ",
        "size": "64KB",
        "checksum": "0xABCD"
    }))
}

#[tauri::command]
pub async fn reset_flash_count(_port: String) -> Result<String, String> {
    println!("Resetting flash counter");
    Ok("ลบ FLASHCOUNT สำเร็จ".into())
}

#[tauri::command]
pub async fn simulate_flash(port: String, data: Vec<u8>) -> Result<String, String> {
    if data.is_empty() {
        return Err("ข้อมูลเป็น空ว่าง".to_string());
    }
    let checksum: u16 = rom::checksum::calculate(&data);
    println!("Simulating flash to {}: {} bytes, checksum=0x{:04X}", port, data.len(), checksum);
    Ok(format!("แฟลชสำเร็จ {} ไบต์, checksum=0x{:04X}", data.len(), checksum))
}