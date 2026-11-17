// DTC database: eculib fault codes (01-01 style) + MIL blink codes (PDF).
// Thai descriptions adapted from the reference tools' tables.

export interface DtcEntry {
  th: string
  symptom: string
}

/** eculib get_faults codes (from honda.py DTC dict). */
export const ECU_DTC: Record<string, DtcEntry> = {
  '01-01': { th: 'MAP เซ็นเซอร์ไฟต่ำ', symptom: 'เครื่องเดินปกติ' },
  '01-02': { th: 'MAP เซ็นเซอร์ไฟสูง', symptom: 'เครื่องเดินปกติ' },
  '02-01': { th: 'MAP เซ็นเซอร์เพี้ยน', symptom: 'เครื่องเดินปกติ' },
  '07-01': { th: 'ECT เซ็นเซอร์ไฟต่ำ', symptom: 'สตาร์ทยากตอนเครื่องเย็น' },
  '07-02': { th: 'ECT เซ็นเซอร์ไฟสูง', symptom: 'สตาร์ทยากตอนเครื่องเย็น' },
  '08-01': { th: 'TP เซ็นเซอร์ไฟต่ำ', symptom: 'บิดแล้วตอบสนองไม่ดี' },
  '08-02': { th: 'TP เซ็นเซอร์ไฟสูง', symptom: 'บิดแล้วตอบสนองไม่ดี' },
  '09-01': { th: 'IAT เซ็นเซอร์ไฟต่ำ', symptom: 'เครื่องเดินปกติ' },
  '09-02': { th: 'IAT เซ็นเซอร์ไฟสูง', symptom: 'เครื่องเดินปกติ' },
  '11-01': { th: 'สปีดเซ็นเซอร์ไม่มีสัญญาณ', symptom: 'เครื่องเดินปกติ' },
  '12-01': { th: 'หัวฉีดสูบ 1 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '13-01': { th: 'หัวฉีดสูบ 2 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '14-01': { th: 'หัวฉีดสูบ 3 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '15-01': { th: 'หัวฉีดสูบ 4 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '16-01': { th: 'หัวฉีดรองสูบ 1 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '17-01': { th: 'หัวฉีดรองสูบ 2 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '18-01': { th: 'CMP เซ็นเซอร์ไม่มีสัญญาณ', symptom: 'สตาร์ทไม่ติด' },
  '19-01': { th: 'CKP เซ็นเซอร์ไม่มีสัญญาณ', symptom: 'สตาร์ทไม่ติด' },
  '21-01': { th: 'O2 เซ็นเซอร์เสีย', symptom: 'เครื่องเดินปกติ' },
  '23-01': { th: 'ฮีตเตอร์ O2 เสีย', symptom: 'เครื่องเดินปกติ' },
  '25-02': { th: 'Knock เซ็นเซอร์วงจรเสีย', symptom: 'เครื่องเดินปกติ' },
  '25-03': { th: 'Knock เซ็นเซอร์วงจรเสีย', symptom: 'เครื่องเดินปกติ' },
  '29-01': { th: 'IACV วงจรเสีย', symptom: 'เดินเบาไม่นิ่ง' },
  '33-02': { th: 'ECM EEPROM เสีย', symptom: 'จำค่าผิดพลาดไม่ได้' },
  '34-01': { th: 'ECV POT ไฟต่ำ', symptom: 'เครื่องเดินปกติ' },
  '34-02': { th: 'ECV POT ไฟสูง', symptom: 'เครื่องเดินปกติ' },
  '35-01': { th: 'EGCA เสีย', symptom: 'เครื่องเดินปกติ' },
  '48-01': { th: 'หัวฉีดรองสูบ 3 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '49-01': { th: 'หัวฉีดรองสูบ 4 วงจรเสีย', symptom: 'สตาร์ทไม่ติด' },
  '51-01': { th: 'HESD โซลินอยด์เสีย', symptom: 'เครื่องเดินปกติ' },
  '54-01': { th: 'Bank angle ไฟต่ำ', symptom: 'สตาร์ทไม่ติด' },
  '54-02': { th: 'Bank angle ไฟสูง', symptom: 'สตาร์ทไม่ติด' },
  '56-01': { th: 'Knock IC เสีย', symptom: 'เครื่องเดินปกติ' },
  '86-01': { th: 'สื่อสาร serial ขัดข้อง', symptom: 'ต่อกล่องไม่ได้' },
}

/** MIL blink codes (service manual / PDF table). */
export const MIL_CODES: Record<string, DtcEntry> = {
  '1': { th: 'MAP เซ็นเซอร์/วงจร', symptom: 'เครื่องเดินปกติ' },
  '2': { th: 'ท่อลม MAP หลุด', symptom: 'MAP เพี้ยน' },
  '7': { th: 'ECT เซ็นเซอร์/วงจร', symptom: 'สตาร์ทยากตอนเย็น' },
  '8': { th: 'TP เซ็นเซอร์/วงจร', symptom: 'บิดตอบสนองไม่ดี' },
  '9': { th: 'IAT เซ็นเซอร์/วงจร', symptom: 'เครื่องเดินปกติ' },
  '11': { th: 'สปีดเซ็นเซอร์/วงจร', symptom: 'เครื่องเดินปกติ' },
  '12': { th: 'หัวฉีดสูบ 1', symptom: 'สตาร์ทไม่ติด' },
  '13': { th: 'หัวฉีดสูบ 2', symptom: 'สตาร์ทไม่ติด' },
  '14': { th: 'หัวฉีดสูบ 3', symptom: 'สตาร์ทไม่ติด' },
  '15': { th: 'หัวฉีดสูบ 4', symptom: 'สตาร์ทไม่ติด' },
  '18': { th: 'พัลเซอร์/CMP', symptom: 'สตาร์ทไม่ติด' },
  '19': { th: 'พัลส์จุดระเบิด/CKP', symptom: 'สตาร์ทไม่ติด' },
  '21': { th: 'O2 เซ็นเซอร์', symptom: 'เครื่องเดินปกติ' },
  '23': { th: 'ฮีตเตอร์ O2', symptom: 'เครื่องเดินปกติ' },
  '33': { th: 'E2-PROM ใน ECM', symptom: 'ไม่จำค่าผิดพลาด' },
}

/** OBD-II Standard P-codes & U-codes from technician reference manual */
export const OBD2_CODES: Record<string, DtcEntry> = {
  'P0010': { th: 'วงจรตำแหน่งขณะทำงานเพลาข้อเหวี่ยง A (แบงค์ 1)', symptom: 'เครื่องยนต์เดินไม่เรียบ' },
  'P0030': { th: 'วงจรควบคุมฮีตเตอร์เซนเซอร์ความร้อน O2 (แบงค์ 1 เซนเซอร์ 1)', symptom: 'ไฟเครื่องโชว์ กินน้ำมัน' },
  'P0068': { th: 'MAP/MAF สัมพันธ์กับตำแหน่งปีกผีเสื้อผิดปกติ', symptom: 'คันเร่งกระตุก หรือรอบตก' },
  'P0100': { th: 'วงจรปริมาณอัตราการไหลอากาศ (แอร์โฟลว์)', symptom: 'เร่งไม่ขึ้น' },
  'P0105': { th: 'วงจรแรงดัน Manifold Absolute Pressure (MAP)', symptom: 'เครื่องวูบ ควันดำ' },
  'P0110': { th: 'วงจรเซนเซอร์อุณหภูมิไอดี (IAT)', symptom: 'สตาร์ทติดยากตอนร้อน/เย็น' },
  'P0115': { th: 'วงจรเซนเซอร์อุณหภูมิน้ำหล่อเย็น (ECT)', symptom: 'พัดลมไม่ตัด หรือสตาร์ทยาก' },
  'P0120': { th: 'วงจรสวิตช์/เซนเซอร์ตำแหน่งปีกผีเสื้อ (TPS)', symptom: 'รอบเครื่องค้าง คันเร่งกระตุก' },
  'P0130': { th: 'วงจรออกซิเจนเซนเซอร์ (O2 Sensor แบงค์ 1)', symptom: 'กินน้ำมัน อัตราเร่งตก' },
  'P0171': { th: 'ส่วนผสมน้ำมันเชื้อเพลิงบางเกินไป (System Too Lean)', symptom: 'สะอึก เร่งไม่ขึ้น เสี่ยงลูกสูบละลาย' },
  'P0172': { th: 'ส่วนผสมน้ำมันเชื้อเพลิงหนาเกินไป (System Too Rich)', symptom: 'ควันดำ หัวเทียนบอด กินน้ำมัน' },
  'P0200': { th: 'วงจรหัวฉีดน้ำมันเชื้อเพลิงเปิดหรือลัดวงจร', symptom: 'เครื่องสะดุด สตาร์ทไม่ติด' },
  'P0201': { th: 'วงจรหัวฉีดน้ำมันเชื้อเพลิง สูบที่ 1 ขัดข้อง', symptom: 'เดิน 3 สูบ หรือสะดุด' },
  'P0217': { th: 'สภาวะอุณหภูมิเครื่องยนต์/น้ำหล่อเย็นสูงเกินขีดจำกัด', symptom: 'โอเวอร์ฮีท เครื่องน็อค' },
  'P0219': { th: 'สภาวะความเร็วรอบเครื่องยนต์สูงเกินไป (Engine Overspeed)', symptom: 'รอบเกินพิกัด ตัดรอบ' },
  'P0300': { th: 'ตรวจพบจุดระเบิดผิดพลาดหลายสูบ (Random Misfire)', symptom: 'เครื่องสั่นรุนแรง เร่งไม่ขึ้น' },
  'P0301': { th: 'ตรวจพบจุดระเบิดผิดพลาด สูบที่ 1 (Cylinder 1 Misfire)', symptom: 'เครื่องสะดุดที่รอบต่ำ/สูง' },
  'P0325': { th: 'วงจรน็อคเซนเซอร์ 1 (Knock Sensor) ขัดข้อง', symptom: 'เครื่องมีเสียงเขก ลดทอนองศาไฟ' },
  'P0335': { th: 'วงจรเซนเซอร์ตำแหน่งเพลาข้อเหวี่ยง (CKP Sensor A)', symptom: 'สตาร์ทไม่ติด ดับกลางอากาศ' },
  'P0340': { th: 'วงจรเซนเซอร์ตำแหน่งเพลาลูกเบี้ยว (CMP Sensor A)', symptom: 'รอบเครื่องไม่นิ่ง สตาร์ทยาก' },
  'P0350': { th: 'วงจรคอยล์จุดระเบิดไฟแรงสูง/ต่ำ ขัดข้อง', symptom: 'ไม่มีประกายไฟ ดับ' },
  'P0500': { th: 'เซนเซอร์วัดความเร็วรถยนต์ (VSS Vehicle Speed Sensor)', symptom: 'ไมล์ไม่ขึ้น ระบบเกียร์เพี้ยน' },
  'P0505': { th: 'ระบบควบคุมรอบเดินเบาขัดข้อง (Idle Air Control System)', symptom: 'รอบเดินเบาตกหรือสูงผิดปกติ' },
  'P0562': { th: 'ระบบแรงดันไฟฟ้าแบตเตอรี่ต่ำเกินไป (System Voltage Low)', symptom: 'ไฟหรี่ รีสตาร์ทเอง' },
  'P0601': { th: 'เช็คหน่วยความจำควบคุมภายใน (Memory Checksum Error)', symptom: 'กล่องรวน ค่าเพี้ยน' },
  'P0606': { th: 'หน่วยประมวลผล ECM/PCM ทำงานผิดพลาด (Processor Fault)', symptom: 'เครื่องยนต์ดับ ไม่สามารถขับขี่ได้' },
  'U0001': { th: 'การติดต่อสื่อสารความเร็วสูง CAN BUS บกพร่อง', symptom: 'เรือนไมล์ไม่ตอบสนอง' },
  'U0100': { th: 'ขาดการสื่อสารกับโมดูลควบคุมเครื่องยนต์ (Lost Comm with ECM)', symptom: 'ระบบไฟฟ้าหลักไม่ทำงาน' },
  'U0101': { th: 'ขาดการสื่อสารกับโมดูลควบคุมเกียร์ (Lost Comm with TCM)', symptom: 'เกียร์ล็อคเซฟโหมด' }
}

export function describeDtc(code: string): DtcEntry {
  const norm = code.trim().toUpperCase()
  return ECU_DTC[code] ?? MIL_CODES[code] ?? OBD2_CODES[norm] ?? { th: 'รหัสทั่วไป', symptom: 'ตรวจสอบระบบเครื่องยนต์และสายไฟ' }
}

export function searchDtc(query: string): Array<{ code: string; th: string; symptom: string; type: string }> {
  const q = query.trim().toUpperCase()
  if (!q) return []
  const results: Array<{ code: string; th: string; symptom: string; type: string }> = []

  // Search eculib codes
  for (const [code, val] of Object.entries(ECU_DTC)) {
    if (code.toUpperCase().includes(q) || val.th.includes(q) || val.symptom.includes(q)) {
      results.push({ code, th: val.th, symptom: val.symptom, type: 'Honda K-Line (eculib)' })
    }
  }

  // Search MIL codes
  for (const [code, val] of Object.entries(MIL_CODES)) {
    if (code === q || val.th.includes(q) || val.symptom.includes(q)) {
      results.push({ code: `MIL ${code}`, th: val.th, symptom: val.symptom, type: 'Honda MIL Blink' })
    }
  }

  // Search OBD2 codes
  for (const [code, val] of Object.entries(OBD2_CODES)) {
    if (code.toUpperCase().includes(q) || val.th.includes(q) || val.symptom.includes(q)) {
      results.push({ code, th: val.th, symptom: val.symptom, type: 'OBD-II Standard' })
    }
  }

  return results.slice(0, 15)
}
