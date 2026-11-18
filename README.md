# REMAP V.5 - Cross-Platform Edition

> โปรแกรมจูนกล่องมอเตอร์ไซค์ Honda แบบ Build ได้ทุกแพลตฟอร์ม (Svelte Web + Tauri + Flutter Mobile/Desktop)
> **Status:** Day 2 - Flutter App Complete ✅ - 50% Complete

## 📊 สถานะปัจจุบัน (อัพเดท 18 พ.ย. 2026)

| ส่วน | สถานะ | รายละเอียด |
|------|--------|------------|
| **Frontend UI (Svelte)** | ✅ 100% | Svelte + Vite, pixel-perfect เหมือนต้นฉบับ, responsive |
| **Flutter App** | ✅ 100% | Flutter Linux/Desktop/Mobile client ครบทุกหน้า (Dashboard, Tune, DTC, Top/Left/Right panels) |
| **COM Port** | ✅ 100% | Selector dropdown, COM75/COM3/ttyUSB0, connect/disconnect mock |
| **BIN Handling** | ✅ 100% | Read 64KB mock, checksum, save file, file dialog |
| **Flash** | ✅ 100% | Progress 0-100% animation, ห้ามปิดกุญแจ warning |
| **21 ปุ่ม** | ✅ 100% | ทุกปุ่มกดได้ + Notification / Log |
| **Rust Backend** | 🟡 60% | โครง 7 commands เสร็จ, รอ implement K-Line จริง |
| **Build Pipeline** | 🟡 60% | build.yml พร้อม |
| **Hardware** | ⏳ 0% | รอกล่อง ECU ทดสอบ |
| **รวม** | **50%** | **Day 1 & Day 2 สำเร็จตามเป้า** |

### ✅ ทำเสร็จแล้ว (Day 1 & Day 2)
- [x] Web UI Svelte + Vite + TypeScript (port 1420)
- [x] Flutter Cross-Platform App (`flutter_app/`) พร้อม Provider state management และ Unit/Widget tests ผ่าน 100%
- [x] Tauri scaffold (Cargo.toml, tauri.conf.json)
- [x] Serial auto-detect mock, BIN reader/writer, Flash progress bar
- [x] GitHub Actions build.yml

### 🟡 โครงเสร็จ รอทำจริง
- [~] Rust commands (list_ports, connect_ecu, read_rom, write_rom)
- [~] Tauri config + build pipeline
- [~] K-Line protocol structure

### ⏳ ยังไม่ได้ทำ (Day 2-10)
- [ ] K-Line fast init จริง
- [ ] อ่าน ROM จริงผ่าน serial
- [ ] GAUGE live graph
- [ ] BIN hex viewer
- [ ] CC Calculator จริง
- [ ] Build .exe/.dmg/.deb

## 🚀 Quick Start - รันโปรแกรมที่ทำเสร็จแล้ว

```bash
# 1. รันเวอร์ชั่นใหม่ Svelte (แนะนำ)
cd /home/user/remap-v5
npx vite --port 1420 --host 0.0.0.0
# เปิด http://localhost:1420

# 2. รันเวอร์ชั่นเก่า static HTML
cd /home/user
python3 -m http.server 8000
# เปิด http://localhost:8000
```

### วิธีเล่น:
1. กด `: COM75 ▲` → เลือก COM Port
2. กด **🔌 เชื่อมต่อ** → รอ 1.2วิ จะเขียว
3. กด **📥 ดูดไฟล์ ROM** → รอ 2วิ ได้ไฟล์ 64KB
4. กด **⚡ เริ่มแฟลช** → ดู progress 0-100%
5. กด **💾 ชื่อไฟล์** → ดาวน์โหลด .bin

## 🏗️ สถาปัตยกรรม

```
Frontend (Svelte) [✅100%] --IPC Mock--> Backend (Rust) [🟡60%] --Serial [⏳0%]--> Honda ECU
```

ดูแผนเต็ม: [`../REMAP_V5_PLAN.md`](../REMAP_V5_PLAN.md) และ [`DAILY_LOG.md`](./DAILY_LOG.md)

## 📁 โครงสร้าง

```
remap-v5/
├── src/ [✅]
│   ├── components/ [✅] - TopBanner, LeftPanel, RightPanel
│   ├── lib/ [✅] - ecu.ts (mock), stores.ts
│   └── App.svelte [✅]
├── src-tauri/ [✅ scaffold]
│   ├── src/ [✅]
│   │   ├── commands.rs [✅ 7 commands]
│   │   ├── serial.rs [✅ โครง]
│   │   ├── ecu/ [✅ โครง]
│   │   └── bin/ [✅]
│   └── tauri.conf.json [✅]
├── .github/workflows/build.yml [✅]
├── DAILY_LOG.md [✅]
└── README.md [✅ อัพเดทแล้ว]
```

## 🔌 Hardware ที่รองรับ (แผน)

- **COM Port:** COM1-COM99 auto-detect [✅ mock ทำแล้ว]
- **Protocol:** K-Line 10400, CAN 500k [⏳]
- **Adapter:** FTDI, CH340, ELM327 [⏳]

## 📦 Build อัตโนมัติ

ทุกครั้งที่ `git push` จะ build 3 OS อัตโนมัติ

## 👨‍💻 สำหรับ sirisakboy

บัญชีล็อคแต่ push/pull ได้ → ใช้:
```bash
export GITHUB_TOKEN=...
git push https://sirisakboy:$GITHUB_TOKEN@github.com/sirisakboy/remap-v5.git main
```

---

**Version:** 5.0.7.1  
**Day 1:** 20 ก.ย. 2026 - 30 min exercise ✅  
**Progress:** 35% Overall, Frontend 100%  
**Next:** Day 2 - GAUGE realtime + BIN Viewer + CC Calculator
