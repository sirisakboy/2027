import 'package:flutter/foundation.dart';
import '../models/ecu_model.dart';
import '../services/ecu_service.dart';

class EcuState extends ChangeNotifier {
  String comPort = '/dev/ttyUSB0';
  bool isConnected = false;
  bool isConnecting = false;
  String currentView = 'dashboard'; // 'dashboard', 'tune', 'dtc'
  
  String selectedModelPn = '38770-K03-H01';
  String ecmId = 'Wave110i (38770-K03-H01)';
  String fileSize = '64KB (65536 bytes)';
  String startOffset = '0x8000';
  String accessOffset = '0xFFFF';
  String checksumOffset = '0x0000';
  String binChecksum = '0x00 (OK)';
  bool binChecksumValid = true;
  String recommendedBinName = '38770-K03-H01-Wave110i.bin';
  String selectedFileName = 'Default_Stock.bin';
  Uint8List? selectedBin;

  String logMessage = '🚀 พร้อมใช้งาน - Remap V.5 Flutter Linux Edition';
  bool isFlashing = false;
  double flashProgress = 0.0;

  // Live Data
  int rpm = 1450;
  double tps = 0.0;
  double ect = 85.0;
  double afr = 14.7;
  double injectorMs = 2.4;
  double ignitionDeg = 12.0;
  double batteryV = 13.8;

  void setView(String view) {
    currentView = view;
    notifyListeners();
  }

  void setComPort(String port) {
    comPort = port;
    notifyListeners();
  }

  void setSelectedModel(String val) {
    selectedModelPn = val;
    final m = findModelByPn(val);
    if (m != null) {
      ecmId = '${m.model} (${m.pn})';
      recommendedBinName = '${m.pn}-${m.model}.bin';
    }
    notifyListeners();
  }

  void setLog(String msg) {
    logMessage = msg;
    notifyListeners();
  }

  Future<void> connectEcu() async {
    isConnecting = true;
    notifyListeners();
    setLog('🔌 กำลังเชื่อมต่อ $comPort (K-Line 10400 Baud)...');
    await Future.delayed(const Duration(milliseconds: 900));
    isConnecting = false;
    isConnected = true;
    setLog('✅ เชื่อมต่อ $comPort สำเร็จ! ECU พร้อมทำงาน');
    notifyListeners();
  }

  void disconnectEcu() {
    isConnected = false;
    setLog('🔌 ตัดการเชื่อมต่อ ECU แล้ว');
    notifyListeners();
  }

  Future<void> readRom() async {
    if (!isConnected) {
      setLog('⚠️ กรุณาเชื่อมต่อ ECU ก่อนดูดไฟล์ ROM');
      return;
    }
    setLog('📥 กำลังดูดไฟล์ ROM จาก ECU (64KB)...');
    await Future.delayed(const Duration(seconds: 1));
    final bin = EcuService.generateMockRom(64);
    applyBin(bin, 'KEIHIN-64KB-DUMP.bin', '✅ ดูดไฟล์ ROM สำเร็จ');
  }

  void applyBin(Uint8List bin, String name, String sourceLabel) {
    selectedBin = bin;
    selectedFileName = name;
    recommendedBinName = name;
    fileSize = '${(bin.length / 1024).round()}KB (${bin.length} bytes)';

    final analysis = EcuService.analyzeBin(bin, name);
    ecmId = analysis.detectedId;
    startOffset = analysis.startOffset;
    accessOffset = analysis.accessOffset;
    checksumOffset = analysis.checksumOffset;
    binChecksum = analysis.checksum;
    binChecksumValid = analysis.isValidChecksum;

    if (analysis.matchedModel != null) {
      selectedModelPn = analysis.matchedModel!.pn;
    }

    setLog('$sourceLabel: ${analysis.detectedId} | Checksum: ${analysis.checksum}');
    notifyListeners();
  }

  Future<void> flashRom() async {
    if (selectedBin == null) {
      setLog('⚠️ กรุณาเลือกหรือดูดไฟล์ BIN ก่อนแฟลช');
      return;
    }
    if (!isConnected) {
      setLog('⚠️ กรุณาเชื่อมต่อสายจูน ECU ก่อนแฟลช');
      return;
    }

    isFlashing = true;
    flashProgress = 0.0;
    setLog('⚡ กำลังแฟลช ROM... ห้ามปิดสวิตช์กุญแจเด็ดขาด!');
    notifyListeners();

    for (int i = 1; i <= 20; i++) {
      await Future.delayed(const Duration(milliseconds: 80));
      flashProgress = i / 20.0;
      notifyListeners();
    }

    isFlashing = false;
    setLog('✅ แฟลชรอมสำเร็จ 100% - ECU ทำงานปกติ');
    notifyListeners();
  }
}
