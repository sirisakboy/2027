// Curated Thai-market ECU database.
// ONLY verified facts: part numbers + calibrations from the PDF table,
// flash sizes from real files in resources/bins, vendors from the PDF.
// Fields left out when unverified (no guessing).

export type Gearbox = 'geared' | 'auto'

export interface EcuModel {
  model: string
  years: string
  pn: string
  cal?: string
  vendor?: 'Keihin' | 'Shindengen'
  flashKB?: number
  gearbox: Gearbox
}

export const ECU_MODELS: EcuModel[] = [
  // ---- geared: Wave family ----
  { model: 'Wave110i', years: '2013-2016', pn: '38770-K03-H01', cal: 'K03S10A', vendor: 'Keihin', flashKB: 64, gearbox: 'geared' },
  { model: 'Wave110i', years: '2011-2012', pn: '38770-KWW-643', cal: 'KWWM30D', flashKB: 48, gearbox: 'geared' },
  { model: 'Wave110i', years: '2016-2017', pn: '38770-K58-T81', flashKB: 64, gearbox: 'geared' },
  { model: 'Wave110i', years: '2021', pn: '38770-K2J-T01', flashKB: 64, gearbox: 'geared' },
  { model: 'Wave125', years: '2012-2013', pn: '38770-KYZ-T03', cal: 'KWWM30D', gearbox: 'geared' },
  { model: 'Wave125i LED', years: '2018', pn: '38770-K73-M41', gearbox: 'geared' },
  // ---- geared: Cub/Dream ----
  { model: 'Dream Super Cub', years: '2019', pn: '38770-K76-T62', cal: 'K76L102', flashKB: 64, gearbox: 'geared' },
  { model: 'Super Cub', years: '2016', pn: '38770-KZV-A61', cal: 'K03S10A', flashKB: 64, gearbox: 'geared' },
  // ---- geared: MSX/Sonic/CBR ----
  { model: 'MSX125', years: '2013-2015', pn: '38770-K26-901', cal: 'KWWM404', flashKB: 48, gearbox: 'geared' },
  { model: 'MSX125 SF', years: '2016-2019', pn: '38770-K26-B13', cal: 'K03S203', flashKB: 64, gearbox: 'geared' },
  { model: 'Sonic150', years: '2017-2019', pn: '38770-K56-N01', cal: 'K56A603', gearbox: 'geared' },
  { model: 'CBR150R', years: '2014-2017', pn: '38770-KPP-T03', cal: 'KPPP206', gearbox: 'geared' },
  { model: 'CBR150R', years: '2017-2018', pn: '38770-K45-N42', cal: 'K45G103', flashKB: 64, gearbox: 'geared' },
  { model: 'CB150R', years: '2017-2019', pn: '38770-K94-T02', gearbox: 'geared' },
  { model: 'Monkey125', years: '2018-2019', pn: '38770-K0F-A01', cal: 'K0FA103', flashKB: 96, gearbox: 'geared' },
  { model: 'CT125', years: '2020', pn: '38770-K2E-T02', flashKB: 96, gearbox: 'geared' },
  { model: 'Winner150', years: '2017-2018', pn: '38770-K56-V01', cal: 'K56A603', gearbox: 'geared' },
  { model: 'RS150R', years: '2016-2019', pn: '38770-K56-M01', cal: 'K56A603', gearbox: 'geared' },
  // ---- auto: Scoopy/Click/Zoomer ----
  { model: 'Scoopy110i', years: '2012-2013', pn: '38770-KYT-901', cal: 'KYTA101', gearbox: 'auto' },
  { model: 'Scoopy i', years: '2013', pn: '38770-K16-901', cal: 'KWWM404', flashKB: 48, gearbox: 'auto' },
  { model: 'Scoopy i', years: '2018', pn: '38770-K16-B61', flashKB: 64, gearbox: 'auto' },
  { model: 'Click110', years: '2011', pn: '38770-KVB-S53', flashKB: 48, gearbox: 'auto' },
  { model: 'Zoomer-X', years: '2013', pn: '38770-K20-T01', flashKB: 64, gearbox: 'auto' },
  { model: 'Spacy110', years: '2011-2016', pn: '38770-KZL-C31', cal: 'KWWM404', flashKB: 48, gearbox: 'auto' },
  // ---- auto: PCX/Vario/BeAT/ADV ----
  { model: 'PCX150', years: '2017-2019', pn: '30400-K96-V01', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'PCX150', years: '2017-2019', pn: '30400-K97A-U01', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'PCX150', years: '2015-2017', pn: '30400-K50A-T02', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'Vario150', years: '2018-2019', pn: '30400-K59A-A11', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'Vario150', years: '2016-2017', pn: '30400-K59F-A01', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'Vario125', years: '2018-2019', pn: '30400-K60R-B71', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'Vario110', years: '2015-2017', pn: '30400-K46-N21', cal: 'K46F204', gearbox: 'auto' },
  { model: 'BeAT ESP', years: '2015-2019', pn: '30400-K25G-611', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'BeAT Sporty', years: '2017-2018', pn: '30400-K81A-N02', vendor: 'Shindengen', gearbox: 'auto' },
  { model: 'ADV150', years: '2019-2021', pn: '30400-K0WA-N01', vendor: 'Shindengen', gearbox: 'auto' },
]

export function findModelByPn(pn: string): EcuModel | undefined {
  return ECU_MODELS.find((m) => m.pn === pn)
}

export function modelsByGearbox(g: Gearbox): EcuModel[] {
  return ECU_MODELS.filter((m) => m.gearbox === g)
}
