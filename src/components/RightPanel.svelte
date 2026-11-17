<script lang="ts">
  import {
    comPort,
    isConnected,
    selectedBin,
    selectedFileName,
    fileSize,
    ecmId,
    logMessage,
    selectedModel,
    startOffset,
    accessOffset,
    checksumOffset,
    binChecksum,
    binChecksumValid,
    recommendedBinName,
    customEcuModels
  } from '../lib/stores'
  import { readRom, calculateCC } from '../lib/ecu'
  import { analyzeBinData } from '../lib/ecu-analyzer'

  let showCC = false
  let bore = 52.4
  let stroke = 57.9
  $: ccResult = calculateCC(bore, stroke)

  let fileInput: HTMLInputElement

  function applyAnalysis(buf: Uint8Array, name: string, sourceLabel: string) {
    const analysis = analyzeBinData(buf, name, $customEcuModels)

    selectedBin.set(buf)
    selectedFileName.set(name)
    recommendedBinName.set(name)
    fileSize.set(`${(buf.length / 1024).toFixed(0)}KB (${buf.length} bytes)`)
    ecmId.set(analysis.matchedModel ? `${analysis.matchedModel.model} (${analysis.matchedModel.pn})` : analysis.detectedId)
    if (analysis.matchedModel) {
      selectedModel.set(analysis.matchedModel.pn)
    }
    startOffset.set(analysis.startOffset)
    accessOffset.set(analysis.accessOffset)
    checksumOffset.set(analysis.checksumOffset)
    binChecksum.set(analysis.checksum)
    binChecksumValid.set(analysis.isValidChecksum)

    const ckBadge = analysis.isValidChecksum ? 'Honda Sum=0x00 OK' : `H8:${analysis.hondaChecksum8}`
    logMessage.set(`${sourceLabel}: ${analysis.detectedId} | ${ckBadge} | Start: ${analysis.startOffset} | CksumOff: ${analysis.checksumOffset}`)
  }

  async function handleRead() {
    if (!$isConnected) {
      logMessage.set('⚠️ กรุณาเชื่อมต่อ ECU ก่อนดูดไฟล์ ROM')
      return
    }
    logMessage.set('📥 กำลังดูดไฟล์ ROM (64KB)...')
    try {
      const data = await readRom($comPort, 0, 65536)
      applyAnalysis(data, 'KEIHIN-64KB.bin', '✅ ดูดไฟล์สำเร็จ')
    } catch (e: any) {
      logMessage.set(`❌ ดูดไฟล์ ROM ไม่สำเร็จ: ${e?.message ?? e}`)
    }
  }

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement
    const f = input.files?.[0]
    if (!f) return
    const reader = new FileReader()
    reader.onload = () => {
      const buf = new Uint8Array(reader.result as ArrayBuffer)
      applyAnalysis(buf, f.name, '📁 Auto Match')
      input.value = ''
    }
    reader.onerror = () => {
      logMessage.set('❌ อ่านไฟล์ BIN ไม่สำเร็จ')
      input.value = ''
    }
    reader.readAsArrayBuffer(f)
  }

  function handleSaveBin() {
    const bin = $selectedBin
    if (!bin) {
      logMessage.set('⚠️ ยังไม่มีไฟล์ BIN ให้บันทึก (กรุณาเลือกหรือดูดไฟล์ BIN ก่อน)')
      return
    }
    const blob = new Blob([new Uint8Array(bin)], { type: 'application/octet-stream' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    const baseName = $selectedFileName.replace(/\.bin$/i, '') || 'ECU_ROM'
    a.href = url
    a.download = `${baseName}_TUNED.bin`
    a.click()
    URL.revokeObjectURL(url)
    logMessage.set(`💾 บันทึกและดาวน์โหลดไฟล์ ${baseName}_TUNED.bin สำเร็จ!`)
  }

  const rightButtons = [
    { id: 'read', label: 'ดูดไฟล์ ROM', icon: '📥' },
    { id: 'bin', label: 'เลือกไฟล์ BIN', icon: '📁' },
    { id: 'save', label: 'บันทึก BIN', icon: '💾' },
    { id: 'cccalc', label: 'คำนวณ CC', icon: '🧮' },
  ]
</script>

<div class="right-panel">
  <div class="panel-title">เครื่องมือ & จัดการไฟล์ ROM</div>
  <div class="button-grid">
    {#each rightButtons as btn}
      <button class="right-btn" on:click={() => {
        if (btn.id === 'read') { handleRead(); return }
        if (btn.id === 'bin') { fileInput.click(); return }
        if (btn.id === 'save') { handleSaveBin(); return }
        if (btn.id === 'cccalc') { showCC = !showCC; return }
      }}>
        <span class="icon">{btn.icon}</span>
        <span class="label">{btn.label}</span>
      </button>
    {/each}
  </div>
  <input type="file" accept=".bin" bind:this={fileInput} on:change={handleFileSelect} style="display:none" />

  <div class="rom-info-card">
    <div class="card-header">📊 ข้อมูลรอม (ROM Metadata)</div>
    <div class="meta-row"><span>ไฟล์ปัจจุบัน:</span> <b class="val">{$selectedFileName}</b></div>
    {#if $recommendedBinName && !$selectedBin}
      <div class="meta-row"><span>ไฟล์แนะนำ:</span> <b class="val hint">{$recommendedBinName}</b></div>
    {/if}
    <div class="meta-row"><span>ขนาดไฟล์:</span> <b class="val">{$fileSize}</b></div>
    <div class="meta-row"><span>Start Offset:</span> <b class="val">{$startOffset}</b></div>
    <div class="meta-row"><span>Access Offset:</span> <b class="val">{$accessOffset}</b></div>
    <div class="meta-row"><span>Checksum Offset:</span> <b class="val">{$checksumOffset}</b></div>
    <div class="meta-row">
      <span>Checksum:</span>
      <b class="val" class:ck-ok={$binChecksumValid === true} class:ck-warn={$binChecksumValid === false}>
        {$binChecksum}
      </b>
    </div>
    <div class="meta-row"><span>ECU Model:</span> <b class="val">{$ecmId}</b></div>
  </div>

  {#if showCC}
    <div class="cc-panel">
      <div class="cc-title">🧮 คำนวณความจุกระบอกสูบ (CC)</div>
      <label>ลูกสูบ bore (มม.) <input type="number" step="0.1" bind:value={bore}></label>
      <label>ช่วงชัก stroke (มม.) <input type="number" step="0.1" bind:value={stroke}></label>
      <div class="cc-result">{ccResult.toFixed(1)} cc</div>
    </div>
  {/if}
</div>

<style>
  .right-panel {
    width: 340px;
    min-height: 0;
    overflow-y: auto;
    background: linear-gradient(180deg, #0a0a0a, #000);
    border-left: 1px solid #1a3a5a;
    padding: 8px;
    display: flex;
    flex-direction: column;
  }
  .panel-title {
    color: #7ad4ff;
    font-size: 11px;
    font-weight: 600;
    margin-bottom: 8px;
    padding-bottom: 4px;
    border-bottom: 1px solid #1a3a5a;
  }
  .button-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
  }
  .right-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    background: #0a3a6a;
    border: 1px solid #00aaff;
    border-radius: 4px;
    color: #a0d8ff;
    font-size: 10px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .right-btn:hover {
    background: #005a9a;
    box-shadow: 0 0 8px rgba(0, 170, 255, 0.4);
  }
  .icon { font-size: 11px; }
  .rom-info-card {
    margin-top: 8px;
    padding: 10px;
    background: linear-gradient(180deg, #051525, #020a14);
    border: 1px solid #1a3a5a;
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 5px;
    font-size: 10px;
    color: #7a9ab8;
  }
  .card-header {
    color: #7ad4ff;
    font-size: 11px;
    font-weight: 600;
    padding-bottom: 4px;
    margin-bottom: 2px;
    border-bottom: 1px solid #005a9a;
  }
  .meta-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    border-bottom: 1px dashed rgba(26, 58, 90, 0.45);
    padding-bottom: 3px;
  }
  .meta-row:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }
  .val {
    color: #a0d8ff;
    font-family: 'Courier New', monospace;
    text-align: right;
    word-break: break-all;
    max-width: 190px;
  }
  .val.hint { color: #facc15; }
  .val.ck-ok { color: #00ffaa; }
  .val.ck-warn { color: #ffaa00; }
  .cc-panel {
    margin-top: 8px;
    padding: 10px;
    background: #001525;
    border: 1px solid #005a9a;
    border-radius: 6px;
    font-size: 11px;
    color: #a0d8ff;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .cc-title {
    color: #7ad4ff;
    font-size: 11px;
    font-weight: 600;
    margin-bottom: 2px;
    padding-bottom: 4px;
    border-bottom: 1px solid #005a9a;
  }
  .cc-panel label { display: flex; justify-content: space-between; align-items: center; gap: 8px; }
  .cc-panel input { width: 90px; height: 24px; padding: 0 6px; }
  .cc-result { font-size: 16px; font-weight: 700; color: #00ffaa; text-align: center; }
</style>