class EcuModel {
  final String model;
  final String years;
  final String pn;
  final String? cal;
  final String vendor;
  final int flashKB;
  final String gearbox;

  const EcuModel({
    required this.model,
    required this.years,
    required this.pn,
    this.cal,
    this.vendor = 'Keihin',
    this.flashKB = 64,
    required this.gearbox,
  });
}

final List<EcuModel> kEcuModels = [
  const EcuModel(model: 'Wave110i', years: '2013-2016', pn: '38770-K03-H01', cal: 'K03S10A', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'Wave110i', years: '2011-2012', pn: '38770-KWW-643', cal: 'KWWM30D', vendor: 'Keihin', flashKB: 48, gearbox: 'geared'),
  const EcuModel(model: 'Wave110i', years: '2016-2017', pn: '38770-K58-T81', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'Wave110i', years: '2021', pn: '38770-K2J-T01', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'Wave125', years: '2012-2013', pn: '38770-KYZ-T03', cal: 'KWWM30D', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'Dream Super Cub', years: '2019', pn: '38770-K76-T62', cal: 'K76L102', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'MSX125', years: '2013-2015', pn: '38770-K26-901', cal: 'KWWM404', vendor: 'Keihin', flashKB: 48, gearbox: 'geared'),
  const EcuModel(model: 'MSX125 SF', years: '2016-2019', pn: '38770-K26-B13', cal: 'K03S203', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'CBR150R', years: '2017-2018', pn: '38770-K45-N42', cal: 'K45G103', vendor: 'Keihin', flashKB: 64, gearbox: 'geared'),
  const EcuModel(model: 'Monkey125', years: '2018-2019', pn: '38770-K0F-A01', cal: 'K0FA103', vendor: 'Keihin', flashKB: 96, gearbox: 'geared'),
  const EcuModel(model: 'CT125', years: '2020', pn: '38770-K2E-T02', vendor: 'Keihin', flashKB: 96, gearbox: 'geared'),
  const EcuModel(model: 'Scoopy i', years: '2018', pn: '38770-K16-B61', vendor: 'Keihin', flashKB: 64, gearbox: 'auto'),
  const EcuModel(model: 'Zoomer-X', years: '2013', pn: '38770-K20-T01', vendor: 'Keihin', flashKB: 64, gearbox: 'auto'),
  const EcuModel(model: 'PCX150', years: '2017-2019', pn: '30400-K96-V01', vendor: 'Shindengen', flashKB: 64, gearbox: 'auto'),
  const EcuModel(model: 'ADV150', years: '2019-2021', pn: '30400-K0WA-N01', vendor: 'Shindengen', flashKB: 64, gearbox: 'auto'),
];

EcuModel? findModelByPn(String pn) {
  try {
    return kEcuModels.firstWhere((m) => m.pn.toUpperCase() == pn.toUpperCase());
  } catch (_) {
    return null;
  }
}
