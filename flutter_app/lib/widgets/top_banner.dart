import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';
import '../models/ecu_model.dart';

class TopBanner extends StatelessWidget {
  const TopBanner({super.key});

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
      decoration: BoxDecoration(
        color: const Color(0xFF0a1a2f),
        border: const Border(bottom: BorderSide(color: Color(0xFF1a3a5a), width: 1)),
        gradient: LinearGradient(
          colors: [const Color(0xFF0a1a2f), const Color(0xFF000000)],
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
        ),
      ),
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          children: [
            // Logo & Title
            const Icon(Icons.bolt, color: Color(0xFF00aaff), size: 28),
            const SizedBox(width: 8),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [
                Text(
                  'REMOTE REMAP V.5 (FLUTTER LINUX EDITION)',
                  style: TextStyle(color: Color(0xFF00ffff), fontWeight: FontWeight.bold, fontSize: 13),
                ),
                Text(
                  'Honda ECU Keihin & Shindengen Tuning Studio',
                  style: TextStyle(color: Color(0xFF7a9ab8), fontSize: 10),
                ),
              ],
            ),
            const SizedBox(width: 20),

            // ECU Model Selector
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: const Color(0xFF05101f),
                border: Border.all(color: const Color(0xFF1a3a5a)),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Row(
                children: [
                  const Text('เลือกโมเดล ECU: ', style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11)),
                  const SizedBox(width: 4),
                  DropdownButton<String>(
                    value: state.selectedModelPn,
                    dropdownColor: const Color(0xFF0a1a2f),
                    style: const TextStyle(color: Colors.white, fontSize: 11),
                    underline: const SizedBox(),
                    items: kEcuModels.map((m) {
                      return DropdownMenuItem<String>(
                        value: m.pn,
                        child: Text('${m.model} (${m.pn})', style: const TextStyle(color: Colors.white, fontSize: 11)),
                      );
                    }).toList(),
                    onChanged: (val) {
                      if (val != null) {
                        state.setSelectedModel(val);
                      }
                    },
                  ),
                ],
              ),
            ),
            const SizedBox(width: 12),

            // COM Port Selector
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: const Color(0xFF05101f),
                border: Border.all(color: const Color(0xFF1a3a5a)),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Row(
                children: [
                  const Text('COM: ', style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11)),
                  DropdownButton<String>(
                    value: state.comPort,
                    dropdownColor: const Color(0xFF0a1a2f),
                    style: const TextStyle(color: Colors.white, fontSize: 11),
                    underline: const SizedBox(),
                    items: ['/dev/ttyUSB0', '/dev/ttyACM0', 'COM75', 'COM3'].map((port) {
                      return DropdownMenuItem(value: port, child: Text(port, style: const TextStyle(color: Colors.white, fontSize: 11)));
                    }).toList(),
                    onChanged: (p) => p != null ? state.setComPort(p) : null,
                  ),
                ],
              ),
            ),
            const SizedBox(width: 8),

            // Connect / Disconnect Button
            ElevatedButton.icon(
              style: ElevatedButton.styleFrom(
                backgroundColor: state.isConnected ? const Color(0xFFff3333) : const Color(0xFF0088cc),
                foregroundColor: Colors.white,
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                textStyle: const TextStyle(fontSize: 11, fontWeight: FontWeight.bold),
              ),
              onPressed: () {
                if (state.isConnected) {
                  state.disconnectEcu();
                } else {
                  state.connectEcu();
                }
              },
              icon: Icon(state.isConnected ? Icons.power_off : Icons.power, size: 14),
              label: Text(state.isConnected ? 'ตัดการเชื่อมต่อ' : 'เชื่อมต่อ ECU'),
            ),
          ],
        ),
      ),
    );
  }
}
