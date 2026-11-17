<script lang="ts">
  import { get } from 'svelte/store'
  import { onDestroy } from 'svelte'
  import {
    comPort,
    isConnected,
    selectedBin,
    selectedFileName,
    selectedModel,
    logMessage,
    liveData,
    isFlashing,
    flashProgress,
    customEcuModels,
    ecuSettings,
    extraOptions,
    recentTuningLogs,
    binChecksum,
    binChecksumValid
  } from '../lib/stores'
  import { applyTune, presetForModel, type TuneStyle } from '../lib/tune'
  import { ECU_MODELS, findModelByPn } from '../lib/ecu-models'
  import { writeRom, isTauriRuntime } from '../lib/ecu'
  import { analyzeBinData } from '../lib/ecu-analyzer'
  import { db, handleFirestoreError, OperationType } from '../lib/firebase'
  import { collection, addDoc } from 'firebase/firestore'

  async function handleFlash() {
    if ($isFlashing) return
    const bin = get(selectedBin)
    if (!bin) {
      logMessage.set('⚠️ เลือกไฟล์ BIN ก่อนเริ่มแฟลช')
      return
    }
    if (isTauriRuntime() && !$isConnected) {
      logMessage.set('⚠️ กรุณาเชื่อมต่อสายจูน ECU ก่อนแฟลชในโหมด Desktop')
      return
    }

    isFlashing.set(true)
    flashProgress.set(0)
    logMessage.set('⚡ กำลังแฟลช... ห้ามปิดสวิตช์กุญแจเด็ดขาด!')

    try {
      for (let i = 0; i <= 90; i += 5) {
        flashProgress.set(i)
        await new Promise(r => setTimeout(r, 60))
      }

      const writeMsg = await writeRom(get(comPort), bin)
      flashProgress.set(100)

      const logData = {
        id: 'log_' + Date.now(),
        creator: 'creator_central',
        fileName: (get(selectedFileName) || 'UNTITLED.bin').slice(0, 256),
        modelPn: (get(selectedModel) || 'UNKNOWN').slice(0, 64),
        details: (lastResult || 'Flash ROM').slice(0, 1024),
        createdAt: new Date().toISOString()
      }

      try {
        await addDoc(collection(db, 'tuning_logs'), logData)
        recentTuningLogs.update((prev) => [logData, ...prev].slice(0, 15))
        logMessage.set(`✅ ${writeMsg} | บันทึกประวัติลงฐานข้อมูลกลางแล้ว`)
      } catch (fsErr) {
        try {
          handleFirestoreError(fsErr, OperationType.CREATE, 'tuning_logs')
        } catch {
          logMessage.set(`✅ ${writeMsg} (⚠️ บันทึก Log คลาวด์ไม่สำเร็จ)`)
        }
      }
    } catch (err: any) {
      flashProgress.set(0)
      logMessage.set(`❌ แฟลชล้มเหลว: ${err?.message ?? err}`)
    } finally {
      isFlashing.set(false)
    }
  }

  let backup: Uint8Array | null = null
  let lastFile = ''
  let lastResult = 'ยังไม่จูน - โหลดไฟล์ BIN ก่อน'
  let limiterPct = 20
  let fuelPct = 20
  let ignDeg = 3

  // auto-tune (realtime trim demo on the live data stream)
  let autoOn = false
  let autoTrim = 0
  let autoTimer: ReturnType<typeof setInterval> | null = null

  // ไฟล์ใหม่ -> ทิ้ง backup + หยุด auto
  $: if ($selectedFileName !== lastFile) {
    lastFile = $selectedFileName
    backup = null
    stopAuto()
    lastResult = 'ไฟล์ใหม่ - backup เคลียร์แล้ว'
  }
  $: allModels = [...$customEcuModels, ...ECU_MODELS]
  $: currentPreset = presetForModel($selectedModel, $selectedFileName)
  $: presetId = currentPreset.id
  $: hasLimiterAddr = currentPreset.limiterU16.length > 0 || currentPreset.limiterU8.length > 0
  $: modelInfo = $selectedModel
    ? ($customEcuModels.find((m) => m.pn === $selectedModel) ?? findModelByPn($selectedModel))
    : undefined

  function needBin(): Uint8Array | null {
    const bin = get(selectedBin)
    if (!bin) logMessage.set('⚠️ โหลดไฟล์ BIN ก่อนจูน')
    return bin
  }

  function ensureBackup(bin: Uint8Array): void {
    if ($ecuSettings.autoBackup && !backup) {
      backup = new Uint8Array(bin)
      logMessage.set('💾 เก็บ backup ไฟล์เดิมอัตโนมัติแล้ว')
    }
  }

  function run(style: TuneStyle, opt: { factor?: number; amount?: number }, label: string): void {
    const bin = needBin()
    if (!bin) return
    ensureBackup(bin)
    const r = applyTune(bin, get(selectedFileName), style, opt, presetForModel(get(selectedModel), get(selectedFileName)))
    if (r.changed === 0) {
      logMessage.set(`⚠️ ${label}: ${r.detail}`)
      return
    }
    selectedBin.set(r.data)
    const reanalyzed = analyzeBinData(r.data, get(selectedFileName), get(customEcuModels))
    binChecksum.set(reanalyzed.checksum)
    binChecksumValid.set(reanalyzed.isValidChecksum)
    lastResult = `${label}: ${r.detail}`
    logMessage.set(`🔧 ${label} เสร็จ - ${r.changed} bytes`)
  }

  function toggleAuto(): void {
    if (autoOn) {
      stopAuto()
      logMessage.set(`🤖 ปิดออโต้จูน (trim ค้าง ${autoTrim >= 0 ? '+' : ''}${autoTrim}%)`)
      return
    }
    if (!needBin()) return
    autoOn = true
    autoTrim = 0
    logMessage.set('🤖 ออโต้จูน ON - ปรับ trim ตาม RPM/TPS สด')
    autoTimer = setInterval(() => {
      const lv = get(liveData)
      const target = lv.rpm > 4000 && lv.tps > 60 ? 8 : lv.rpm > 2500 ? 3 : 0
      autoTrim += Math.sign(target - autoTrim) * Math.min(1, Math.abs(target - autoTrim))
    }, 1000)
  }

  function stopAuto(): void {
    autoOn = false
    if (autoTimer) {
      clearInterval(autoTimer)
      autoTimer = null
    }
  }

  function resetStock(): void {
    if (!backup) {
      logMessage.set('⚠️ ยังไม่มี backup - จูนก่อนหรือโหลดไฟล์ใหม่')
      return
    }
    const restored = new Uint8Array(backup)
    selectedBin.set(restored)
    const reanalyzed = analyzeBinData(restored, get(selectedFileName), get(customEcuModels))
    binChecksum.set(reanalyzed.checksum)
    binChecksumValid.set(reanalyzed.isValidChecksum)
    stopAuto()
    lastResult = 'รีเซ็ทเป็นค่าเดิมก่อนจูนแล้ว'
    logMessage.set('↩️ รีเซ็ทกลับไฟล์เดิมแล้ว')
  }

  onDestroy(stopAuto)
</script>

<div class="tune-panel">
  <div class="tune-title">
    <span>🔧 จูน 7 ปุ่ม</span>
    <span class="preset">preset: {presetId} | Baud: {$ecuSettings.baudRate}</span>
  </div>
  <div class="model-row">
    <select class="model-select" bind:value={$selectedModel} aria-label="เลือกรุ่นกล่อง ECU">
      <option value="">-- เลือกรุ่นกล่อง (หรือเดาจากชื่อไฟล์) --</option>
      {#each allModels as m}
        <option value={m.pn}>{m.model} {m.years} - {m.pn}</option>
      {/each}
    </select>
    {#if modelInfo}
      <div class="model-info">
        {modelInfo.cal ? `Cal ${modelInfo.cal} | ` : ''}{modelInfo.vendor ?? ''}{modelInfo.vendor ? ' | ' : ''}{modelInfo.flashKB ? `${modelInfo.flashKB}KB` : ''} | {modelInfo.gearbox === 'geared' ? 'มีเกียร์' : 'ออโต้'}
      </div>
    {/if}
    {#if $extraOptions.e85Mode || $extraOptions.o2Off || $extraOptions.popBangs}
      <div class="extra-badges">
        {#if $extraOptions.e85Mode}<span class="badge">🌽 E85 ON</span>{/if}
        {#if $extraOptions.o2Off}<span class="badge">🚫 O2 OFF</span>{/if}
        {#if $extraOptions.popBangs}<span class="badge">💥 POP & BANGS</span>{/if}
      </div>
    {/if}
  </div>
  <div class="tune-grid">
    <button
      class="tune-btn"
      disabled={!hasLimiterAddr}
      title={hasLimiterAddr ? 'ปรับเพิ่มรอบตัด (RPM Limiter)' : 'รุ่นนี้ยังไม่มี address เปิดรอบที่ยืนยันแล้ว'}
      on:click={() => run('limiter', { factor: 1 + limiterPct / 100 }, `เปิดรอบ +${limiterPct}%`)}>
      <span class="ticon">🚀</span><span>{hasLimiterAddr ? `เปิดรอบ +${limiterPct}%` : 'เปิดรอบ (N/A)'}</span>
    </button>
    <button class="tune-btn" on:click={() => run('fuel', { factor: 1 + fuelPct / 100 }, `น้ำมัน +${fuelPct}%`)}>
      <span class="ticon">⛽</span><span>น้ำมัน +{fuelPct}%</span>
    </button>
    <button class="tune-btn" on:click={() => run('ignition', { amount: ignDeg }, `ไฟแก่ +${ignDeg}°`)}>
      <span class="ticon">⚡</span><span>ไฟแก่ +{ignDeg}°</span>
    </button>
    <button class="tune-btn" on:click={() => run('lope', { amount: 12 }, 'หอบเดินเบา')}>
      <span class="ticon">🥁</span><span>หอบเดินเบา</span>
    </button>
    <button class="tune-btn" on:click={() => run('e85', {}, 'E85')}>
      <span class="ticon">🌽</span><span>E85 x1.35</span>
    </button>
    <button class="tune-btn" class:auto={autoOn} on:click={toggleAuto}>
      <span class="ticon">🤖</span><span>{autoOn ? `ออโต้ ON (trim ${autoTrim >= 0 ? '+' : ''}${autoTrim}%)` : 'ออโต้จูน'}</span>
    </button>
    <button class="tune-btn reset" on:click={resetStock}>
      <span class="ticon">↩️</span><span>รีเซ็ทค่าเดิม</span>
    </button>
  </div>
  <div class="tune-opts">
    <label>limiter <input type="range" min="10" max="50" step="10" bind:value={limiterPct}><b>+{limiterPct}%</b></label>
    <label>น้ำมัน <input type="range" min="10" max="40" step="10" bind:value={fuelPct}><b>+{fuelPct}%</b></label>
    <label>ไฟ <input type="range" min="2" max="6" step="1" bind:value={ignDeg}><b>+{ignDeg}°</b></label>
  </div>
  <div class="tune-result">{lastResult}</div>

  <div class="flash-section">
    <div class="progress-wrap">
      <div class="progress-bar" style="width: {$flashProgress}%"></div>
      <span class="progress-text">{$flashProgress}%</span>
    </div>
    <button class="flash-btn" class:flashing={$isFlashing} disabled={$isFlashing} on:click={handleFlash}>
      {#if $isFlashing}⏳ กำลังแฟลช...{:else}⚡ เริ่มแฟลช{/if}
    </button>
  </div>
</div>

<style>
  .flash-section { display: flex; flex-direction: column; gap: 8px; margin-top: 10px; }
  .progress-wrap{height:22px;background:linear-gradient(180deg,#0a0a0a,#1a1a1a);border:1px solid #1a3a5a;border-radius:10px;padding:2px;position:relative;overflow:hidden;box-shadow:inset 0 1px 4px rgba(0,0,0,.8)}
  .progress-bar{height:100%;background:linear-gradient(90deg,#003366,#0066cc,#00ccff,#0066cc);border-radius:8px;transition:width .2s;position:relative;overflow:hidden}
  .progress-text{position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);font-size:10px;color:#fff;font-weight:700;text-shadow:0 1px 2px #000}
  .flash-btn{height:44px;background:linear-gradient(180deg,#0a3a6a 0%, #084080 20%, #052a55 50%, #031a33 100%);border:1px solid #00ccff;border-radius:6px;box-shadow:0 0 12px rgba(0,180,255,.6), inset 0 1px 0 rgba(255,255,255,.3);color:#fff;font-size:14px;font-weight:700;cursor:pointer;display:flex;align-items:center;justify-content:center;gap:8px;transition:.25s;position:relative;overflow:hidden}
  .flash-btn:hover:not(:disabled){transform:translateY(-2px);box-shadow:0 0 20px rgba(0,220,255,.9),0 0 40px rgba(0,120,255,.5)}
  .flash-btn:disabled{opacity:0.75;cursor:not-allowed}
  .flash-btn.flashing{animation:pulse 1s infinite;background:linear-gradient(180deg,#0a6a3a,#084040)}
  @keyframes pulse{0%,100%{box-shadow:0 0 12px rgba(0,255,150,.6)}50%{box-shadow:0 0 24px rgba(0,255,150,1)}}
  .tune-title {
    color: #7ad4ff;
    font-size: 11px;
    font-weight: 600;
    margin-bottom: 8px;
    padding-bottom: 4px;
    border-bottom: 1px solid #005a9a;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .preset {
    font-size: 9px;
    color: #00d4ff;
    font-weight: 600;
  }
  .model-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 8px;
  }
  .model-select {
    height: 26px;
    background: #0a3a6a;
    border: 1px solid #00aaff;
    border-radius: 4px;
    color: #a0d8ff;
    font-size: 11px;
    padding: 0 6px;
  }
  .model-info {
    font-size: 10px;
    color: #00ffaa;
  }
  .extra-badges {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }
  .badge {
    font-size: 9px;
    padding: 2px 6px;
    border-radius: 3px;
    background: #052040;
    border: 1px solid #00ccff;
    color: #7ad4ff;
  }
  .tune-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
  }
  .tune-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    padding: 7px 4px;
    background: #0a3a6a;
    border: 1px solid #00aaff;
    border-radius: 4px;
    color: #a0d8ff;
    font-size: 10px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .tune-btn:hover:not(:disabled) {
    background: #005a9a;
    box-shadow: 0 0 8px rgba(0, 170, 255, 0.4);
  }
  .tune-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
  .tune-btn.auto {
    background: #0a5a3a;
    border-color: #00ffaa;
    color: #7affaa;
  }
  .tune-btn.reset {
    background: #5a2a0a;
    border-color: #ffaa00;
    color: #ffcc7a;
  }
  .ticon { font-size: 12px; }
  .tune-opts {
    display: flex;
    gap: 12px;
    margin-top: 8px;
    font-size: 10px;
    color: #7a9ab8;
    flex-wrap: wrap;
  }
  .tune-opts label { display: flex; align-items: center; gap: 6px; }
  .tune-opts input[type="range"] { width: 90px; accent-color: #00aaff; }
  .tune-opts b { color: #00ffaa; min-width: 38px; }
  .tune-result {
    margin-top: 6px;
    font-size: 10px;
    color: #00ffaa;
    font-family: 'Courier New', monospace;
    word-break: break-all;
  }
</style>
