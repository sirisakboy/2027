import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';

class TunePanel extends StatefulWidget {
  const TunePanel({super.key});

  @override
  State<TunePanel> createState() => _TunePanelState();
}

class _TunePanelState extends State<TunePanel> {
  double limiterPct = 20.0;
  double fuelPct = 20.0;
  double ignDeg = 3.0;
  bool e85Mode = false;
  bool o2Off = false;
  bool popBangs = false;

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: ListView(
        children: [
          const Text(
            '🛠️ โหมดปรับแต่งตารางน้ำมันและไฟจุดระเบิด (ECU Tuning Studio)',
            style: TextStyle(color: Color(0xFF00ffff), fontWeight: FontWeight.bold, fontSize: 13),
          ),
          const SizedBox(height: 12),

          // Presets Card
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: const Color(0xFF05101f),
              border: Border.all(color: const Color(0xFF1a3a5a)),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('🔥 เลือกสไตล์การจูนสำเร็จรูป (Preset Tunes)', style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                Wrap(
                  spacing: 8,
                  runSpacing: 8,
                  children: [
                    _presetButton('🏁 สเต็ปใช้งานทั่วไป (Street)', () => state.setLog('🔧 ใช้สเต็ป Street แล้ว')),
                    _presetButton('⚡ สเต็ปเรณู/สนาม (Racing Open)', () => state.setLog('🔧 ใช้สเต็ป Racing Open แล้ว')),
                    _presetButton('⛽ สเต็ปน้ำมัน E85', () => state.setLog('🔧 ใช้สเต็ป E85 แล้ว')),
                  ],
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),

          // Sliders Card
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: const Color(0xFF05101f),
              border: Border.all(color: const Color(0xFF1a3a5a)),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('⚙️ ปรับแต่งค่าละเอียดย่อย (Fine Tuning)', style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11, fontWeight: FontWeight.bold)),
                const SizedBox(height: 12),
                _sliderRow('ปลดรอบเครื่อง (RPM Limiter):', limiterPct, 0, 50, '%', (v) => setState(() => limiterPct = v)),
                _sliderRow('เพิ่มจ่ายน้ำมัน (Fuel Trim):', fuelPct, -30, 50, '%', (v) => setState(() => fuelPct = v)),
                _sliderRow('เพิ่มองศาไฟจุดระเบิด (Ignition Adv):', ignDeg, -5, 10, 'deg', (v) => setState(() => ignDeg = v)),
              ],
            ),
          ),
          const SizedBox(height: 12),

          // Switches Card
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: const Color(0xFF05101f),
              border: Border.all(color: const Color(0xFF1a3a5a)),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('⚡ ฟังก์ชั่นพิเศษ (Advanced Options)', style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 11, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                SwitchListTile(
                  title: const Text('โหมดน้ำมัน E85 (Advanced Map)', style: TextStyle(color: Colors.white, fontSize: 12)),
                  value: e85Mode,
                  onChanged: (v) => setState(() => e85Mode = v),
                  activeColor: const Color(0xFF00ffff),
                  dense: true,
                ),
                SwitchListTile(
                  title: const Text('ปิดเซ็นเซอร์ออกซิเจน (O2 Sensor OFF)', style: TextStyle(color: Colors.white, fontSize: 12)),
                  value: o2Off,
                  onChanged: (v) => setState(() => o2Off = v),
                  activeColor: const Color(0xFF00ffff),
                  dense: true,
                ),
                SwitchListTile(
                  title: const Text('เบิ้ลตัด / ป๊อปแบงค์ (Pop & Bangs)', style: TextStyle(color: Colors.white, fontSize: 12)),
                  value: popBangs,
                  onChanged: (v) => setState(() => popBangs = v),
                  activeColor: const Color(0xFF00ffff),
                  dense: true,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _presetButton(String label, VoidCallback onTap) {
    return ElevatedButton(
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF0a3a6a),
        foregroundColor: Colors.white,
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        textStyle: const TextStyle(fontSize: 11, fontWeight: FontWeight.bold),
      ),
      onPressed: onTap,
      child: Text(label),
    );
  }

  Widget _sliderRow(String label, double val, double min, double max, String unit, ValueChanged<double> onChanged) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(label, style: const TextStyle(color: Color(0xFFa0d8ff), fontSize: 11)),
            Text('${val > 0 && label.contains('Trim') ? '+' : ''}${val.toStringAsFixed(1)} $unit', style: const TextStyle(color: Color(0xFF00ffff), fontSize: 11, fontWeight: FontWeight.bold)),
          ],
        ),
        Slider(
          value: val,
          min: min,
          max: max,
          divisions: 50,
          activeColor: const Color(0xFF00ffff),
          inactiveColor: const Color(0xFF1a3a5a),
          onChanged: onChanged,
        ),
      ],
    );
  }
}
