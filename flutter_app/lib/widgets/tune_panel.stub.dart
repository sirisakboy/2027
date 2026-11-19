import 'package:flutter/material.dart';

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
    return const Placeholder();
  }
}
