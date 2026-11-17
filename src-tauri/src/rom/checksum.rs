/// CRC-16 CCITT-FALSE (0x1021 polynomial)
/// ใช้ในการตรวจสอบความสมบูรณ์ของไฟล์ ECU Honda
pub fn calculate(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Simple byte sum checksum (legacy support)
pub fn calculate_simple(data: &[u8]) -> u16 {
    data.iter().map(|&b| b as u16).sum::<u16>()
}

pub fn verify(data: &[u8], expected: u16) -> bool {
    calculate(data) == expected
}

pub fn verify_simple(data: &[u8], expected: u16) -> bool {
    calculate_simple(data) == expected
}

/// Honda 8-bit checksum from eculib: ((sum ^ 0xFF) + 1) & 0xFF.
/// ใช้กับ K-Line frame และตรวจ ROM ทั้งภาพ (ผลรวมต้องเป็น 0).
pub fn honda(data: &[u8]) -> u8 {
    let sum: u32 = data.iter().map(|&b| b as u32).sum();
    (((sum ^ 0xFF) + 1) & 0xFF) as u8
}

/// ROM ถูกต้องเมื่อ checksum ทั้งภาพเป็น 0 (validate_checksums ใน eculib).
pub fn honda_image_valid(image: &[u8]) -> bool {
    honda(image) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crc16_calculate() {
        // ตัวอย่างการคำนวณ CRC-16
        let data = [0x00, 0x01, 0x02, 0x03];
        let _crc = calculate(&data);
    }
    
    #[test]
    fn test_simple_checksum() {
        let data = [0x01, 0x02, 0x03, 0x04];
        assert_eq!(calculate_simple(&data), 10);
    }

    #[test]
    fn test_honda_checksum_pdf_vector() {
        // WAKEUP FE 04 72 -> 8C (PDF Table 17)
        assert_eq!(honda(&[0xFE, 0x04, 0x72]), 0x8C);
        assert!(honda_image_valid(&[0xFE, 0x04, 0x72, 0x8C]));
    }
}