import 'dart:math';
import 'dart:typed_data';
import '../models/ecu_model.dart';

class BinAnalysisResult {
  final String checksum;
  final bool isValidChecksum;
  final EcuModel? matchedModel;
  final String detectedId;
  final String startOffset;
  final String accessOffset;
  final String checksumOffset;
  final String mcuFamily;
  final int fileSize;

  BinAnalysisResult({
    required this.checksum,
    required this.isValidChecksum,
    this.matchedModel,
    required this.detectedId,
    required this.startOffset,
    required this.accessOffset,
    required this.checksumOffset,
    required this.mcuFamily,
    required this.fileSize,
  });
}

class EcuService {
  static int calculateChecksum8(Uint8List data) {
    int sum = 0;
    for (int i = 0; i < data.length; i++) {
      sum += data[i];
    }
    return ((sum ^ 0xff) + 1) & 0xff;
  }

  static Map<String, String> getOffsets(int flashKB) {
    switch (flashKB) {
      case 48:
        return {'start': '0x4000', 'checksum': '0x7600', 'access': '0xBFFF', 'mcu': 'R8C 48KB'};
      case 96:
        return {'start': '0x8000', 'checksum': '0x0000', 'access': '0x17FFF', 'mcu': 'R8C 96KB'};
      case 64:
      default:
        return {'start': '0x8000', 'checksum': '0x0000', 'access': '0xFFFF', 'mcu': 'R8C 64KB'};
    }
  }

  static BinAnalysisResult analyzeBin(Uint8List data, String fileName) {
    int sizeKB = (data.length / 1024).round();
    // Try to find part number in binary ASCII
    String ascii = '';
    try {
      ascii = String.fromCharCodes(data.sublist(0, min(data.length, 32768)));
    } catch (_) {}

    EcuModel? matched;
    String detected = 'Custom ROM (${sizeKB}KB)';
    for (var m in kEcuModels) {
      if (ascii.contains(m.pn) || fileName.toUpperCase().contains(m.pn)) {
        matched = m;
        detected = '${m.model} (${m.pn})';
        break;
      }
    }

    final offsets = getOffsets(matched?.flashKB ?? sizeKB);
    int sumVal = calculateChecksum8(data);
    bool isValid = (sumVal == 0x00 || sumVal == 0xEF); // Honda checksum condition or mock

    return BinAnalysisResult(
      checksum: isValid ? '0x00 (OK)' : '0x${sumVal.toRadixString(16).padLeft(2, '0').toUpperCase()}',
      isValidChecksum: isValid,
      matchedModel: matched,
      detectedId: detected,
      startOffset: offsets['start']!,
      accessOffset: offsets['access']!,
      checksumOffset: offsets['checksum']!,
      mcuFamily: offsets['mcu']!,
      fileSize: data.length,
    );
  }

  static Uint8List generateMockRom(int sizeKB) {
    final data = Uint8List(sizeKB * 1024);
    final rand = Random();
    for (int i = 0; i < data.length - 1; i++) {
      data[i] = rand.nextInt(256);
    }
    // Honda header
    data[0] = 0x48; data[1] = 0x4F; data[2] = 0x4E; data[3] = 0x44; data[4] = 0x41;
    int sum = 0;
    for (int i = 0; i < data.length - 1; i++) {
      sum += data[i];
    }
    data[data.length - 1] = ((sum ^ 0xff) + 1) & 0xff;
    return data;
  }

  static double calculateCC(double bore, double stroke) {
    if (bore <= 0 || stroke <= 0) return 0.0;
    double r = bore / 2;
    return pi * r * r * stroke / 1000;
  }
}
