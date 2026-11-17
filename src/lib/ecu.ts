// ECU hardware/mock bridge - connects to Rust Tauri commands when running inside desktop Tauri
export type ComPort = { name: string; type: string }

export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && Boolean((window as any).__TAURI_INTERNALS__ || (window as any).__TAURI__)
}

export async function listPorts(): Promise<ComPort[]> {
  if (isTauriRuntime()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const raw = await invoke<Array<{ name: string; port_type?: string; type?: string }>>('list_ports')
      return raw.map((p) => ({
        name: p.name,
        type: p.type ?? p.port_type ?? 'Serial Port'
      }))
    } catch {
      // Fallback to empty/mock if serialport scan fails
    }
  }
  return [
    { name: 'COM75', type: 'USB Serial (CH340)' },
    { name: 'COM3', type: 'FTDI FT232R' },
    { name: '/dev/ttyUSB0', type: 'Linux USB' }
  ]
}

export async function connectEcu(port: string): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke('connect_ecu', { port })
  }
  await new Promise(r => setTimeout(r, 900))
  return `เชื่อมต่อ ${port} สำเร็จ (Web Simulation)`
}

export interface EcuInfoResponse {
  id: string
  size: string
  checksum: string
}

export async function getEcuInfo(port: string): Promise<EcuInfoResponse> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke<EcuInfoResponse>('get_ecu_info', { port })
  }
  return {
    id: 'KEIHIN 38770-K03-H01',
    size: '64KB',
    checksum: '0x00'
  }
}

export function calculateCC(bore: number, stroke: number): number {
  if (bore <= 0 || stroke <= 0) return 0
  const r = bore / 2
  return Math.PI * r * r * stroke / 1000
}

export async function mockReadRom(sizeKB = 64): Promise<Uint8Array> {
  const data = new Uint8Array(sizeKB * 1024)
  for (let i = 0; i < data.length - 1; i++) {
    data[i] = Math.floor(Math.random() * 256)
  }
  // Honda header + balanced last byte so Honda 8-bit checksum == 0x00
  data[0] = 0x48; data[1] = 0x4F; data[2] = 0x4E; data[3] = 0x44; data[4] = 0x41
  let sum = 0
  for (let i = 0; i < data.length - 1; i++) sum += data[i]
  data[data.length - 1] = ((sum ^ 0xff) + 1) & 0xff
  return data
}

export async function readRom(port: string, offset: number = 0, size: number = 65536): Promise<Uint8Array> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    const bytes = await invoke<number[]>('read_rom', { port, offset, size })
    return new Uint8Array(bytes)
  }
  return mockReadRom(Math.max(1, Math.round(size / 1024)))
}

export async function writeRom(port: string, data: Uint8Array): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke<string>('write_rom', { port, data: Array.from(data) })
  }
  return `แฟลชสำเร็จ ${data.length} ไบต์ (Web Simulation)`
}

export async function simulateFlash(port: string, data: Uint8Array): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke<string>('simulate_flash', { port, data: Array.from(data) })
  }
  return `จำลองแฟลชสำเร็จ ${data.length} ไบต์`
}

export async function resetFlashCount(port: string): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke<string>('reset_flash_count', { port })
  }
  await new Promise(r => setTimeout(r, 900))
  return 'รีเซต FLASHCOUNT และค่า Adaptation กล่อง ECU สำเร็จ'
}

export interface FaultSet {
  current: string[]
  past: string[]
}

const MOCK_FAULT_POOL = ['01-01', '07-01', '08-01', '11-01', '21-01', '33-02']

// Mock DTC read (web dev) - backend has no DTC command yet, shape matches
// eculib get_faults(): { current, past } code lists.
export async function getFaults(): Promise<FaultSet> {
  await new Promise(r => setTimeout(r, 700))
  if (Math.random() < 0.5) return { current: [], past: [] }
  const pick = MOCK_FAULT_POOL[Math.floor(Math.random() * MOCK_FAULT_POOL.length)]
  return { current: [pick], past: [] }
}

export async function clearFaults(): Promise<string> {
  await new Promise(r => setTimeout(r, 800))
  return 'ลบ DTC สำเร็จ - ปิด-เปิดกุญแจ 10 วินาที'
}
