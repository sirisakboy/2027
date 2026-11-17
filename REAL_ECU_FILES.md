# ไฟล์ ECU จริง - พร้อมใช้งานแล้ว ✅

> **Source:** GitHub repos `sirisakboy/file` และ `sirisakboy/eculib`
> **Updated:** 20 ก.ย. 2026 - Day 1 Evening

## 📦 ไฟล์ที่ Copy มาแล้วใน `resources/`

### BIN Files (7 ไฟล์) - ไฟล์กล่องจริง

#### Stock (เดิมโรงงาน) - 6 ไฟล์
```
resources/bins/stock/
├── 38770-K2J-T01.bin (64KB) - Wave110i 2021 K2J-T01 ⭐ ยอดนิยมสุด!
├── CBR150-K45.bin (64KB) - CBR150R K45-TA1
├── CT125-K2E.bin (96KB) - CT125 K2E-T02
├── KVB-S53-Click110-48KB.bin (48KB) - Click110 KVB-S53
├── Scoopy-K16-B61.bin (64KB) - Scoopy i K16-B61
└── Zoomer-K20.bin (64KB) - Zoomer-X K20-T01
```

#### Modified (ไฟล์โม) - 1 ไฟล์
```
resources/bins/modified/
└── 110i 16000RPM.bin (64KB) - Wave110i 16000RPM (โมรอบสูง)
```

### XDF Files (26 ไฟล์) - นิยาม Map

```
resources/xdf/thai/
├── 64kb/ (12 files) - ยอดนิยมสุด!
│   ├── 38770-K03-H01 - Wave110i.xdf
│   ├── 38770-K16-B61 - Scoopy110i.xdf
│   ├── 38770-K2J-T01-.xdf ⭐ Wave110i 2021
│   ├── 38770-K45-N42-Cbr150i.xdf - CBR150
│   ├── 38770-K58-T81 - Wave110i.xdf
│   ├── 38770-K58M-TC1 - Wave110i LED.xdf
│   ├── 38770-K73-M62-T61 - Wave125i LED.xdf
│   └── ... (อีก 5 ไฟล์)
├── 48kb/ (8 files) - Click, Scoopy, MSX, Wave125i
│   ├── 38770-K03-T61 - Wave110i.xdf
│   ├── 38770-K20-901 - Zoomer110i.xdf
│   ├── 38770-K26-901 - MSX125i.xdf
│   └── ...
└── 96kb/ (4 files) - Monkey, CB150R
    ├── 38770-K0F-A01 MONKEY125 - RFi.xdf
    └── 38770-K94-T02 CB150R - RFi.xdf
```

## 🔍 รายละเอียด BIN ไฟล์จริง

### วิเคราะห์จากไฟล์จริง:

#### 1. KVB-S53 Click110 48KB (ที่ copy มาแล้ว)
```
Size: 49152 bytes (48KB)
Header: 00 e2 00 00... (code, ไม่ใช่ ASCII)
Checksum: 0xFC00
Honda Checksum: 0x00 (valid! ตาม eculib)
Model: Click110 KVB-S53
```

#### 2. ขนาดไฟล์ตามรุ่น (จาก file repo ทั้งหมด 50+ ไฟล์)
- **48KB**: Click110 KVB-S53, Scoopy K16-901, Spacy KZL-C03 (รถเล็กเก่า)
- **56KB**: CBR150 KPP-T03
- **64KB**: Wave110i K03, K2J, Scoopy K16-B61, Zoomer K20, MSX K26, CBR150 K45 (ยอดนิยม!)
- **96KB**: Monkey K0F, CT125 K2E, CB150R K94 (รุ่นใหม่ 2021)
- **128KB**: CBR300 K33
- **512KB**: Forza300

### XDF คืออะไร?
- XDF = TunerPro Definition File
- บอกว่าใน BIN ที่ address ไหนคืออะไร
- ตัวอย่าง K03-H01 Wave110i 64KB:
  - Fuel Map Low: 0x8000, 16x16, ms
  - Fuel Map High: 0x8400, 16x16, ms
  - Ignition Map Low: 0x9000, 16x16, °
  - RPM Limiter: 0xA000, 1x1, RPM

## 💻 Code ที่เขียนแล้วใช้ไฟล์จริง

### 1. `src/lib/bin-parser.ts` [✅ ทำแล้ว]
```typescript
// Real BIN parser ใช้ไฟล์จริง
- checksum8bitHonda() - จาก eculib
- parseBinFile() - อ่าน BIN จริง 48/64/96KB
- REAL_BIN_FILES - list 11 ไฟล์ยอดนิยมจาก file repo
- REAL_XDF_DATABASE - XDF definitions จริง
- extractMap() - ดึง fuel map จาก BIN
```

### 2. `src/components/BinViewer.svelte` [✅ ทำแล้ว Day 1 Evening]
```svelte
- แสดง list BIN จริง 8 ไฟล์ยอดนิยมจาก file repo
- คลิกโหลด BIN (mock แต่ใช้ขนาดจริง)
- แสดง Info: size, model, checksum, header
- แสดง Hex view 256 bytes แรก
- แสดง Fuel Map 16x16 สีน้ำเงิน-แดง
- บอกว่ามี BIN 11+ ไฟล์, XDF 26 ไฟล์ใน resources/
```

### 3. `src-tauri/src/ecu/honda.rs` [✅ อัพเดทด้วย eculib จริง]
```rust
// Real Honda protocol จาก eculib/honda.py
- checksum8bit_honda() - ((sum ^ 0xFF)+1) & 0xFF
- format_message() - mtype + [size] + data + [checksum]
- HondaCommand enum - Ping, Diag, DetectState, ReadInfo, etc.
- HondaECU struct - fast_init_sequence(), detect_state_commands()
- init_recover_sequence(), init_write_sequence(), erase_sequence()
- DTC_CODES - 26 codes
- Tests - checksum, format_message, ping
```

## 📚 eculib - Library จริง

### จาก `sirisakboy/eculib` (Python)
```python
# eculib/honda.py - 265 บรรทัด ของจริง!
- ECUSTATE: OFF, READ, OK, WRITE, ERASING, ERROR, etc.
- DTC: 01-01 MAP low, 07-01 ECT low, 08-01 TP low, etc.
- format_read(location): pack >I -> [tmp[1], tmp[3], tmp[2]]
- checksum8bitHonda(data): ((sum ^ 0xFF)+1) & 0xFF
- HondaECU:
  - init(): FTDI bitmode 1, 0x01, write 0x00, sleep 70ms, write 0x01, bitmode 0, flush, sleep 130ms (Fast Init!)
  - send(): write + wait echo + read response
  - send_command(): retries, checksum validation, dispatcher
  - ping(): [0xFE][0x72]
  - detect_ecu_state(): ส่ง [0x72][0x71,0x00], [0x7D][0x01,0x01,0x00], etc.
  - probe_tables(): [0x10,0x11,0x17,0x20,0x21,0x60,0x61...]
  - do_init_recover(), do_init_write(), do_erase(), get_faults()
```

**แปลงเป็น Rust แล้วใน `honda.rs`!**

## 🚀 วิธีใช้ไฟล์จริงในโปรแกรม

### ในโปรแกรม REMAP V.5 (port 1420) ตอนนี้:

1. เปิดโปรแกรม - จะเห็น **BIN Files จริง** list 8 ไฟล์ยอดนิยม
   - K2J-T01 Wave110i 2021 64KB (ยอดนิยม)
   - K0F Monkey 96KB
   - K45 CBR150R 64KB
   - etc.

2. คลิกไฟล์ - โหลด BIN mock แต่ใช้ขนาดจริง 48/64/96KB
   - แสดง Info: model, size, checksum, header
   - แสดง Hex: 256 bytes แรก
   - แสดง Fuel Map: 16x16 สี

3. ในอนาคต Day 2:
   - โหลด BIN จริงจาก `resources/bins/stock/` (6 ไฟล์ที่มีแล้ว)
   - Parse ด้วย `bin-parser.ts`
   - แสดง Map จริงตาม XDF

### สำหรับเขียน Code ต่อ:

```bash
# ดูไฟล์จริงทั้งหมด
ls file_real/ | head -n 20
ls file_real/"XDF THAI"/64kb/

# ดู eculib จริง
cat eculib_real/eculib/honda.py
cat eculib_real/eculib/base.py

# ไฟล์ที่ copy มาแล้ว
ls remap-v5/resources/bins/stock/
ls remap-v5/resources/xdf/thai/64kb/
```

## 📊 สถานะไฟล์จริง

| ประเภท | มีใน file repo | Copy มาแล้ว | เหลือ |
|--------|----------------|-------------|-------|
| BIN 48KB | 5 ไฟล์ | 1 ไฟล์ | 4 ไฟล์ |
| BIN 64KB | 20+ ไฟล์ | 4 ไฟล์ | 16+ ไฟล์ |
| BIN 96KB | 5 ไฟล์ | 1 ไฟล์ | 4 ไฟล์ |
| BIN Modified | 30+ ไฟล์ | 1 ไฟล์ | 29+ ไฟล์ |
| XDF 48KB | 8 ไฟล์ | 8 ไฟล์ | 0 ✅ |
| XDF 64KB | 12 ไฟล์ | 12 ไฟล์ | 0 ✅ |
| XDF 96KB | 4 ไฟล์ | 4 ไฟล์ | 0 ✅ |
| **รวม** | **~90 ไฟล์** | **33 ไฟล์** | **~57 ไฟล์** |

## 🎯 Next Steps

### Day 2 (30 นาที):
- [ ] Copy BIN เพิ่มอีก 10 ไฟล์ยอดนิยม (Wave110i, Dream, Monkey)
- [ ] เขียน `bin-parser.ts` ให้อ่าน BIN จริงจาก resources/ (ไม่ใช่ mock)
- [ ] ทำหน้า Compare BIN - เทียบ stock vs modified
- [ ] ทำ XDF parser - อ่าน XDF XML จริง

### Day 3:
- [ ] แปลง eculib ทั้งหมดเป็น Rust (honda.rs ทำแล้ว 80%)
- [ ] ทดสอบ checksum กับ BIN จริง 6 ไฟล์ที่มี
- [ ] ทำ fuel map editor

---

**สรุป:** มีไฟล์ ECU จริงพร้อมใช้งานแล้ว 33 ไฟล์ (BIN 7 + XDF 26) จากทั้งหมด 90+ ไฟล์ใน GitHub พร้อมเอามาเขียนโปรแกรมให้ใช้งานได้จริง!

**Source:** 
- BIN/XDF: https://github.com/sirisakboy/file
- eculib: https://github.com/sirisakboy/eculib
- Legacy: https://github.com/sirisakboy/files-archive
