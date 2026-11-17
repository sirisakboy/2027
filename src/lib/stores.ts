import { writable } from 'svelte/store'
import type { EcuModel } from './ecu-models'

export const comPort = writable<string>('COM75')

// Connection state
export const isConnected = writable<boolean>(false)
export const isConnecting = writable<boolean>(false)
export const ecuStatus = writable<string>('รอการเชื่อมต่อ')

// File state
export const fileSize = writable<string>('ไม่มีไฟล์')
export const ecmId = writable<string>('--')
export const selectedBin = writable<Uint8Array | null>(null)
export const selectedFileName = writable<string>('ยังไม่มีไฟล์')
export const currentView = writable<'dashboard' | 'tune' | 'dtc'>('dashboard')

// Validation / lookup metadata (kept separate so validating an ID doesn't fake a loaded BIN)
export const recommendedBinName = writable<string>('')
export const checksumOffset = writable<string>('--')
export const binChecksum = writable<string>('--')
export const binChecksumValid = writable<boolean | null>(null)

// Offset state
export const startOffset = writable<string>('0x0000')
export const accessOffset = writable<string>('0xFFFF')

// Flash state
export const flashProgress = writable<number>(0)
export const isFlashing = writable<boolean>(false)

// Logging
export const logMessage = writable<string>('เตรียมใช้งาน...')

// Live Data State for ANAROG/GAUGE
export const liveData = writable<LiveDataState>({
  rpm: 0,
  tps: 0,
  ect: 0,
  iat: 0,
  map: 0,
  throttle: 0,
  fuel: 0,
  ignition: 0
})

export interface LiveDataState {
  rpm: number
  tps: number      // Throttle Position Sensor (%)
  ect: number      // Engine Coolant Temperature (°C)
  iat: number      // Intake Air Temperature (°C)
  map: number      // Manifold Absolute Pressure (kPa)
  throttle: number // Throttle Angle (%)
  fuel: number     // Fuel Trim (%)
  ignition: number // Ignition Timing (degrees BTDC)
}

// Mock live data generator
export function generateMockLiveData(): LiveDataState {
  const now = Date.now()
  return {
    rpm: Math.floor(800 + Math.sin(now / 2000) * 600),
    tps: Math.floor(Math.sin(now / 1500) * 30 + 20),
    ect: Math.floor(85 + Math.cos(now / 3000) * 3),
    iat: Math.floor(25 + Math.sin(now / 2500) * 2),
    map: Math.floor(100 + Math.sin(now / 1800) * 20),
    throttle: Math.floor(Math.sin(now / 1200) * 15 + 30),
    fuel: Math.floor(Math.sin(now / 1000) * 2 + 0),
    ignition: Math.floor(10 + Math.sin(now / 800) * 3)
  }
}

// Selected ECU model part number ('' = not chosen, detect from filename)
export const selectedModel = writable<string>('')

// Mock file size selector (48KB, 56KB, 64KB, 128KB)
export const mockFileSize = writable<number>(64)

// Central DB & UI settings stores
export const customEcuModels = writable<EcuModel[]>([])

export interface TuningLogEntry {
  id: string
  creator: string
  fileName: string
  modelPn: string
  details: string
  createdAt: string
}

export const recentTuningLogs = writable<TuningLogEntry[]>([])

export interface EcuSettings {
  baudRate: string
  autoBackup: boolean
}

export const ecuSettings = writable<EcuSettings>({
  baudRate: '10400',
  autoBackup: true
})

export interface ExtraOptions {
  e85Mode: boolean
  o2Off: boolean
  popBangs: boolean
}

export const extraOptions = writable<ExtraOptions>({
  e85Mode: false,
  o2Off: false,
  popBangs: false
})