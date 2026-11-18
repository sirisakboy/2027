import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';
import '../services/ecu_service.dart';

class RightPanel extends StatefulWidget {
  const RightPanel({super.key});

  @override
  State<RightPanel> createState() => _RightPanelState();
}

class _RightPanelState extends State<RightPanel> {
  bool showCC = false;
  double bore = 52.4;
  double stroke = 57.9;

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();
    double ccResult = EcuService.calculateCC(bore, stroke);

    return Container(
      width: 260,
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          colors: [Color(0xFF000000), Color(0xFF0a1a2f)],
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
        ),
        border: Border(left: BorderSide(color: Color(0xFF1a3a5a), width: 1)),
      ),
      padding: const EdgeInsets.all(10),
      child: ListView(
        children: [
          const Text(
            'เครื่องมือ & จัดการไฟล์ ROM',
            style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),

          // Action Buttons Grid
          GridView.count(
            crossAxisCount: 2,
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            mainAxisSpacing: 6,
            crossAxisSpacing: 6,
            childAspectRatio: 2.2,
            children: [
              _buildToolButton(
                icon: Icons.download,
                label: 'ดูดไฟล์ ROM',
                onTap: () => state.readRom(),
              ),
              _buildToolButton(
                icon: Icons.folder_open,
                label: 'เลือกไฟล์ BIN',
                onTap: () {
                  // Simulate loading a bin
                  final bin = EcuService.generateMockRom(64);
                  state.applyBin(bin, 'Wave110i_Stock_64K.bin', '📁 เลือกไฟล์ BIN');
                },
              ),
              _buildToolButton(
                icon: Icons.save,
                label: 'บันทึก BIN',
                onTap: () {
                  if (state.selectedBin == null) {
                    state.setLog('⚠️ ยังไม่มีไฟล์ BIN ให้บันทึก');
                  } else {
                    state.setLog('💾 บันทึกไฟล์ ${state.selectedFileName}_TUNED.bin สำเร็จ!');
                  }
                },
              ),
              _buildToolButton(
                icon: Icons.calculate,
                label: 'คำนวณ CC',
                onTap: () => setState(() => showCC = !showCC),
              ),
            ],
          ),
          const SizedBox(height: 12),

          // ROM Info Card
          Container(
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: const Color(0xFF05101f),
              border: Border.all(color: const Color(0xFF1a3a5a)),
              borderRadius: BorderRadius.circular(6),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  '📊 ข้อมูลรอม (ROM Metadata)',
                  style: TextStyle(color: Color(0xFF00ffff), fontSize: 11, fontWeight: FontWeight.bold),
                ),
                const Divider(color: Color(0xFF1a3a5a)),
                _metaRow('ไฟล์ปัจจุบัน:', state.selectedFileName),
                _metaRow('ไฟล์แนะนำ:', state.recommendedBinName),
                _metaRow('ขนาดไฟล์:', state.fileSize),
                _metaRow('Start Offset:', state.startOffset),
                _metaRow('Access Offset:', state.accessOffset),
                _metaRow('Checksum Offset:', state.checksumOffset),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text('Checksum:', style: TextStyle(color: Color(0xFF7a9ab8), fontSize: 11)),
                    Text(
                      state.binChecksum,
                      style: TextStyle(
                        color: state.binChecksumValid ? const Color(0xFF7affaa) : const Color(0xFFffaa55),
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                _metaRow('ECU Model:', state.ecmId),
              ],
            ),
          ),

          if (showCC) ...[
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: const Color(0xFF0a2a1f),
                border: Border.all(color: const Color(0xFF00ffaa)),
                borderRadius: BorderRadius.circular(6),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    '🧮 คำนวณความจุกระบอกสูบ (CC)',
                    style: TextStyle(color: Color(0xFF7affaa), fontSize: 11, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 8),
                  Row(
                    children: [
                      const Expanded(child: Text('ลูกสูบ Bore (mm):', style: TextStyle(color: Colors.white, fontSize: 10))),
                      SizedBox(
                        width: 70,
                        child: TextField(
                          style: const TextStyle(color: Colors.white, fontSize: 11),
                          controller: TextEditingController(text: bore.toString()),
                          onChanged: (v) => setState(() => bore = double.tryParse(v) ?? bore),
                          decoration: const InputDecoration(isDense: true, contentPadding: EdgeInsets.all(4)),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      const Expanded(child: Text('ช่วงชัก Stroke (mm):', style: TextStyle(color: Colors.white, fontSize: 10))),
                      SizedBox(
                        width: 70,
                        child: TextField(
                          style: const TextStyle(color: Colors.white, fontSize: 11),
                          controller: TextEditingController(text: stroke.toString()),
                          onChanged: (v) => setState(() => stroke = double.tryParse(v) ?? stroke),
                          decoration: const InputDecoration(isDense: true, contentPadding: EdgeInsets.all(4)),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Center(
                    child: Text(
                      '${ccResult.toStringAsFixed(1)} cc',
                      style: const TextStyle(color: Color(0xFF00ffff), fontSize: 16, fontWeight: FontWeight.bold),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildToolButton({required IconData icon, required String label, required VoidCallback onTap}) {
    return ElevatedButton(
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF0a3a6a),
        foregroundColor: const Color(0xFFa0d8ff),
        padding: EdgeInsets.zero,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(4),
          side: const BorderSide(color: Color(0xFF00aaff)),
        ),
      ),
      onPressed: onTap,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(icon, size: 16, color: const Color(0xFF00ffff)),
          const SizedBox(height: 2),
          Text(label, style: const TextStyle(fontSize: 10, fontWeight: FontWeight.bold)),
        ],
      ),
    );
  }

  Widget _metaRow(String label, String val) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 2),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(color: Color(0xFF7a9ab8), fontSize: 11)),
          Flexible(
            child: Text(
              val,
              style: const TextStyle(color: Colors.white, fontSize: 11, fontWeight: FontWeight.bold),
              overflow: TextOverflow.ellipsis,
              textAlign: TextAlign.right,
            ),
          ),
        ],
      ),
    );
  }
}
