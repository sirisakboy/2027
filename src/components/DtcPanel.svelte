<script lang="ts">
  import { isConnected, logMessage } from '../lib/stores'
  import { getFaults, clearFaults, type FaultSet } from '../lib/ecu'
  import { describeDtc, searchDtc } from '../lib/dtc'

  let faults: FaultSet = { current: [], past: [] }
  let reading = false
  let clearing = false
  let hasRead = false
  let dtcQuery = ''

  $: lookupResults = dtcQuery.trim() ? searchDtc(dtcQuery) : []

  async function handleRead() {
    if (!$isConnected) {
      logMessage.set('⚠️ เชื่อมต่อ ECU ก่อนอ่าน DTC')
      return
    }
    reading = true
    logMessage.set('🔍 กำลังอ่าน DTC...')
    try {
      faults = await getFaults()
      hasRead = true
      const n = faults.current.length + faults.past.length
      logMessage.set(n === 0 ? '✅ ไม่มี DTC - กล่องปกติ' : `⚠️ พบ ${n} รหัสผิดปกติ`)
    } catch (err: any) {
      logMessage.set(`❌ อ่าน DTC ไม่สำเร็จ: ${err?.message ?? err}`)
    } finally {
      reading = false
    }
  }

  async function handleClear() {
    if (!hasRead || (faults.current.length === 0 && faults.past.length === 0)) {
      logMessage.set('⚠️ ไม่มี DTC ให้ลบ - อ่านก่อน')
      return
    }
    clearing = true
    try {
      const msg = await clearFaults()
      faults = { current: [], past: [] }
      logMessage.set(`✅ ${msg}`)
    } catch (err: any) {
      logMessage.set(`❌ ลบ DTC ไม่สำเร็จ: ${err?.message ?? err}`)
    } finally {
      clearing = false
    }
  }
</script>

<div class="dtc-panel">
  <div class="dtc-title">🔍 DTC โค้ดผิดปกติ (K-Line / MIL / OBD-II)</div>
  <div class="dtc-actions">
    <button class="dtc-btn" on:click={handleRead} disabled={reading}>
      {reading ? '⏳ กำลังอ่าน...' : '📖 อ่าน DTC'}
    </button>
    <button class="dtc-btn danger" on:click={handleClear} disabled={clearing}>
      {clearing ? '⏳ กำลังลบ...' : '🧹 ลบ DTC'}
    </button>
  </div>
  {#if !hasRead}
    <div class="dtc-empty">ยังไม่อ่าน - กด “อ่าน DTC” (ต้องเชื่อมต่อ ECU ก่อน)</div>
  {:else if faults.current.length === 0 && faults.past.length === 0}
    <div class="dtc-ok">✅ No DTC - กล่องปกติ</div>
  {:else}
    {#if faults.current.length > 0}
      <div class="dtc-group">ปัจจุบัน ({faults.current.length})</div>
      {#each faults.current as code}
        {@const d = describeDtc(code)}
        <div class="dtc-item cur"><b>{code}</b> {d.th} <span>({d.symptom})</span></div>
      {/each}
    {/if}
    {#if faults.past.length > 0}
      <div class="dtc-group">เคยเกิด ({faults.past.length})</div>
      {#each faults.past as code}
        {@const d = describeDtc(code)}
        <div class="dtc-item past"><b>{code}</b> {d.th} <span>({d.symptom})</span></div>
      {/each}
    {/if}
  {/if}

  <div class="dtc-lookup">
    <div class="dtc-group">📚 ค้นหารหัสโค้ด DTC / OBD-II (ใช้งานได้แม้ออฟไลน์)</div>
    <input
      type="text"
      class="dtc-search"
      bind:value={dtcQuery}
      placeholder="ค้นหาเช่น P0120, 07-01, MIL 8, MAP, หัวฉีด..."
      aria-label="ค้นหารหัส DTC"
    />
    {#if dtcQuery.trim()}
      {#if lookupResults.length === 0}
        <div class="dtc-empty">ไม่พบรหัสที่ตรงกับ "{dtcQuery}"</div>
      {:else}
        <div class="dtc-results">
          {#each lookupResults as item}
            <div class="dtc-item lookup">
              <div class="lookup-top"><b>{item.code}</b> <span class="type-tag">{item.type}</span></div>
              <div>{item.th} <span>— {item.symptom}</span></div>
            </div>
          {/each}
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .dtc-panel {
    margin-top: 8px;
    padding: 10px;
    background: #001525;
    border: 1px solid #005a9a;
    border-radius: 6px;
    flex-shrink: 0;
  }
  .dtc-title {
    color: #7ad4ff;
    font-size: 11px;
    font-weight: 600;
    margin-bottom: 8px;
    padding-bottom: 4px;
    border-bottom: 1px solid #005a9a;
  }
  .dtc-actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
    margin-bottom: 8px;
  }
  .dtc-btn {
    padding: 7px 4px;
    background: #0a3a6a;
    border: 1px solid #00aaff;
    border-radius: 4px;
    color: #a0d8ff;
    font-size: 10px;
    cursor: pointer;
  }
  .dtc-btn:hover:not(:disabled) {
    background: #005a9a;
  }
  .dtc-btn:disabled {
    opacity: 0.6;
    cursor: wait;
  }
  .dtc-btn.danger {
    background: #5a2a0a;
    border-color: #ffaa00;
    color: #ffcc7a;
  }
  .dtc-empty, .dtc-ok {
    font-size: 10px;
    color: #7a9ab8;
    text-align: center;
    padding: 6px;
  }
  .dtc-ok { color: #00ffaa; }
  .dtc-group {
    font-size: 10px;
    color: #7ad4ff;
    font-weight: 600;
    margin: 6px 0 4px;
  }
  .dtc-item {
    font-size: 10px;
    padding: 4px 6px;
    border-radius: 3px;
    margin-bottom: 4px;
    color: #ffd4d4;
    background: rgba(255, 60, 60, 0.1);
    border: 1px solid rgba(255, 60, 60, 0.3);
  }
  .dtc-item.past {
    color: #ffe9b8;
    background: rgba(255, 170, 0, 0.08);
    border-color: rgba(255, 170, 0, 0.3);
  }
  .dtc-item.lookup {
    color: #d0f0ff;
    background: rgba(0, 140, 255, 0.08);
    border-color: rgba(0, 170, 255, 0.3);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .lookup-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .type-tag {
    font-size: 9px;
    color: #00ffaa;
  }
  .dtc-item span { color: #7a9ab8; }
  .dtc-lookup {
    margin-top: 10px;
    padding-top: 8px;
    border-top: 1px dashed #1a3a5a;
  }
  .dtc-search {
    width: 100%;
    box-sizing: border-box;
    background: #051020;
    border: 1px solid #005a9a;
    border-radius: 4px;
    color: #fff;
    padding: 6px 8px;
    font-size: 11px;
    margin-bottom: 6px;
  }
  .dtc-results {
    max-height: 220px;
    overflow-y: auto;
  }
</style>
