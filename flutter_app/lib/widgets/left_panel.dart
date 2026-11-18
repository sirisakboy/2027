import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/ecu_state.dart';

class LeftPanel extends StatelessWidget {
  const LeftPanel({super.key});

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    final menuItems = [
      {'id': 'dashboard', 'label': 'DASHBOARD', 'icon': Icons.home},
      {'id': 'tune', 'label': 'TUNE', 'icon': Icons.build},
      {'id': 'dtc', 'label': 'DTC', 'icon': Icons.search},
    ];

    return Container(
      width: 110,
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          colors: [Color(0xFF0a1a2f), Color(0xFF000000)],
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
        ),
        border: Border(right: BorderSide(color: Color(0xFF1a3a5a), width: 1)),
      ),
      padding: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'เมนูหลัก',
            style: TextStyle(color: Color(0xFF7ad4ff), fontSize: 10, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          ...menuItems.map((item) {
            bool isActive = state.currentView == item['id'];
            return Padding(
              padding: const EdgeInsets.only(bottom: 6),
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: isActive ? const Color(0xFF0088cc) : const Color(0xFF0a3a6a),
                  foregroundColor: isActive ? Colors.white : const Color(0xFFa0d8ff),
                  elevation: isActive ? 4 : 0,
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 8),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(4),
                    side: BorderSide(color: isActive ? const Color(0xFF00ffff) : const Color(0xFF00aaff)),
                  ),
                ),
                onPressed: () => state.setView(item['id'] as String),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(item['icon'] as IconData, size: 16),
                    const SizedBox(height: 4),
                    Text(
                      item['label'] as String,
                      style: const TextStyle(fontSize: 10, fontWeight: FontWeight.bold),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
            );
          }),
          const Spacer(),
          // Connection status badge
          Container(
            padding: const EdgeInsets.all(6),
            decoration: BoxDecoration(
              color: state.isConnected ? const Color(0xFF003319) : const Color(0xFF330a0a),
              border: Border.all(color: state.isConnected ? const Color(0xFF00ffaa) : const Color(0xFFff5f56)),
              borderRadius: BorderRadius.circular(4),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Container(
                  width: 6,
                  height: 6,
                  decoration: BoxDecoration(
                    color: state.isConnected ? const Color(0xFF00ffaa) : const Color(0xFFff5f56),
                    shape: BoxShape.circle,
                  ),
                ),
                const SizedBox(width: 4),
                Text(
                  state.isConnected ? 'ออนไลน์' : 'ออฟไลน์',
                  style: TextStyle(
                    color: state.isConnected ? const Color(0xFF7affaa) : const Color(0xFF7a9ab8),
                    fontSize: 9,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
