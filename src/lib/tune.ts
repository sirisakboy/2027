// Tuning engine - real byte transforms on the loaded BIN.
// Evidence base (from repo file diffs, /tmp/file_real analysis):
// - K03-H01: limiter cells u16LE @0x9682/0x968C/0xAC93/0xAC9D (8B _12000RPM),
//   fuel window block0 (53.bin), idle window block1 (53 หอบ.bin)
// - K58-T81: limiter u8 @0x0046/0x0048 (2B 11500- file), fuel blocks 0-1 (E85)
// - Unknown models: generic R8C windows, NO limiter addrs (button disabled
//   with a note) - we never guess limiter addresses.

export type TuneStyle = 'limiter' | 'fuel' | 'ignition' | 'lope' | 'e85'

export interface TunePreset {
  id: string
  label: string
  match: (fileName: string) => boolean
  /** u16LE limiter cells */
  limiterU16: number[]
  /** u8 limiter cells */
  limiterU8: number[]
  /** [start, end) map window for fuel/ignition */
  fuel: [number, number]
  /** [start, end) idle window for lope */
  idle: [number, number]
}

const K03: TunePreset = {
  id: 'K03-H01',
  label: 'K03-H01 Wave110i',
  match: (n) => n.includes('K03-H01') || n.includes('K03'),
  limiterU16: [0x9682, 0x968c, 0xac93, 0xac9d],
  limiterU8: [],
  fuel: [0x0000, 0x1000],
  idle: [0x1000, 0x2000],
}

const K58: TunePreset = {
  id: 'K58-T81',
  label: 'K58-T81 Wave110i',
  match: (n) => n.includes('K58'),
  limiterU16: [],
  limiterU8: [0x0046, 0x0048],
  fuel: [0x0000, 0x2000],
  idle: [0x1000, 0x2000],
}

const GENERIC: TunePreset = {
  id: 'generic',
  label: 'Generic R8C (ไม่มี preset)',
  match: () => true,
  limiterU16: [],
  limiterU8: [],
  fuel: [0x0000, 0x1000],
  idle: [0x1000, 0x2000],
}

export function detectPreset(fileName: string): TunePreset {
  if (K03.match(fileName) && !GENERIC_ONLY(fileName)) return K03
  if (K58.match(fileName)) return K58
  return GENERIC
}

/** Prefer an explicitly chosen ECU model over filename guessing. */
export function presetForModel(pn: string, fileName: string): TunePreset {
  if (pn.includes('K03')) return K03
  if (pn.includes('K58')) return K58
  if (pn) return GENERIC
  return detectPreset(fileName)
}

// K03 matcher also fires on plain 'K03' (e.g. K03S10A cal names) - that's
// intended: same R8C family layout. Files that are clearly another model
// (K20/K26/K45/K73/K76/K0F/K2J/KVB/K16) fall through to generic.
function GENERIC_ONLY(n: string): boolean {
  return /K2J|K20|K26|K45|K73|K76|K0F|KVB|K16|K2E|KYZ|KWW|K58|KYT|KYK|KYL|KZV|K15|K33|K56|K64/.test(n)
}

export interface TuneResult {
  data: Uint8Array
  changed: number
  detail: string
}

function clamp8(v: number): number {
  return v < 0 ? 0 : v > 255 ? 255 : Math.round(v)
}

function readU16LE(d: Uint8Array, a: number): number {
  return d[a] | (d[a + 1] << 8)
}

function writeU16LE(d: Uint8Array, a: number, v: number): void {
  d[a] = v & 0xff
  d[a + 1] = (v >> 8) & 0xff
}

function inRange(d: Uint8Array, a: number, need: number): boolean {
  return a >= 0 && a + need <= d.length
}

export interface TuneOpt {
  /** limiter/fuel/e85 scale, e.g. 1.1 = +10% */
  factor?: number
  /** ignition additive degrees */
  amount?: number
}

/** Apply one tuning style. Never mutates input - returns a new image. */
export function applyTune(
  src: Uint8Array,
  fileName: string,
  style: TuneStyle,
  opt: TuneOpt = {},
  preset?: TunePreset,
): TuneResult {
  const p = preset ?? detectPreset(fileName)
  const out = new Uint8Array(src)
  let changed = 0
  let detail = ''

  switch (style) {
    case 'limiter': {
      const f = opt.factor ?? 1.1
      const cells: string[] = []
      for (const a of p.limiterU16) {
        if (!inRange(out, a, 2)) continue
        const old = readU16LE(out, a)
        const v = Math.min(0xffff, Math.round(old * f))
        writeU16LE(out, a, v)
        changed += 2
        cells.push(`0x${a.toString(16)}:${old}->${v}`)
      }
      for (const a of p.limiterU8) {
        if (!inRange(out, a, 1)) continue
        const old = out[a]
        out[a] = clamp8(old * f)
        changed += 1
        cells.push(`0x${a.toString(16)}:${old}->${out[a]}`)
      }
      detail = cells.length ? cells.join(' ') : 'ไม่มี preset limiter ของรุ่นนี้'
      break
    }
    case 'fuel': {
      const f = opt.factor ?? 1.2
      const [s, e] = p.fuel
      for (let i = s; i < Math.min(e, out.length); i++) {
        const v = clamp8(out[i] * f)
        if (v !== out[i]) {
          out[i] = v
          changed++
        }
      }
      detail = `fuel window 0x${s.toString(16)}-0x${Math.min(e, out.length).toString(16)} x${f}`
      break
    }
    case 'ignition': {
      const deg = opt.amount ?? 3
      const [s, e] = p.fuel
      for (let i = s; i < Math.min(e, out.length); i++) {
        const v = clamp8(out[i] + deg)
        if (v !== out[i]) {
          out[i] = v
          changed++
        }
      }
      detail = `ign +${deg} @0x${s.toString(16)}-0x${Math.min(e, out.length).toString(16)}`
      break
    }
    case 'lope': {
      const amp = opt.amount ?? 12
      const [s, e] = p.idle
      const end = Math.min(e, out.length)
      const cols = 16
      for (let i = s; i < end; i++) {
        const row = Math.floor((i - s) / cols)
        const col = (i - s) % cols
        const d = (row + col) % 2 === 0 ? amp : -amp
        const v = clamp8(out[i] + d)
        if (v !== out[i]) {
          out[i] = v
          changed++
        }
      }
      detail = `lope ±${amp} @0x${s.toString(16)}-0x${end.toString(16)}`
      break
    }
    case 'e85': {
      const [s, e] = p.fuel
      for (let i = s; i < Math.min(e, out.length); i++) {
        const v = clamp8(out[i] * 1.35)
        if (v !== out[i]) {
          out[i] = v
          changed++
        }
      }
      detail = `E85 fuel x1.35 @0x${s.toString(16)}-0x${Math.min(e, out.length).toString(16)}`
      break
    }
  }

  return { data: out, changed, detail: `[${p.id}] ${detail} (${changed} bytes)` }
}
