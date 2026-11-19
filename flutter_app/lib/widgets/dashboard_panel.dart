import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';

class DashboardPanel extends StatelessWidget {
  const DashboardPanel({super.key});

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Flash Control Banner
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: const Color(0xFF1a0505),
              border: Border.all(color: const Color(0xFFff3333)),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Row(
              children: [
                const Icon(Icons.warning_amber_rounded, color: Color(0xFFff5f56), size: 32),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: const [
                      Text(
                        '⚡ ระบบแฟลชกล่อง ECU (Flash ROM)',
                        style: TextStyle(color: Color(0xFFffaa55), fontWeight: FontWeight.bold, fontSize: 13),
                      ),
                      Text(
                        'คำเตือน: ห้ามปิดสวิตช์กุญแจ หรือถอดสาย USB ระหว่างกำลังแฟลชเด็ดขาด!',
                        style: TextStyle(color: Color(0xFFa0d8ff), fontSize: 11),
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 12),
                ElevatedButton.icon(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFFcc0000),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  ),
                  onPressed: state.isFlashing ? null : () => state.flashRom(),
                  icon: const Icon(Icons.flash_on),
                  label: Text(state.isFlashing ? 'กำลังแฟลช...' : 'เริ่มแฟลช (FLASH)'),
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),

          if (state.isFlashing) ...[
            LinearProgressIndicator(
              value: state.flashProgress,
              backgroundColor: const Color(0xFF1a3a5a),
              color: const Color(0xFF00ffaa),
              minHeight: 8,
            ),
            const SizedBox(height: 12),
          ],

          // Live Gauges Grid
          const Text(
            '📊 สถานะเครื่องยนต์เรียลไทม์ (Live Telemetry)',
            style: TextStyle(color: Color(0xFF00ffff), fontWeight: FontWeight.bold, fontSize: 12),
          ),
          const SizedBox(height: 8),

          Expanded(
            child: GridView.count(
              crossAxisCount: 3,
              mainAxisSpacing: 10,
              crossAxisSpacing: 10,
              childAspectRatio: 1.6,
              children: [
                _buildGaugeCard('รอบเครื่อง (RPM)', '${state.rpm} RPM', Icons.speed, const Color(0xFF00ffaa)),
                _buildGaugeCard('ลิ้นเร่ง (TPS)', '${state.tps.toStringAsFixed(1)} %', Icons.settings_input_component, const Color(0xFF00aaff)),
                _buildGaugeCard('อุณหภูมิเครื่อง (ECT)', '${state.ect.toStringAsFixed(0)} °C', Icons.thermostat, const Color(0xFFffaa55)),
                _buildGaugeCard('อัตราส่วนผสม (AFR)', state.afr.toStringAsFixed(1), Icons.science, const Color(0xFFff5f56)),
                _buildGaugeCard('หัวฉีด (Injector)', '${state.injectorMs.toStringAsFixed(1)} ms', Icons.opacity, const Color(0xFFcc7aff)),
                _buildGaugeCard('แรงดันแบตเตอรี่', '${state.batteryV.toStringAsFixed(1)} V', Icons.battery_charging_full, const Color(0xFFffff55)),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildGaugeCard(String title, String value, IconData icon, Color color) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFF05101f),
        border: Border.all(color: color.withValues(alpha: 0.5)),
        borderRadius: BorderRadius.circular(8),
        boxShadow: [
          BoxShadow(color: color.withValues(alpha: 0.1), blurRadius: 6, spreadRadius: 1),
        ],
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, color: color, size: 16),
              const SizedBox(width: 6),
              Text(title, style: const TextStyle(color: Color(0xFF7a9ab8), fontSize: 11)),
            ],
          ),
          const SizedBox(height: 8),
          Text(
            value,
            style: TextStyle(color: color, fontSize: 20, fontWeight: FontWeight.bold),
          ),
        ],
      ),
    );
  }
}
