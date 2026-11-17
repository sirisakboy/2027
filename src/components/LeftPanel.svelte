<script lang="ts">
  import { isConnected, currentView } from '../lib/stores'
  
  const menuItems = [
    { id: 'dashboard', label: 'DASHBOARD', icon: '🏠' },
    { id: 'tune', label: 'TUNE', icon: '🛠️' },
    { id: 'dtc', label: 'DTC', icon: '🔍' },
  ]
</script>

<div class="left-panel">
  <div class="panel-title">เมนูหลัก</div>
  <div class="menu-list">
    {#each menuItems as item}
      <button class="menu-btn" class:active={$currentView === item.id} on:click={() => currentView.set(item.id as any)}>
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </button>
    {/each}
  </div>
  
  <div class="conn-badge" class:online={$isConnected}>
    <span class="dot"></span>
    <span>{$isConnected ? 'ออนไลน์ (ECU)' : 'ออฟไลน์ (ROM)'}</span>
  </div>
</div>

<style>
  .left-panel {
    width: 110px;
    min-height: 0;
    overflow-y: auto;
    background: linear-gradient(90deg, #0a1a2f, #000);
    border-right: 1px solid #1a3a5a;
    padding: 8px;
    display: flex;
    flex-direction: column;
  }
  .panel-title {
    color: #7ad4ff;
    font-size: 10px;
    font-weight: 600;
    margin-bottom: 8px;
    padding-bottom: 4px;
    border-bottom: 1px solid #1a3a5a;
  }
  .menu-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .menu-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    background: #0a3a6a;
    border: 1px solid #00aaff;
    border-radius: 4px;
    color: #a0d8ff;
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .menu-btn:hover {
    background: #005a9a;
    box-shadow: 0 0 10px rgba(0, 170, 255, 0.5);
  }
  .menu-btn.active {
    background: linear-gradient(180deg, #0088cc, #004488);
    border-color: #00ffff;
    color: #ffffff;
    font-weight: 700;
    box-shadow: 0 0 12px rgba(0, 255, 255, 0.6);
  }
  .icon { font-size: 12px; }
  .label { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .conn-badge {
    margin-top: auto;
    padding: 6px 4px;
    background: rgba(0, 0, 0, 0.6);
    border: 1px solid #1a3a5a;
    border-radius: 4px;
    font-size: 9px;
    color: #7a9ab8;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    text-align: center;
  }
  .conn-badge.online {
    border-color: #00ffaa;
    color: #7affaa;
    background: rgba(0, 50, 25, 0.5);
  }
  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #ff5f56;
    display: inline-block;
  }
  .conn-badge.online .dot {
    background: #00ffaa;
    box-shadow: 0 0 6px #00ffaa;
  }
</style>