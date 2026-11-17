import { ECU_MODELS, findModelByPn, type EcuModel } from './ecu-models'

export interface BinAnalysisResult {
  checksum: string
  hondaChecksum8: string
  crc16: string
  matchedModel?: EcuModel
  detectedId: string
  startOffset: string
  accessOffset: string
  checksumOffset: string
  mcuFamily: string
  fileSize: number
  isValidChecksum: boolean
}

export interface EcuLookupResult {
  model?: EcuModel
  recommendedBinName: string
  checksumOffset: string
  startOffset: string
  accessOffset: string
  mcuFamily: string
  flashSizeKB: number
  foundInDb: boolean
}

export interface EcuFormatCheck {
  state: 'idle' | 'valid' | 'warning' | 'invalid'
  standard: 'Keihin' | 'Shindengen' | 'TCU/ECM' | 'Database' | 'Custom' | 'None'
  message: string
}

/** Honda 8-bit checksum matching src-tauri/src/rom/checksum.rs: ((sum ^ 0xFF) + 1) & 0xFF */
export function checksum8bitHonda(data: Uint8Array): number {
  let sum = 0
  for (let i = 0; i < data.length; i++) {
    sum += data[i]
  }
  return ((sum ^ 0xff) + 1) & 0xff
}

/** CRC-16 CCITT-FALSE (0x1021) matching src-tauri/src/rom/checksum.rs */
export function crc16Honda(data: Uint8Array): number {
  let crc = 0xffff
  for (let i = 0; i < data.length; i++) {
    crc ^= data[i] << 8
    for (let b = 0; b < 8; b++) {
      crc = crc & 0x8000 ? ((crc << 1) ^ 0x1021) & 0xffff : (crc << 1) & 0xffff
    }
  }
  return crc
}

export function getOffsetProfileByKB(flashKB: number): {
  startOffset: string
  checksumOffset: string
  accessOffset: string
  mcuFamily: string
} {
  switch (flashKB) {
    case 48:
      return { startOffset: '0x4000', checksumOffset: '0x7600', accessOffset: '0xBFFF', mcuFamily: 'R8C 48KB' }
    case 56:
      return { startOffset: '0x0000', checksumOffset: '0xDFEF', accessOffset: '0xDFFF', mcuFamily: 'H8C 56KB' }
    case 96:
      return { startOffset: '0x8000', checksumOffset: '0x0000', accessOffset: '0x17FFF', mcuFamily: 'R8C 96KB' }
    case 128:
      return { startOffset: '0xE0000', checksumOffset: '0xB000', accessOffset: '0x1FFFF', mcuFamily: 'M16C 128KB' }
    case 64:
    default:
      return { startOffset: '0x8000', checksumOffset: '0x0000', accessOffset: '0xFFFF', mcuFamily: 'R8C 64KB' }
  }
}

export function validateEcuIdFormat(raw: string, customModels: EcuModel[] = []): EcuFormatCheck {
  const val = raw.trim().toUpperCase()
  if (!val) {
    return { state: 'idle', standard: 'None', message: '' }
  }

  const keihinRegex = /^38770-[A-Z0-9]{2,5}-[A-Z0-9]{2,4}$/
  const shindengenRegex = /^30400-[A-Z0-9]{2,5}-[A-Z0-9]{2,4}$/
  const tcuRegex = /^(31200|38800)-[A-Z0-9]{2,5}-[A-Z0-9]{2,4}$/

  const lookup = lookupEcuDatabase(val, customModels)

  if (keihinRegex.test(val)) {
    return {
      state: 'valid',
      standard: 'Keihin',
      message: lookup.foundInDb && lookup.model
        ? `Keihin Standard (${lookup.model.model}) | BIN: ${lookup.recommendedBinName}`
        : `Keihin Standard Format | แนะนำไฟล์: ${lookup.recommendedBinName}`
    }
  }
  if (shindengenRegex.test(val)) {
    return {
      state: 'valid',
      standard: 'Shindengen',
      message: lookup.foundInDb && lookup.model
        ? `Shindengen Standard (${lookup.model.model}) | BIN: ${lookup.recommendedBinName}`
        : `Shindengen Standard Format | แนะนำไฟล์: ${lookup.recommendedBinName}`
    }
  }
  if (tcuRegex.test(val)) {
    return {
      state: 'valid',
      standard: 'TCU/ECM',
      message: `Honda ECM/TCU Format | แนะนำไฟล์: ${lookup.recommendedBinName}`
    }
  }
  if (lookup.foundInDb && lookup.model) {
    return {
      state: 'valid',
      standard: 'Database',
      message: `พบในฐานข้อมูล: ${lookup.model.model} (${lookup.model.pn}) | Offset: ${lookup.startOffset}`
    }
  }
  if (/^[A-Z0-9-]{4,24}$/.test(val)) {
    return {
      state: 'warning',
      standard: 'Custom',
      message: 'รูปแบบกำหนดเอง (ยังไม่ตรงมาตรฐาน 38770-XXX-XXX หรือ 30400-XXX-XXX)'
    }
  }
  return {
    state: 'invalid',
    standard: 'None',
    message: 'รูปแบบไม่ถูกต้อง (ใช้ได้เฉพาะ A-Z, 0-9 และ - ความยาว 4-24 ตัวอักษร)'
  }
}

export function lookupEcuDatabase(query: string, customModels: EcuModel[] = []): EcuLookupResult {
  const cleanQuery = query.trim().toUpperCase()
  const allModels = [...customModels, ...ECU_MODELS]

  if (!cleanQuery) {
    const profile = getOffsetProfileByKB(64)
    return {
      recommendedBinName: 'SELECT_ECU_ID.bin',
      checksumOffset: profile.checksumOffset,
      startOffset: profile.startOffset,
      accessOffset: profile.accessOffset,
      mcuFamily: profile.mcuFamily,
      flashSizeKB: 64,
      foundInDb: false
    }
  }

  const normalizedQuery = cleanQuery.replace(/-/g, '')
  const matchedModel = allModels.find(
    m =>
      m.pn.toUpperCase() === cleanQuery ||
      m.pn.toUpperCase().replace(/-/g, '') === normalizedQuery ||
      m.pn.toUpperCase().includes(cleanQuery) ||
      m.model.toUpperCase().includes(cleanQuery)
  )

  if (matchedModel) {
    const kb = matchedModel.flashKB ?? 64
    const profile = getOffsetProfileByKB(kb)
    return {
      model: matchedModel,
      recommendedBinName: `${matchedModel.model.replace(/\s+/g, '_')}_${matchedModel.pn.replace(/-/g, '_')}_STOCK.bin`,
      checksumOffset: profile.checksumOffset,
      startOffset: profile.startOffset,
      accessOffset: profile.accessOffset,
      mcuFamily: profile.mcuFamily,
      flashSizeKB: kb,
      foundInDb: true
    }
  }

  const profile = getOffsetProfileByKB(64)
  return {
    recommendedBinName: `CUSTOM_${cleanQuery.replace(/[^A-Z0-9_-]/g, '_')}.bin`,
    checksumOffset: profile.checksumOffset,
    startOffset: profile.startOffset,
    accessOffset: profile.accessOffset,
    mcuFamily: profile.mcuFamily,
    flashSizeKB: 64,
    foundInDb: false
  }
}

export function analyzeBinData(buf: Uint8Array, filename: string, customModels: EcuModel[] = []): BinAnalysisResult {
  const allModels = [...customModels, ...ECU_MODELS]

  // 1. Real Honda 8-bit checksum & CRC16-CCITT (mirrors src-tauri/src/rom/checksum.rs)
  const hondaVal = checksum8bitHonda(buf)
  const crcVal = crc16Honda(buf)
  const hondaHex = '0x' + hondaVal.toString(16).toUpperCase().padStart(2, '0')
  const crcHex = '0x' + crcVal.toString(16).toUpperCase().padStart(4, '0')

  // 2. Search for part number pattern in filename or binary text
  let detectedId = ''
  let matchedModel: EcuModel | undefined = undefined
  const upperName = filename.toUpperCase()

  for (const model of allModels) {
    const upperPn = model.pn.toUpperCase()
    if (upperName.includes(upperPn.replace(/-/g, '')) || upperName.includes(upperPn)) {
      matchedModel = model
      detectedId = model.pn
      break
    }
  }

  // If not found in filename, scan binary ASCII text for Keihin/Shindengen PN
  if (!detectedId) {
    const sampleSize = Math.min(buf.length, 65536)
    let textChunk = ''
    for (let i = 0; i < sampleSize; i++) {
      const charCode = buf[i]
      textChunk += charCode >= 32 && charCode <= 126 ? String.fromCharCode(charCode) : '.'
    }
    const pnMatch = textChunk.match(/(3[08]\d{3}-[A-Z0-9]{2,5}-[A-Z0-9]{2,4})/i)
    if (pnMatch) {
      detectedId = pnMatch[1].toUpperCase()
      matchedModel = allModels.find(m => m.pn.toUpperCase() === detectedId) ?? findModelByPn(detectedId)
    }
  }

  const sizeKB = Math.round(buf.length / 1024)
  if (!detectedId) {
    detectedId = `ROM_${sizeKB}KB_CUSTOM`
  }

  // 3. Determine offsets based on exact file size & verified PDF table
  const knownKB = buf.length === 49152 || buf.length === 48576 ? 48
    : buf.length === 57344 ? 56
    : buf.length === 98304 ? 96
    : buf.length >= 131072 ? 128
    : 64
  const profile = getOffsetProfileByKB(matchedModel?.flashKB ?? knownKB)

  return {
    checksum: `${crcHex} (H8:${hondaHex})`,
    hondaChecksum8: hondaHex,
    crc16: crcHex,
    matchedModel,
    detectedId,
    startOffset: profile.startOffset,
    accessOffset: profile.accessOffset,
    checksumOffset: profile.checksumOffset,
    mcuFamily: profile.mcuFamily,
    fileSize: buf.length,
    isValidChecksum: buf.length > 0 && hondaVal === 0
  }
}
