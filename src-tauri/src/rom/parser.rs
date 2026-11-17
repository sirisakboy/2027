//! Real Honda Keihin BIN parser.
//! Works on actual ROM dumps (see resources/bins/). Map cell ADDRESSES come
//! from XDF definitions (future xdf.rs) - this module validates the image
//! and extracts rectangular regions on demand.

/// MCU family inferred from image size (matches file-repo + ECM ID table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McuFamily {
    R8c48k,
    R8c64k,
    R8c96k,
    Unknown,
}

pub const SIZE_48K: usize = 48 * 1024;
pub const SIZE_64K: usize = 64 * 1024;
pub const SIZE_96K: usize = 96 * 1024;

impl McuFamily {
    pub fn from_size(size: usize) -> Self {
        match size {
            SIZE_48K => Self::R8c48k,
            SIZE_64K => Self::R8c64k,
            SIZE_96K => Self::R8c96k,
            _ => Self::Unknown,
        }
    }

    pub fn flash_size(&self) -> usize {
        match self {
            Self::R8c48k => SIZE_48K,
            Self::R8c64k => SIZE_64K,
            Self::R8c96k => SIZE_96K,
            Self::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinInfo {
    pub size: usize,
    pub family: McuFamily,
    /// Whole-image Honda checksum == 0 (eculib validate_checksums).
    /// NOTE: some families store the checksum at a dedicated offset
    /// (ECM ID table: 7600/B000/DFEF/3FFF8/...) instead, so `false` here
    /// does not always mean a modified file.
    pub checksum_valid: bool,
    pub checksum_value: u8,
}

pub fn honda_checksum(data: &[u8]) -> u8 {
    let sum: u32 = data.iter().map(|&b| b as u32).sum();
    (((sum ^ 0xFF) + 1) & 0xFF) as u8
}

pub fn analyze(data: &[u8]) -> BinInfo {
    BinInfo {
        size: data.len(),
        family: McuFamily::from_size(data.len()),
        checksum_valid: honda_checksum(data) == 0,
        checksum_value: honda_checksum(data),
    }
}

#[derive(Debug, Clone)]
pub struct KeihinBin {
    pub info: BinInfo,
    pub data: Vec<u8>,
}

impl KeihinBin {
    /// Parse a ROM dump. Only accepts known Honda image sizes.
    pub fn parse(data: Vec<u8>) -> Result<Self, String> {
        let info = analyze(&data);
        if info.family == McuFamily::Unknown {
            return Err(format!("ขนาดไฟล์ไม่รู้จัก ({} bytes) - ต้องเป็น 48/64/96KB", data.len()));
        }
        Ok(Self { info, data })
    }

    /// Extract a rows x cols region (e.g. 16x16 fuel map) at `offset`.
    /// Returns empty vec when out of bounds - never panics.
    pub fn map_at(&self, offset: usize, rows: usize, cols: usize) -> Vec<Vec<u8>> {
        let len = rows.saturating_mul(cols);
        if len == 0 || offset.saturating_add(len) > self.data.len() {
            return vec![];
        }
        self.data[offset..offset + len]
            .chunks(cols)
            .map(|row| row.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real ROM dumps from the file repo (resources/bins/stock/).
    const K03_H01: &[u8] = include_bytes!("../../../resources/bins/stock/38770-K03-H01-Wave110i.bin");
    const K2J_T01: &[u8] = include_bytes!("../../../resources/bins/stock/38770-K2J-T01.bin");
    const KVB_S53: &[u8] = include_bytes!("../../../resources/bins/stock/KVB-S53-Click110-48KB.bin");

    #[test]
    fn real_64k_stock_image_validates() {
        let bin = KeihinBin::parse(K03_H01.to_vec()).expect("known 64KB size");
        assert_eq!(bin.info.size, 65536);
        assert_eq!(bin.info.family, McuFamily::R8c64k);
        assert!(bin.info.checksum_valid, "stock K03-H01 whole-image sum must be 0");
    }

    #[test]
    fn real_48k_stock_image_validates() {
        let bin = KeihinBin::parse(KVB_S53.to_vec()).expect("known 48KB size");
        assert_eq!(bin.info.family, McuFamily::R8c48k);
        assert!(bin.info.checksum_valid);
    }

    #[test]
    fn k2j_size_detected_checksum_per_offset() {
        // K2J-T01 keeps its checksum at a dedicated offset (not whole-image),
        // so only size/family are asserted here.
        let bin = KeihinBin::parse(K2J_T01.to_vec()).expect("known 64KB size");
        assert_eq!(bin.info.family, McuFamily::R8c64k);
    }

    #[test]
    fn map_extraction_and_bounds() {
        let bin = KeihinBin::parse(K03_H01.to_vec()).unwrap();
        let map = bin.map_at(0x8000, 16, 16);
        assert_eq!(map.len(), 16);
        assert!(map.iter().all(|row| row.len() == 16));
        assert!(bin.map_at(0xFFFF, 16, 16).is_empty());
        assert!(bin.map_at(0, 0, 16).is_empty());
    }

    #[test]
    fn unknown_size_rejected() {
        assert!(KeihinBin::parse(vec![0u8; 100]).is_err());
        assert!(KeihinBin::parse(vec![]).is_err());
    }
}
