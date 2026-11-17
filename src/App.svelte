<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { get } from 'svelte/store'
  import LeftPanel from './components/LeftPanel.svelte'
  import RightPanel from './components/RightPanel.svelte'
  import TunePanel from './components/TunePanel.svelte'
  import DtcPanel from './components/DtcPanel.svelte'
  import {
    comPort,
    isConnected,
    isConnecting,
    ecuStatus,
    fileSize,
    ecmId,
    startOffset,
    accessOffset,
    checksumOffset,
    recommendedBinName,
    logMessage,
    selectedBin,
    selectedFileName,
    currentView,
    liveData,
    generateMockLiveData,
    selectedModel,
    customEcuModels,
    recentTuningLogs,
    ecuSettings,
    extraOptions
  } from './lib/stores'
  import { listPorts, connectEcu, getEcuInfo, resetFlashCount } from './lib/ecu'
  import { lookupEcuDatabase, validateEcuIdFormat } from './lib/ecu-analyzer'
  import { applyTune, presetForModel } from './lib/tune'
  import type { EcuModel } from './lib/ecu-models'
  import { db, handleFirestoreError, OperationType } from './lib/firebase'
  import { collection, addDoc, getDocs, query, orderBy, limit } from 'firebase/firestore'

  let ports: Array<{ name: string; type: string }> = []
  let showPorts = false
  let currentTime = ''
  let timeTimer: ReturnType<typeof setInterval> | null = null
  let liveTimer: ReturnType<typeof setInterval> | null = null

  // ECU ID validation state
  let inputEcuId = ''
  let validatedModelInfo: { model: string; pn: string; years?: string } | null = null
  let validationStatus = ''
  let formatHint = ''
  let formatState: 'idle' | 'valid' | 'warning' | 'invalid' = 'idle'
  let lookupResult: ReturnType<typeof lookupEcuDatabase> | null = null

  function checkEcuFormat() {
    const val = inputEcuId.trim()
    if (!val) {
      formatHint = ''
      formatState = 'idle'
      lookupResult = null
      return
    }
    const models = get(customEcuModels)
    lookupResult = lookupEcuDatabase(val, models)
    const fmt = validateEcuIdFormat(val, models)
    formatState = fmt.state
    const icon = fmt.state === 'valid' ? '🟢' : fmt.state === 'warning' ? '🟡' : '🔴'
    formatHint = fmt.message ? `${icon} ${fmt.message}` : ''
  }

  async function validateAndPrepareEcu() {
    const raw = inputEcuId.trim().toUpperCase()
    if (!raw) {
      logMessage.set('⚠️ กรุณากรอกรหัส ECU ID')
      return
    }
    const models = get(customEcuModels)
    const fmt = validateEcuIdFormat(raw, models)
    if (fmt.state === 'invalid') {
      formatState = 'invalid'
      formatHint = `🔴 ${fmt.message}`
      logMessage.set(`⚠️ ${fmt.message}`)
      return
    }

    const result = lookupEcuDatabase(raw, models)
    lookupResult = result

    if (result.foundInDb && result.model) {
      validatedModelInfo = result.model
      selectedModel.set(result.model.pn)
      ecmId.set(`${result.model.model} (${result.model.pn})`)
      startOffset.set(result.startOffset)
      accessOffset.set(result.accessOffset)
      checksumOffset.set(result.checksumOffset)
      recommendedBinName.set(result.recommendedBinName)
      validationStatus = `✅ Auto-Matched: ${result.model.model} (${result.model.years}) | MCU: ${result.mcuFamily} | BIN: ${result.recommendedBinName} | Start: ${result.startOffset} | CksumOff: ${result.checksumOffset}`
      logMessage.set(validationStatus)

      try {
        await addDoc(collection(db, 'ecu_validations'), {
          ecuId: result.model.pn.slice(0, 64),
          model: result.model.model.slice(0, 128),
          recommendedBin: result.recommendedBinName.slice(0, 256),
          startOffset: result.startOffset.slice(0, 32),
          checksumOffset: result.checksumOffset.slice(0, 32),
          validatedAt: new Date().toISOString()
        })
      } catch (e) {
        try {
          handleFirestoreError(e, OperationType.CREATE, 'ecu_validations')
        } catch {
          // Non-blocking telemetry log failure
        }
      }
    } else {
      selectedModel.set(raw)
      ecmId.set(raw)
      startOffset.set(result.startOffset)
      accessOffset.set(result.accessOffset)
      checksumOffset.set(result.checksumOffset)
      recommendedBinName.set(result.recommendedBinName)
      validatedModelInfo = { model: 'Custom ECU', pn: raw }
      validationStatus = `⚠️ Custom ECU: ${raw} | แนะนำไฟล์: ${result.recommendedBinName} | Start: ${result.startOffset} | CksumOff: ${result.checksumOffset}`
      logMessage.set(validationStatus)
    }
  }

  // Modal states
  let activeModal: string | null = null
  let newEcuPn = ''
  let newEcuName = ''
  let newEcuVendor: 'Keihin' | 'Shindengen' = 'Keihin'
  let newEcuFlashKB = 64
  let isResetting = false
  let baudRate = $ecuSettings.baudRate
  let autoBackup = $ecuSettings.autoBackup
  let extraE85 = $extraOptions.e85Mode
  let extraO2Off = $extraOptions.o2Off
  let extraPopBangs = $extraOptions.popBangs

  async function loadCentralDatabase() {
    try {
      const ecuSnap = await getDocs(query(collection(db, 'ecu_part_numbers'), orderBy('createdAt', 'desc'), limit(50)))
      const loadedModels: EcuModel[] = ecuSnap.docs.map((d) => {
        const data = d.data()
        const vRaw = String(data.vendor || '').toUpperCase()
        return {
          model: String(data.model || 'Custom ECU'),
          years: 'Custom DB',
          pn: String(data.pn || ''),
          vendor: vRaw === 'SHINDENGEN' ? 'Shindengen' : 'Keihin',
          flashKB: Number(data.flashKB) || 64,
          gearbox: 'geared'
        }
      }).filter((m) => m.pn.length > 0)
      customEcuModels.set(loadedModels)
    } catch (e) {
      try {
        handleFirestoreError(e, OperationType.LIST, 'ecu_part_numbers')
      } catch {
        // Ignore gracefully if offline
      }
    }

    try {
      const logSnap = await getDocs(query(collection(db, 'tuning_logs'), orderBy('createdAt', 'desc'), limit(10)))
      recentTuningLogs.set(logSnap.docs.map((d) => {
        const data = d.data()
        return {
          id: String(data.id || d.id),
          creator: String(data.creator || 'creator_central'),
          fileName: String(data.fileName || ''),
          modelPn: String(data.modelPn || 'UNKNOWN'),
          details: String(data.details || ''),
          createdAt: String(data.createdAt || '')
        }
      }))
    } catch (e) {
      try {
        handleFirestoreError(e, OperationType.LIST, 'tuning_logs')
      } catch {
        // Ignore gracefully if offline
      }
    }
  }

  onMount(async () => {
    ports = await listPorts()
    updateTime()
    timeTimer = setInterval(updateTime, 1000)
    liveTimer = setInterval(() => liveData.set(generateMockLiveData()), 1000)
    await loadCentralDatabase()
  })

  onDestroy(() => {
    if (timeTimer) clearInterval(timeTimer)
    if (liveTimer) clearInterval(liveTimer)
  })

  function openModal(name: string) {
    if (name === 'settings') {
      baudRate = $ecuSettings.baudRate
      autoBackup = $ecuSettings.autoBackup
    } else if (name === 'extra') {
      extraE85 = $extraOptions.e85Mode
      extraO2Off = $extraOptions.o2Off
      extraPopBangs = $extraOptions.popBangs
    }
    activeModal = name
  }

  function closeModal() {
    activeModal = null
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && activeModal) {
      closeModal()
    }
  }

  async function saveNewEcu() {
    const pn = newEcuPn.trim().toUpperCase()
    const modelName = newEcuName.trim() || 'Custom ECU'
    if (!pn || pn.length > 64) {
      logMessage.set('⚠️ กรุณากรอกรหัสกล่อง ECU ให้ถูกต้อง (1-64 ตัวอักษร)')
      return
    }
    try {
      await addDoc(collection(db, 'ecu_part_numbers'), {
        pn,
        model: modelName.slice(0, 128),
        vendor: newEcuVendor.toUpperCase(),
        flashKB: Number(newEcuFlashKB) || 64,
        createdAt: new Date().toISOString()
      })
      customEcuModels.update((prev) => [
        { model: modelName, years: 'Custom DB', pn, vendor: newEcuVendor, flashKB: Number(newEcuFlashKB) || 64, gearbox: 'geared' },
        ...prev
      ])
      logMessage.set(`✅ เพิ่มรหัสกล่อง ${pn} ลงฐานข้อมูลกลางสำเร็จ`)
      newEcuPn = ''
      newEcuName = ''
      closeModal()
    } catch (e) {
      try {
        handleFirestoreError(e, OperationType.CREATE, 'ecu_part_numbers')
      } catch {
        logMessage.set('❌ บันทึกรหัสกล่องลงฐานข้อมูลกลางไม่สำเร็จ')
      }
    }
  }

  async function performReset() {
    if (isResetting) return
    isResetting = true
    logMessage.set('🔄 กำลังรีเซตค่า EEPROM และ Adaptation กล่อง ECU...')
    try {
      const msg = await resetFlashCount($comPort)
      logMessage.set(`✅ ${msg} - ปิดสวิตช์กุญแจ 10 วินาที`)
      closeModal()
    } catch (e: any) {
      logMessage.set(`❌ รีเซตกล่องไม่สำเร็จ: ${e?.message ?? e}`)
    } finally {
      isResetting = false
    }
  }

  function saveSettings() {
    ecuSettings.set({ baudRate, autoBackup })
    logMessage.set(`⚙️ บันทึกตั้งค่าแล้ว (Baud: ${baudRate} bps | Auto-Backup: ${autoBackup ? 'ON' : 'OFF'})`)
    closeModal()
  }

  function applyExtraOptions() {
    extraOptions.set({ e85Mode: extraE85, o2Off: extraO2Off, popBangs: extraPopBangs })
    const bin = get(selectedBin)
    if (bin && extraE85) {
      const r = applyTune(bin, get(selectedFileName), 'e85', {}, presetForModel(get(selectedModel), get(selectedFileName)))
      if (r.changed > 0) {
        selectedBin.set(r.data)
        logMessage.set(`🚀 ใช้ฟังก์ชั่นพิเศษ (E85 +${r.changed} bytes) สำเร็จ`)
        closeModal()
        return
      }
    }
    const flags = [
      extraE85 ? 'E85' : null,
      extraO2Off ? 'O2-OFF' : null,
      extraPopBangs ? 'Pop&Bangs' : null
    ].filter(Boolean)
    logMessage.set(flags.length ? `🚀 เปิดฟังก์ชั่นพิเศษ: ${flags.join(', ')}` : '🚀 ปิดฟังก์ชั่นเพิ่มเติมทั้งหมด')
    closeModal()
  }

  function updateTime() {
    const now = new Date()
    const days = ['อาทิตย์', 'จันทร์', 'อังคาร', 'พุธ', 'พฤหัส', 'ศุกร์', 'เสาร์']
    const d = days[now.getDay()]
    const dd = String(now.getDate()).padStart(2, '0')
    const mm = String(now.getMonth() + 1).padStart(2, '0')
    const yyyy = now.getFullYear() + 543
    const hh = String(now.getHours()).padStart(2, '0')
    const mi = String(now.getMinutes()).padStart(2, '0')
    const ss = String(now.getSeconds()).padStart(2, '0')
    currentTime = `${d} - ${dd}-${mm}-${yyyy} - ${hh}:${mi}:${ss} น.`
  }

  async function handleConnect() {
    if ($isConnected) {
      isConnected.set(false)
      ecuStatus.set('รอการเชื่อมต่อ')
      logMessage.set('🔌 ตัดการเชื่อมต่อแล้ว')
      return
    }
    isConnecting.set(true)
    ecuStatus.set('กำลังเชื่อมต่อ...')
    logMessage.set(`🔌 กำลังเชื่อมต่อ ${$comPort} (${$ecuSettings.baudRate} bps)...`)
    try {
      const msg = await connectEcu($comPort)
      const info = await getEcuInfo($comPort)
      isConnected.set(true)
      ecuStatus.set('เชื่อมต่อสำเร็จ')
      if ($fileSize === 'ไม่มีไฟล์') fileSize.set(`${info.size} (พร้อมอ่าน)`)
      if ($ecmId === '--') ecmId.set(info.id)
      logMessage.set(`✅ ${msg}`)
    } catch (e: any) {
      ecuStatus.set('เชื่อมต่อล้มเหลว')
      logMessage.set(`❌ ${e.message}`)
    } finally {
      isConnecting.set(false)
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeydown} />

<div class="app-window">
  <div class="title-bar">
    <div class="win-title">REMAP V.5 - Tauri Edition</div>
    <div class="controls"><span>—</span><span>□</span><span>✕</span></div>
  </div>
  <div class="menu-bar">
    <button class="menu-btn" on:click={() => openModal('add-ecu')}>เพิ่มรหัสกล่อง</button>
    <button class="menu-btn" on:click={() => openModal('reset-ecu')}>รีเซตกล่อง</button>
    <button class="menu-btn" on:click={() => openModal('profile')}>รูปโปร / พินเอาท์</button>
    <button class="menu-btn" on:click={() => openModal('settings')}>ตั้งค่า</button>
    <button class="menu-btn" on:click={() => openModal('extra')}>ฟังชั่นเพิ่มเติม</button>
    <button class="menu-btn" on:click={() => openModal('install')}>วิธีติดตั้งโปรแกรม</button>
    <button class="menu-btn" on:click={() => openModal('help')}>ช่วยเหลือ</button>
    <span class="datetime">{currentTime}</span>
  </div>

  <div class="main-content">
    <LeftPanel />

    <section class="center-panel">
      {#if $currentView === 'dashboard'}
        <div class="dashboard-area">
          <h3>📊 LIVEDATA & ECU VALIDATION</h3>
          <div class="ecu-input-card">
            <label for="ecu-id-input"><b>🔍 กรอกหรือตรวจสอบรหัส ECU / ECM / TCU ID (Part Number):</b></label>
            <div class="ecu-input-row">
              <input
                id="ecu-id-input"
                type="text"
                bind:value={inputEcuId}
                on:input={checkEcuFormat}
                on:keydown={(e) => { if (e.key === 'Enter') validateAndPrepareEcu() }}
                placeholder="เช่น 38770-K03-H01 หรือ 30400-K96-V01"
              >
              <button class="action-btn" on:click={validateAndPrepareEcu}>ตรวจสอบและเตรียมโปรไฟล์</button>
            </div>
            {#if formatHint}
              <div class="format-hint" class:valid={formatState === 'valid'} class:warning={formatState === 'warning'} class:invalid={formatState === 'invalid'}>
                {formatHint}
              </div>
            {/if}
            {#if lookupResult && inputEcuId.trim()}
              <div class="lookup-meta">
                <span>MCU: <b>{lookupResult.mcuFamily}</b></span>
                <span>Start Offset: <b>{lookupResult.startOffset}</b></span>
                <span>Checksum Offset: <b>{lookupResult.checksumOffset}</b></span>
                <span>BIN: <b>{lookupResult.recommendedBinName}</b></span>
              </div>
            {/if}
            {#if validationStatus}
              <div class="validation-result" class:success={validatedModelInfo && validatedModelInfo.model !== 'Custom ECU'}>
                {validationStatus}
              </div>
            {/if}
          </div>
          <div class="live-grid">
            <div>RPM: {$liveData.rpm}</div>
            <div>TPS: {$liveData.tps}%</div>
            <div>ECT: {$liveData.ect}°C</div>
            <div>MAP: {$liveData.map} kPa</div>
          </div>

          <div class="db-logs-card">
            <div class="db-logs-header">
              <span>☁️ ประวัติการจูนจากฐานข้อมูลกลาง ({$recentTuningLogs.length})</span>
              <span>กล่องกำหนดเอง: {$customEcuModels.length} รุ่น</span>
            </div>
            {#if $recentTuningLogs.length === 0}
              <div class="db-empty">ยังไม่มีประวัติการแฟลชในฐานข้อมูลกลาง</div>
            {:else}
              <div class="db-log-list">
                {#each $recentTuningLogs.slice(0, 5) as log}
                  <div class="db-log-item">
                    <b>{log.modelPn}</b> · <span>{log.fileName}</span> · <small>{log.details}</small>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {:else if $currentView === 'tune'}
        <TunePanel />
      {:else if $currentView === 'dtc'}
        <DtcPanel />
      {/if}
    </section>

    <RightPanel />
  </div>

  <div class="status-bar">
    <div class="status-top">
      <div class="com-badge" role="button" tabindex="0" on:click={() => showPorts = !showPorts} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); showPorts = !showPorts } }} style="cursor:pointer">
        : {$comPort} {#if showPorts}▼{:else}▲{/if}
      </div>
      {#if showPorts}
        <div class="port-dropdown" role="listbox">
          {#each ports as p}
            <div class="port-item" role="option" tabindex="0" aria-selected={$comPort === p.name} on:click={() => { comPort.set(p.name); showPorts = false }} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); comPort.set(p.name); showPorts = false } }}>{p.name} - {p.type}</div>
          {/each}
        </div>
      {/if}
      <button class="connect-btn" class:connected={$isConnected} on:click={handleConnect} disabled={$isConnecting}>
        {#if $isConnecting}⏳ กำลังเชื่อม...{:else if $isConnected}✅ เชื่อมต่อแล้ว{:else}🔌 เชื่อมต่อ{/if}
      </button>
      <div class="status-grid">
        <span>🟡 <b>{$logMessage}</b></span>
        <span>ขนาดไฟล์: <b class:wait={$fileSize.includes('รอ') || $fileSize === 'ไม่มีไฟล์'}>{$fileSize}</b></span>
        <span>อีซีเอ็ม ไอดี: <b class:wait={$ecmId === '--'}>{$ecmId}</b></span>
      </div>
    </div>
    <div class="status-bottom">
      <span>คำเตือน *** ห้าม!!! ปิด สวิตช์ กุญแจ ระหว่างแฟลช แบ็คอัพไฟล์เดิมอัตโนมัติ: {$ecuSettings.autoBackup ? 'เปิด' : 'ปิด'} ***</span>
      <span style="display:flex;gap:12px"><span>Baud: {$ecuSettings.baudRate}</span><span class="version">© REMAP V.5 Tauri Edition v5.0.7 ✅</span></span>
    </div>
  </div>

  {#if activeModal}
    <div class="modal-backdrop" role="presentation" on:click={closeModal}>
      <div class="modal-box" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1" on:click|stopPropagation on:keydown|stopPropagation>
        <div class="modal-header">
          <h3 id="modal-title">
            {#if activeModal === 'add-ecu'}📥 เพิ่มรหัสกล่อง ECU
            {:else if activeModal === 'reset-ecu'}🔄 รีเซตกล่อง ECU
            {:else if activeModal === 'profile'}📐 รูปโปร / แผนผังพินเอาท์ ECU
            {:else if activeModal === 'settings'}⚙️ ตั้งค่าโปรแกรม
            {:else if activeModal === 'extra'}🚀 ฟังก์ชั่นเพิ่มเติม
            {:else if activeModal === 'install'}📖 วิธีติดตั้งโปรแกรม
            {:else if activeModal === 'help'}❓ ช่วยเหลือ & เกี่ยวกับ
            {/if}
          </h3>
          <button class="close-modal" aria-label="ปิดหน้าต่าง" on:click={closeModal}>✕</button>
        </div>
        <div class="modal-body">
          {#if activeModal === 'add-ecu'}
            <div class="form-group">
              <label for="new-ecu-pn">รหัสพาร์ทกล่อง (Part Number):</label>
              <input id="new-ecu-pn" type="text" bind:value={newEcuPn} placeholder="เช่น 38770-K0Z-T01">
            </div>
            <div class="form-group">
              <label for="new-ecu-name">ชื่อรุ่นรถ / รายละเอียด:</label>
              <input id="new-ecu-name" type="text" bind:value={newEcuName} placeholder="เช่น Click 160 / PCX 160">
            </div>
            <div class="form-group">
              <label for="new-ecu-vendor">ค่ายกล่อง (Vendor):</label>
              <select id="new-ecu-vendor" bind:value={newEcuVendor}>
                <option value="Keihin">KEIHIN (Honda)</option>
                <option value="Shindengen">SHINDENGEN</option>
              </select>
            </div>
            <div class="form-group">
              <label for="new-ecu-size">ขนาดรอม (Flash Size):</label>
              <select id="new-ecu-size" bind:value={newEcuFlashKB}>
                <option value={48}>48 KB (R8C - Start 0x4000 / Cksum 0x7600)</option>
                <option value={56}>56 KB (H8C - Start 0x0000 / Cksum 0xDFEF)</option>
                <option value={64}>64 KB (R8C - Start 0x8000 / Cksum 0x0000)</option>
                <option value={96}>96 KB (R8C - Start 0x8000 / Cksum 0x0000)</option>
                <option value={128}>128 KB (M16C - Start 0xE0000 / Cksum 0xB000)</option>
              </select>
            </div>
            <button class="action-btn" on:click={saveNewEcu}>บันทึกรหัสกล่องลงฐานข้อมูล</button>
          {:else if activeModal === 'reset-ecu'}
            <p>การรีเซตกล่องจะทำการเคลียร์ค่า Fault Codes (DTC), รีเซต Flash Count / EEPROM และล้างค่าความจำระยะสั้นของกล่อง ECU</p>
            <div class="modal-actions">
              <button class="action-btn danger" disabled={isResetting} on:click={performReset}>
                {isResetting ? '⏳ กำลังรีเซต...' : 'ยืนยันรีเซตกล่อง'}
              </button>
              <button class="action-btn secondary" on:click={closeModal}>ยกเลิก</button>
            </div>
          {:else if activeModal === 'profile'}
            <div style="max-height:350px;overflow-y:auto;display:flex;flex-direction:column;gap:10px">
              <p><b>🔌 แผนผังพินเอาท์ K-Line / Back Door Flash Programmer (Honda ECU):</b></p>

              <div class="pinout-section">
                <b>1. พินเอาท์ K-Line (33 Pin / 39 Pin):</b>
                <ul class="pinout-list">
                  <li><b>PCX 150 / ADV 150 / Vario 125-150:</b> ใช้พอร์ต 33 Pin (K-Line Pin ขวาบน, +12V ขวาล่าง, GND ซ้ายล่าง)</li>
                  <li><b>Beat New / PCX 160 / Scoopy New / ADV 160:</b> รองรับกล่อง 33 Pin และ 39 Pin (Vario 160 MCU RH850)</li>
                  <li><b>CBR 250 RR / CBR 150R:</b> รองรับพอร์ต Keihin ECU ขนาดเล็ก</li>
                </ul>
              </div>

              <div class="pinout-section" style="margin-top:6px">
                <b>2. Back Door Flash Programmer (MCU V850E2 & RH850):</b>
                <ul class="pinout-list">
                  <li><b>Beat K25G-K81 (MCU V850E2):</b> ใช้พิน 1: GND, 3: Vcc, 6: Vcc, 7: B1, 9: Reset</li>
                  <li><b>Beat K1AA / ADV 160 & PCX 160 (MCU RH850):</b> ใช้พิน 1: Gnd, 2: B0, 6: Vcc, 7: B1, 8: B5, 9: Vcc</li>
                </ul>
              </div>
            </div>
          {:else if activeModal === 'settings'}
            <div class="form-group">
              <label for="baud-rate-select">ความเร็ว Baud Rate:</label>
              <select id="baud-rate-select" bind:value={baudRate}>
                <option value="10400">10,400 bps (มาตรฐาน Honda K-Line)</option>
                <option value="9600">9,600 bps</option>
                <option value="38400">38,400 bps</option>
              </select>
            </div>
            <div class="form-group checkbox-group">
              <label><input type="checkbox" bind:checked={autoBackup}> แบ็คอัพไฟล์เดิมอัตโนมัติก่อนจูน/เขียน (Backup Stock ROM)</label>
            </div>
            <button class="action-btn" on:click={saveSettings}>บันทึกตั้งค่า</button>
          {:else if activeModal === 'extra'}
            <div class="form-group checkbox-group">
              <label><input type="checkbox" bind:checked={extraE85}> เปิดโหมดน้ำมัน E85 (x1.35 Fuel Multiplier)</label>
            </div>
            <div class="form-group checkbox-group">
              <label><input type="checkbox" bind:checked={extraO2Off}> ปิดระบบเซนเซอร์ออกซิเจน (O2 Sensor Disable / Closed Loop Off)</label>
            </div>
            <div class="form-group checkbox-group">
              <label><input type="checkbox" bind:checked={extraPopBangs}> เปิดท่อลั่น / Pop & Bangs (Ignition Retard on Decel)</label>
            </div>
            <button class="action-btn" on:click={applyExtraOptions}>ใช้ฟังก์ชั่นพิเศษ</button>
          {:else if activeModal === 'install'}
            <ol class="install-steps">
              <li>ดาวน์โหลดและติดตั้งไดรเวอร์ USB Serial CH340 / FTDI ให้เรียบร้อย</li>
              <li>ต่อสาย USB Interface เข้ากับคอมพิวเตอร์และเลือกพอร์ต COM ให้ตรงกันในโปรแกรม</li>
              <li>เปิดสวิตช์กุญแจรถจักรยานยนต์เพื่อจ่ายไฟให้กล่อง ECU</li>
              <li>โหลดไฟล์ BIN เดิมอ่านค่า ปรับแต่ง และกด "เริ่มแฟลช" ห้ามปิดสวิตช์กุญแจเด็ดขาด</li>
            </ol>
          {:else if activeModal === 'help'}
            <p><b>REMAP V.5 - Tauri Edition</b></p>
            <p>โปรแกรมสำหรับจูนกล่อง ECU มอเตอร์ไซค์ Honda (Keihin / Shindengen) รองรับการเปิดรอบ, เพิ่มน้ำมัน, ปรับองศาไฟจุดระเบิด, ลบโค้ด DTC และแฟลชรอม.</p>
            <p style="color:#00ffaa;margin-top:8px">เชื่อมต่อฐานข้อมูลกลางผู้สร้าง (Firestore) โดยไม่ต้องล็อกอิน.</p>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
.menu-btn{background:none;border:none;color:#9fb3c8;font-size:11px;cursor:pointer;padding:0;font-family:inherit}
.menu-btn:hover{color:#00d4ff;text-decoration:underline}
.modal-backdrop{position:fixed;inset:0;background:rgba(0,0,0,0.8);display:grid;place-items:center;z-index:1000;backdrop-filter:blur(4px)}
.modal-box{width:450px;max-width:90vw;background:#001525;border:1px solid #00aaff;border-radius:8px;box-shadow:0 0 30px rgba(0,170,255,.4);display:flex;flex-direction:column;overflow:hidden}
.modal-header{background:linear-gradient(180deg,#0a3a6a,#052040);padding:10px 14px;display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #005a9a}
.modal-header h3{color:#7ad4ff;font-size:13px;margin:0}
.close-modal{background:none;border:none;color:#ff5f56;font-size:16px;cursor:pointer;font-weight:700}
.modal-body{padding:16px;color:#a0d8ff;font-size:12px;display:flex;flex-direction:column;gap:12px}
.form-group{display:flex;flex-direction:column;gap:4px}
.form-group label{color:#7ad4ff;font-size:11px}
.form-group input, .form-group select{background:#051020;border:1px solid #005a9a;border-radius:4px;color:#fff;padding:6px 10px;font-size:12px}
.checkbox-group label{display:flex;align-items:center;gap:8px;cursor:pointer}
.action-btn{background:linear-gradient(180deg,#0a3a6a,#052040);border:1px solid #00aaff;border-radius:4px;color:#fff;padding:8px;font-weight:600;cursor:pointer;margin-top:6px}
.action-btn:hover:not(:disabled){background:#005a9a;box-shadow:0 0 10px rgba(0,170,255,.6)}
.action-btn:disabled{opacity:0.6;cursor:not-allowed}
.action-btn.danger{background:linear-gradient(180deg,#6a0a0a,#400505);border-color:#ff5f56}
.action-btn.danger:hover:not(:disabled){background:#9a0000}
.action-btn.secondary{background:#222;border-color:#555}
.modal-actions{display:flex;gap:8px}
.pinout-list, .install-steps{padding-left:18px;display:flex;flex-direction:column;gap:6px}
.pinout-list li, .install-steps li{color:#c0e8ff}
.ecu-input-card{background:linear-gradient(180deg,#051525,#020a14);border:1px solid #1a3a5a;border-radius:6px;padding:12px;display:flex;flex-direction:column;gap:8px;margin-bottom:4px;box-shadow:inset 0 1px 3px rgba(0,0,0,.6)}
.ecu-input-card label{color:#7ad4ff;font-size:11px}
.ecu-input-row{display:flex;gap:8px}
.ecu-input-row input{flex:1;background:#000;border:1px solid #005a9a;border-radius:4px;color:#fff;padding:6px 10px;font-size:12px}
.format-hint{font-size:11px;padding:4px 8px;border-radius:4px;background:#111;border:1px solid #333;color:#aaa}
.format-hint.valid{background:#0a1a0a;border-color:#00ffaa;color:#7affaa}
.format-hint.warning{background:#1a1a0a;border-color:#facc15;color:#fde047}
.format-hint.invalid{background:#1a0a0a;border-color:#ff5f56;color:#ff9f9a}
.lookup-meta{display:flex;flex-wrap:wrap;gap:10px;font-size:10px;color:#7a9ab8;background:#020a14;border:1px dashed #1a3a5a;padding:6px 8px;border-radius:4px}
.lookup-meta b{color:#00d4ff;font-family:'Courier New', monospace}
.validation-result{font-size:11px;padding:6px 10px;border-radius:4px;background:#1a0a0a;border:1px solid #ff5f56;color:#ff9f9a}
.validation-result.success{background:#0a1a0a;border-color:#00ffaa;color:#7affaa}
.db-logs-card{background:#051020;border:1px solid #1a3a5a;border-radius:6px;padding:10px;display:flex;flex-direction:column;gap:6px}
.db-logs-header{display:flex;justify-content:space-between;font-size:11px;color:#7ad4ff;font-weight:600;border-bottom:1px solid #1a3a5a;padding-bottom:4px}
.db-empty{font-size:10px;color:#7a9ab8}
.db-log-list{display:flex;flex-direction:column;gap:4px}
.db-log-item{font-size:10px;color:#a0d8ff;padding:4px 6px;background:rgba(0,100,200,0.08);border-radius:3px;border-left:2px solid #00aaff}
.db-log-item small{color:#7a9ab8}
.app-window{
  width:min(1280px,100vw);
  height:calc(100vh - 24px);
  margin:12px auto;
  background:#000;
  border:1px solid #1a2f4a;
  box-shadow:0 0 0 1px #0a1a2f, 0 20px 80px rgba(0,0,0,.9), 0 0 60px rgba(0,100,200,.15);
  overflow:hidden;
  position:relative;
  display:flex;
  flex-direction:column;
}
.title-bar{height:26px;flex-shrink:0;background:linear-gradient(180deg,#f0f0f0,#d0d0d0);display:flex;align-items:center;justify-content:space-between;padding:0 8px;font-size:12px;color:#222;font-weight:600}
.title-bar .win-title{display:flex;align-items:center;gap:6px}
.title-bar .win-title::before{content:'';width:12px;height:12px;background:radial-gradient(circle at 30% 30%, #ff5f56, #c0392b);border-radius:2px;display:inline-block}
.controls{display:flex;gap:2px}.controls span{width:26px;height:18px;background:linear-gradient(180deg,#fafafa,#d8d8d8);border:1px solid #aaa;display:grid;place-items:center;font-size:10px;cursor:pointer}
.controls span:last-child{background:#e81123;color:#fff;border-color:#a00}
.menu-bar{height:22px;flex-shrink:0;background:#0a0a0a;border-bottom:1px solid #1e2e44;display:flex;align-items:center;gap:18px;padding:0 12px;font-size:11px;color:#9fb3c8;white-space:nowrap;overflow-x:auto}
.menu-bar .datetime{margin-left:auto;color:#7a8a9a;font-size:10px}
.main-content{flex:1;min-height:0;overflow:hidden;display:grid;grid-template-columns:110px 1fr 340px;background:radial-gradient(800px 400px at 50% 0%, rgba(0,100,200,.15), transparent 70%),linear-gradient(180deg,#02060c,#000)}
.center-panel{position:relative;padding:8px 10px;display:flex;flex-direction:column;gap:8px;min-height:0;overflow-y:auto;background:radial-gradient(600px 300px at 50% 20%, rgba(0,120,255,.12), transparent)}
.dashboard-area{display:flex;flex-direction:column;gap:12px;padding:12px;color:#fff}
.live-grid{display:grid;grid-template-columns:repeat(auto-fill, minmax(120px, 1fr));gap:10px;background:#051020;padding:10px;border-radius:4px;border:1px solid #1a3a5a}
.status-bar{min-height:58px;flex-shrink:0;background:linear-gradient(180deg,#0a0a0a,#000);border-top:1px solid #1a2f4a;display:flex;flex-direction:column;padding:4px 8px;gap:2px;font-size:10px;position:relative}
.status-top{display:flex;align-items:center;gap:8px;position:relative}
.com-badge{background:linear-gradient(180deg,#0a3a6a,#052040);border:1px solid #00aaff;border-radius:3px;padding:2px 8px;color:#7ad4ff;font-weight:700;box-shadow:0 0 8px rgba(0,150,255,.4)}
.connect-btn{height:22px;padding:0 10px;background:linear-gradient(180deg,#0a3a6a,#052040);border:1px solid #00aaff;border-radius:3px;color:#7ad4ff;font-size:10px;cursor:pointer;transition:.2s}
.connect-btn:hover{box-shadow:0 0 10px rgba(0,200,255,.6);color:#fff}
.connect-btn.connected{background:linear-gradient(180deg,#0a5a3a,#052a20);border-color:#00ffaa;color:#7affaa}
.connect-btn:disabled{opacity:.6;cursor:not-allowed}
.port-dropdown{position:absolute;top:100%;left:0;background:#001a3a;border:1px solid #00aaff;border-radius:4px;z-index:100;box-shadow:0 4px 20px rgba(0,0,0,.8)}
.port-item{padding:6px 12px;font-size:11px;color:#7ad4ff;cursor:pointer;white-space:nowrap}
.port-item:hover{background:rgba(0,170,255,.2);color:#fff}
.status-grid{display:flex;gap:18px;color:#7a9ab8;flex-wrap:wrap}
.status-grid b{color:#a0d0ff;font-weight:600}
.status-grid .wait{color:#facc15}
.status-bottom{display:flex;justify-content:space-between;align-items:center;font-size:9px;color:#5a7a9a;border-top:1px dashed #0f243f;margin-top:2px;padding-top:2px}
.status-bottom .version{color:#00d4ff;font-weight:600}
@media(max-width:860px){
  .main-content{grid-template-columns:1fr;grid-template-rows:auto auto auto}
  .center-panel{order:-1}
  .status-bar{height:auto}
}
@media(max-width:600px){
  .app-window{margin:0;border:none;min-height:100vh}
  .status-grid{gap:8px;font-size:9px}
}
</style>
