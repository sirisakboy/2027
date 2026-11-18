import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'providers/ecu_state.dart';
import 'widgets/top_banner.dart';
import 'widgets/left_panel.dart';
import 'widgets/right_panel.dart';
import 'widgets/dashboard_panel.dart';
import 'widgets/tune_panel.dart';
import 'widgets/dtc_panel.dart';

void main() {
  runApp(const RemapApp());
}

class RemapApp extends StatelessWidget {
  const RemapApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => EcuState()),
      ],
      child: MaterialApp(
        title: 'REMOTE REMAP V.5 (Flutter Linux Edition)',
        debugShowCheckedModeBanner: false,
        theme: ThemeData(
          brightness: Brightness.dark,
          scaffoldBackgroundColor: const Color(0xFF020813),
          primaryColor: const Color(0xFF00aaff),
          colorScheme: const ColorScheme.dark(
            primary: Color(0xFF00aaff),
            secondary: Color(0xFF00ffff),
            surface: Color(0xFF05101f),
          ),
          fontFamily: 'monospace',
        ),
        home: const MainHomePage(),
      ),
    );
  }
}

class MainHomePage extends StatelessWidget {
  const MainHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    final state = context.watch<EcuState>();

    Widget centerWidget;
    switch (state.currentView) {
      case 'tune':
        centerWidget = const TunePanel();
        break;
      case 'dtc':
        centerWidget = const DtcPanel();
        break;
      case 'dashboard':
      default:
        centerWidget = const DashboardPanel();
        break;
    }

    return Scaffold(
      body: Column(
        children: [
          const TopBanner(),
          Expanded(
            child: Row(
              children: [
                const LeftPanel(),
                Expanded(child: centerWidget),
                const RightPanel(),
              ],
            ),
          ),
          // Bottom Status Log Bar
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: const BoxDecoration(
              color: Color(0xFF020813),
              border: Border(top: BorderSide(color: Color(0xFF1a3a5a), width: 1)),
            ),
            child: Row(
              children: [
                const Icon(Icons.terminal, color: Color(0xFF00ffaa), size: 14),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    state.logMessage,
                    style: const TextStyle(color: Color(0xFFa0d8ff), fontSize: 11),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                const Text(
                  'REMAP V.5 • Linux Flutter',
                  style: TextStyle(color: Color(0xFF7a9ab8), fontSize: 10),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
