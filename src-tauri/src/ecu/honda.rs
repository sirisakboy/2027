// Honda Keihin ECU Protocol
// Ported from eculib/honda.py (sirisakboy/eculib) - pure logic, no I/O.
// Transport (serial read/write) lives in crate::serial; this module only
// builds messages, validates responses and classifies ECU state so every
// function below is unit-testable without hardware.

pub const WAKEUP: u8 = 0xFE;
pub const ACK: u8 = 0x72;

// Kept for backwards compatibility with existing callers.
pub enum HondaCommand {
    ReadRom { addr: u32, len: u16 },
    WriteRom { addr: u32, data: Vec<u8> },
    ResetFlash,
    GetId,
}

impl HondaCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::ReadRom { addr, len } => {
                vec![0x72, 0x01, (*addr >> 16) as u8, (*addr >> 8) as u8, *addr as u8, (*len >> 8) as u8, *len as u8]
            },
            _ => vec![]
        }
    }
}

/// ECU operating states (mirrors eculib ECUSTATE).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcuState {
    Undefined = -1,
    Off = 0,
    Read = 1,
    Reading = 2,
    Ok = 3,
    RecoverOld = 4,
    RecoverNew = 5,
    Write = 6,
    Writing = 7,
    Erasing = 8,
    InitWrite = 9,
    InitRecover = 10,
    Error = 11,
    Unknown = 12,
}

/// DTC code -> description, straight from eculib.
pub const DTC_TABLE: &[(&str, &str)] = &[
    ("01-01", "MAP sensor circuit low voltage"),
    ("01-02", "MAP sensor circuit high voltage"),
    ("02-01", "MAP sensor performance problem"),
    ("07-01", "ECT sensor circuit low voltage"),
    ("07-02", "ECT sensor circuit high voltage"),
    ("08-01", "TP sensor circuit low voltage"),
    ("08-02", "TP sensor circuit high voltage"),
    ("09-01", "IAT sensor circuit low voltage"),
    ("09-02", "IAT sensor circuit high voltage"),
    ("11-01", "VS sensor no signal"),
    ("12-01", "No.1 primary injector circuit malfunction"),
    ("13-01", "No.2 primary injector circuit malfunction"),
    ("14-01", "No.3 primary injector circuit malfunction"),
    ("15-01", "No.4 primary injector circuit malfunction"),
    ("16-01", "No.1 secondary injector circuit malfunction"),
    ("17-01", "No.2 secondary injector circuit malfunction"),
    ("18-01", "CMP sensor no signal"),
    ("19-01", "CKP sensor no signal"),
    ("21-01", "O2 sensor malfunction"),
    ("23-01", "O2 sensor heater malfunction"),
    ("25-02", "Knock sensor circuit malfunction"),
    ("25-03", "Knock sensor circuit malfunction"),
    ("29-01", "IACV circuit malfunction"),
    ("33-02", "ECM EEPROM malfunction"),
    ("34-01", "ECV POT low voltage malfunction"),
    ("34-02", "ECV POT high voltage malfunction"),
    ("35-01", "EGCA malfunction"),
    ("48-01", "No.3 secondary injector circuit malfunction"),
    ("49-01", "No.4 secondary injector circuit malfunction"),
    ("51-01", "HESD linear solenoid malfunction"),
    ("54-01", "Bank angle sensor circuit low voltage"),
    ("54-02", "Bank angle sensor circuit high voltage"),
    ("56-01", "Knock sensor IC malfunction"),
    ("86-01", "Serial communication malfunction"),
];

pub fn dtc_description(code: &str) -> Option<&'static str> {
    DTC_TABLE.iter().find(|(c, _)| *c == code).map(|(_, d)| *d)
}

/// Honda 8-bit checksum: ((sum ^ 0xFF) + 1) & 0xFF.
pub fn checksum8bit_honda(data: &[u8]) -> u8 {
    let sum: u32 = data.iter().map(|&b| b as u32).sum();
    (((sum ^ 0xFF) + 1) & 0xFF) as u8
}

/// Legacy variant kept from eculib.
pub fn checksum8bit_legacy(data: &[u8]) -> u8 {
    let sum: u32 = data.iter().map(|&b| b as u32).sum();
    (0xFF - ((sum.wrapping_sub(1)) >> 8)) as u8
}

/// Whole-image validation: a correct Honda ROM sums to zero.
pub fn image_checksum_valid(image: &[u8]) -> bool {
    checksum8bit_honda(image) == 0
}

/// Pack a 32-bit address the way eculib format_read does: [b1, b3, b2].
pub fn format_read(location: u32) -> [u8; 3] {
    let b = location.to_be_bytes();
    [b[1], b[3], b[2]]
}

/// Build a framed message: mtype + [size] + data + [checksum].
/// Returns (bytes, mtype_len, data_len).
pub fn format_message(mtype: &[u8], data: &[u8]) -> (Vec<u8>, usize, usize) {
    let ml = mtype.len();
    let dl = data.len();
    let msgsize = (0x02 + ml + dl) as u8;
    let mut msg = Vec::with_capacity(ml + 1 + dl + 1);
    msg.extend_from_slice(mtype);
    msg.push(msgsize);
    msg.extend_from_slice(data);
    let cksum = checksum8bit_honda(&msg);
    msg.push(cksum);
    debug_assert_eq!(msg[ml] as usize, msg.len());
    (msg, ml, dl)
}

/// Parsed response from send_command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedResponse {
    pub rtype: Vec<u8>,
    pub rlen: u8,
    pub data: Vec<u8>,
    pub dlen: i32,
}

/// Validate checksum + response-type echo, then split header/payload.
/// Mirrors the ml == 1/2/3 branches of eculib send_command.
pub fn validate_response(mtype: &[u8], resp: &[u8]) -> Option<ParsedResponse> {
    let ml = mtype.len();
    if resp.len() < ml + 2 {
        return None;
    }
    if checksum8bit_honda(&resp[..resp.len() - 1]) != resp[resp.len() - 1] {
        return None;
    }
    let rmtype = &resp[..ml];
    let valid = match ml {
        3 => rmtype[0] == mtype[0] | 0x10 && rmtype[1] == mtype[1] | 0x10,
        2 => rmtype == mtype,
        1 => rmtype[0] == mtype[0] & 0x0f,
        _ => false,
    };
    if !valid {
        return None;
    }
    let rlen = resp[ml];
    let dlen = rlen as i32 - 2 - ml as i32;
    let data = resp[ml + 1..resp.len() - 1].to_vec();
    Some(ParsedResponse { rtype: rmtype.to_vec(), rlen, data, dlen })
}

/// Probes of detect_ecu_state, in order. The caller performs I/O and feeds
/// each response to classify(); Some(state) is decisive, None means "next".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectProbe {
    T0,
    RecoverOld,
    RecoverNew,
    WriteStatus,
    ReadInfo,
}

pub const DETECT_SEQUENCE: &[DetectProbe] = &[
    DetectProbe::T0,
    DetectProbe::RecoverOld,
    DetectProbe::RecoverNew,
    DetectProbe::WriteStatus,
    DetectProbe::ReadInfo,
];

impl DetectProbe {
    pub fn request(&self) -> (Vec<u8>, Vec<u8>) {
        match self {
            Self::T0 => (vec![0x72], vec![0x71, 0x00]),
            Self::RecoverOld => (vec![0x7d], vec![0x01, 0x01, 0x00]),
            Self::RecoverNew => (vec![0x7b], vec![0x00, 0x01, 0x01]),
            Self::WriteStatus => (vec![0x7e], vec![0x01, 0x01, 0x00]),
            Self::ReadInfo => (vec![0x82, 0x82, 0x00], vec![0x00, 0x00, 0x00, 0x08]),
        }
    }

    pub fn classify(&self, parsed: Option<&ParsedResponse>) -> Option<EcuState> {
        match self {
            Self::T0 => match parsed {
                Some(r) if r.data.len() >= 7 && r.data[5..7] != [0x00, 0x00] => Some(EcuState::Ok),
                _ => None,
            },
            Self::RecoverOld => parsed.map(|_| EcuState::RecoverOld),
            Self::RecoverNew => parsed.map(|_| EcuState::RecoverNew),
            Self::WriteStatus => parsed.map(|r| {
                if r.data.get(1) == Some(&0xf0) { EcuState::Error } else { EcuState::Write }
            }),
            Self::ReadInfo => parsed.map(|_| EcuState::Read),
        }
    }
}

/// Default table IDs for probe_tables.
pub const PROBE_TABLES: [u8; 12] =
    [0x10, 0x11, 0x17, 0x20, 0x21, 0x60, 0x61, 0x67, 0x70, 0x71, 0xd0, 0xd1];

pub fn probe_table_request(table: u8) -> (Vec<u8>, Vec<u8>) {
    (vec![0x72], vec![0x71, table])
}

pub fn init_recover_messages() -> Vec<(Vec<u8>, Vec<u8>)> {
    vec![
        (vec![0x7b], vec![0x00, 0x01, 0x01]),
        (vec![0x7b], vec![0x00, 0x01, 0x02]),
        (vec![0x7b], vec![0x00, 0x01, 0x03]),
        (vec![0x7b], vec![0x00, 0x02, 0x76, 0x03, 0x17]),
        (vec![0x7b], vec![0x00, 0x03, 0x75, 0x05, 0x13]),
    ]
}

pub fn init_write_messages() -> Vec<(Vec<u8>, Vec<u8>)> {
    vec![
        (vec![0x7d], vec![0x01, 0x01, 0x01]),
        (vec![0x7d], vec![0x01, 0x01, 0x02]),
        (vec![0x7d], vec![0x01, 0x01, 0x03]),
        (vec![0x7d], vec![0x01, 0x02, 0x50, 0x47, 0x4d]),
        (vec![0x7d], vec![0x01, 0x03, 0x2d, 0x46, 0x49]),
    ]
}

pub fn erase_messages() -> Vec<(Vec<u8>, Vec<u8>)> {
    vec![
        (vec![0x7e], vec![0x01, 0x02]),
        (vec![0x7e], vec![0x01, 0x03, 0x00, 0x00]),
        (vec![0x7e], vec![0x01, 0x0b, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff]),
        (vec![0x7e], vec![0x01, 0x0e, 0x01, 0x90]),
        (vec![0x7e], vec![0x01, 0x01, 0x01]),
        (vec![0x7e], vec![0x01, 0x04, 0xff]),
    ]
}

pub fn erase_poll_request() -> (Vec<u8>, Vec<u8>) {
    (vec![0x7e], vec![0x01, 0x05])
}

/// do_erase_wait polls until payload byte 1 reads 0x00.
pub fn erase_complete(data: &[u8]) -> bool {
    data.get(1) == Some(&0x00)
}

pub fn post_write_messages() -> Vec<(Vec<u8>, Vec<u8>)> {
    vec![
        (vec![0x7e], vec![0x01, 0x09]),
        (vec![0x7e], vec![0x01, 0x0a]),
        (vec![0x7e], vec![0x01, 0x0c]),
        (vec![0x7e], vec![0x01, 0x0d]),
    ]
}

/// do_post_write succeeds when the final reply payload byte 1 is 0x0f.
pub fn post_write_ok(data: &[u8]) -> bool {
    data.get(1) == Some(&0x0f)
}

/// Fault request builders: current = [0x72][0x74, i], past = [0x72][0x73, i].
pub fn fault_request(past: bool, index: u8) -> (Vec<u8>, Vec<u8>) {
    (vec![0x72], vec![if past { 0x73 } else { 0x74 }, index])
}

/// Pull "%02d-%02d" codes from payload pairs at indices (3,4), (5,6), (7,8).
pub fn extract_fault_codes(data: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    for j in [3usize, 5, 7] {
        if data.len() > j + 1 && data[j] != 0 {
            out.push(format!("{:02}-{:02}", data[j], data[j + 1]));
        }
    }
    out
}

/// get_faults stops paging when payload byte 2 reads 0x00.
pub fn faults_done(data: &[u8]) -> bool {
    data.get(2) == Some(&0x00)
}

// --- Seed/Key unlock framework ----------------------------------------------
/// Challenge-response unlock required before erase/write on newer ECUs.
///
/// Flow (from the reference flash tool's behavior):
/// 1. after Fast Init + state detect, tool sends "request seed"
/// 2. ECU replies with a seed (typically 2-4 bytes)
/// 3. tool sends back `key = transform(seed, vendor_secret)`
/// 4. ECU accepts -> erase/write unlocked; rejects -> [`EcuState::Error`]
///
/// eculib's K-Line path needs no seed/key (older R8C families). The concrete
/// request bytes and the `transform` differ per [`EcuVendor`] and must be
/// recovered by dynamic analysis: capture seed->key pairs with the reference
/// tool on a bench ECU (or decompile it), then implement [`SeedKeyDerive`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcuVendor {
    Keihin,
    Shindengen,
    Unknown,
}

/// Key derivation for one vendor. Implement per captured behavior.
pub trait SeedKeyDerive {
    fn vendor(&self) -> EcuVendor;
    fn derive_key(&self, seed: &[u8]) -> Vec<u8>;
}

/// Ordered unlock steps the caller drives over the transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnlockStep {
    RequestSeed,
    SendKey,
    Done,
}

pub fn unlock_flow() -> Vec<UnlockStep> {
    vec![UnlockStep::RequestSeed, UnlockStep::SendKey, UnlockStep::Done]
}

/// Accept shape for the key reply. Default rule (refine per capture):
/// first status byte 0x00 means unlocked.
pub fn key_accepted(payload: &[u8]) -> bool {
    payload.first() == Some(&0x00)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wakeup_vector_matches_pdf_table17() {
        assert_eq!(format_message(&[0xFE], &[0x72]).0, vec![0xFE, 0x04, 0x72, 0x8C]);
    }

    #[test]
    fn vin_vector_matches_pdf_table17() {
        assert_eq!(format_message(&[0x72], &[0x71, 0x00]).0, vec![0x72, 0x05, 0x71, 0x00, 0x18]);
    }

    #[test]
    fn live_data_vector_matches_pdf_table17() {
        assert_eq!(format_message(&[0x72], &[0x71, 0x17]).0, vec![0x72, 0x05, 0x71, 0x17, 0x01]);
    }

    #[test]
    fn read_dtc_vector_matches_pdf_table17() {
        assert_eq!(format_message(&[0x72], &[0x74, 0x01]).0, vec![0x72, 0x05, 0x74, 0x01, 0x14]);
    }

    #[test]
    fn clear_dtc_vector_matches_pdf_table17() {
        assert_eq!(format_message(&[0x72], &[0x60, 0x01]).0, vec![0x72, 0x05, 0x60, 0x01, 0x28]);
    }

    #[test]
    fn format_read_byte_order() {
        assert_eq!(format_read(0x12345678), [0x34, 0x78, 0x56]);
    }

    #[test]
    fn framed_response_roundtrip() {
        let (msg, ml, dl) = format_message(&[0x72], &[0x71, 0x00]);
        assert_eq!((ml, dl), (1, 2));
        // ml == 1 branch expects echo byte masked with 0x0f
        let mut echo = msg.clone();
        echo[0] &= 0x0f;
        let last = echo.len() - 1;
        echo[last] = checksum8bit_honda(&echo[..last]);
        let parsed = validate_response(&[0x72], &echo).expect("valid echo");
        assert_eq!(parsed.data, vec![0x71, 0x00]);
    }

    #[test]
    fn bad_checksum_rejected() {
        let mut msg = format_message(&[0x72], &[0x71, 0x00]).0;
        let last = msg.len() - 1;
        msg[last] ^= 0xFF;
        assert!(validate_response(&[0x72], &msg).is_none());
    }

    #[test]
    fn dtc_lookup() {
        assert_eq!(dtc_description("07-01"), Some("ECT sensor circuit low voltage"));
        assert_eq!(dtc_description("99-99"), None);
    }

    #[test]
    fn detect_classify_ok_and_write() {
        let t0 = ParsedResponse { rtype: vec![0x72], rlen: 9, data: vec![0, 0, 0, 0, 0, 1, 2], dlen: 6 };
        assert_eq!(DetectProbe::T0.classify(Some(&t0)), Some(EcuState::Ok));
        let t0zero = ParsedResponse { rtype: vec![0x72], rlen: 9, data: vec![0; 7], dlen: 6 };
        assert_eq!(DetectProbe::T0.classify(Some(&t0zero)), None);
        let w = ParsedResponse { rtype: vec![0x7e], rlen: 5, data: vec![0, 0xf0], dlen: 1 };
        assert_eq!(DetectProbe::WriteStatus.classify(Some(&w)), Some(EcuState::Error));
    }

    #[test]
    fn fault_helpers() {
        assert_eq!(extract_fault_codes(&[0, 0, 1, 7, 1, 0, 0, 0, 0]), vec!["07-01".to_string()]);
        assert!(faults_done(&[0, 0, 0]));
        assert!(erase_complete(&[0, 0x00]));
        assert!(post_write_ok(&[0, 0x0f]));
    }

    struct DummyDerive;
    impl SeedKeyDerive for DummyDerive {
        fn vendor(&self) -> EcuVendor {
            EcuVendor::Keihin
        }
        fn derive_key(&self, seed: &[u8]) -> Vec<u8> {
            seed.iter().map(|b| b ^ 0xFF).collect()
        }
    }

    #[test]
    fn unlock_flow_order_and_key_shape() {
        assert_eq!(
            unlock_flow(),
            vec![UnlockStep::RequestSeed, UnlockStep::SendKey, UnlockStep::Done]
        );
        let d = DummyDerive;
        assert_eq!(d.vendor(), EcuVendor::Keihin);
        assert_eq!(d.derive_key(&[0x12, 0x34]), vec![0xED, 0xCB]);
        assert!(key_accepted(&[0x00, 0x11]));
        assert!(!key_accepted(&[0x01]));
        assert!(!key_accepted(&[]));
    }
}
