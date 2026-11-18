import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';

class DtcPanel extends StatefulWidget {
  const DtcPanel({super.key});

  @override
  State<DtcPanel> createState() => _DtcPanelState();
}

class _DtcPanelState extends State<DtcPanel> {
  List<Map<String, String>> dtcList = [
    {'code': 'P0115', 'desc': 'Engine Coolant Temperature Sensor Circuit Malfunction', 'status': 'Active'},
    {'code': 'P0120', 'desc': 'Throttle Position Sensor / Switch A Circuit Malfunction', 'status': 'Stored'},
    {'code': 'P0230', 'desc': 'Fuel Pump Primary Circuit Malfunction', 'status': 'Cleared'},
  ];

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                '🔍 วิเคราะห์รหัสข้อผิดพลาด (DTC - Diagnostic Trouble Codes)',
                style: TextStyle(color: Color(0xFF00ffff), fontWeight: FontWeight.bold, fontSize: 13),
              ),
              Row(
                children: [
                  ElevatedButton.icon(
                    style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF0088cc), foregroundColor: Colors.white),
                    onPressed: () => state.setLog('🔍 อ่าน DTC จากกล่อง ECU สำเร็จ (พบ 2 โค้ด)'),
                    icon: const Icon(Icons.refresh, size: 14),
                    label: const Text('อ่าน DTC', style: TextStyle(fontSize: 11)),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton.icon(
                    style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFFcc5500), foregroundColor: Colors.white),
                    onPressed: () {
                      setState(() => dtcList.clear());
                      state.setLog('🧹 ล้างโค้ด DTC ในกล่อง ECU เรียบร้อยแล้ว');
                    },
                    icon: const Icon(Icons.delete_sweep, size: 14),
                    label: const Text('ล้าง DTC', style: TextStyle(fontSize: 11)),
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 12),

          Expanded(
            child: Container(
              decoration: BoxDecoration(
                color: const Color(0xFF05101f),
                border: Border.all(color: const Color(0xFF1a3a5a)),
                borderRadius: BorderRadius.circular(8),
              ),
              child: dtcList.isEmpty
                  ? const Center(
                      child: Text(
                        '✨ ไม่พบรหัสข้อผิดพลาด (No DTC Errors Found)',
                        style: TextStyle(color: Color(0xFF7affaa), fontSize: 13, fontWeight: FontWeight.bold),
                      ),
                    )
                  : ListView.separated(
                      itemCount: dtcList.length,
                      separatorBuilder: (_, __) => const Divider(color: Color(0xFF1a3a5a), height: 1),
                      itemBuilder: (context, index) {
                        final dtc = dtcList[index];
                        return ListTile(
                          leading: const Icon(Icons.error_outline, color: Color(0xFFffaa55)),
                          title: Text(dtc['code']!, style: const TextStyle(color: Color(0xFF00ffff), fontWeight: FontWeight.bold, fontSize: 13)),
                          subtitle: Text(dtc['desc']!, style: const TextStyle(color: Color(0xFFa0d8ff), fontSize: 11)),
                          trailing: Container(
                            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                            decoration: BoxDecoration(
                              color: const Color(0xFF331a00),
                              border: Border.all(color: const Color(0xFFffaa55)),
                              borderRadius: BorderRadius.circular(4),
                            ),
                            child: Text(dtc['status']!, style: const TextStyle(color: Color(0xFFffaa55), fontSize: 10, fontWeight: FontWeight.bold)),
                          ),
                        );
                      },
                    ),
            ),
          ),
        ],
      ),
    );
  }
}
