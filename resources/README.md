# Resources - Real ECU Files

Restored Stage 0 (23 ก.ย. 2026) from https://github.com/sirisakboy/file :
- `bins/stock/` - 10 ไฟล์ (48KB x2, 64KB x6, 96KB x2)
- `bins/modified/` - `110i-16000RPM.bin` (เทียบกับ stock ใน TunerPro)
- `xdf/models/` - XDF คู่ตัว 6 ไฟล์ (K2J, K2E, K45, KVB, K16-B61, K20)
- `xdf/{64kb,48kb,96kb}/` - XDF THAI รวม (~29 ไฟล์)

คู่สำหรับเปิดใน TunerPro RT: BIN ใน `bins/stock/` + XDF ชื่อเดียวกันใน
`xdf/models/` (หรือ `xdf/64kb|48kb|96kb/` สำหรับ K03/K58/K26/K0F)

Full restore (ถ้าต้องการทั้ง repo ต้นทาง):

To restore:
```bash
git clone https://github.com/sirisakboy/file.git file_real
cp file_real/"2021 S6"/"38770-k2J-T01 110i 2021"/38770-K2J-T01.bin resources/bins/stock/
cp file_real/"XDF THAI"/64kb/*.xdf resources/xdf/thai/64kb/
rm -rf file_real
```

Files are documented in ECU_FILES_REAL.md and REAL_ECU_FILES.md
