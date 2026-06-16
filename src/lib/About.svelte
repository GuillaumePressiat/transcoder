<script>
  let { onclose } = $props();
</script>

<div class="overlay" onclick={onclose}>
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <button class="close" onclick={onclose}>✕</button>

    <div class="logo-wrap">
      <span class="logo">⌗</span>
      <h1>Transcoder</h1>
      <span class="version">v0.1.0</span>
    </div>

    <p class="tagline">Transposition et gestion de listes de codes médicaux<br/>CCAM · CIM-10 · FINESS · ETCetera</p>

    <div class="divider"></div>

    <div class="meta">
      <div class="row">
        <span class="label">Auteur</span>
        <span class="value">Guillaume Pressiat</span>
      </div>
      <div class="row">
        <span class="label">Source</span>
        <a class="value link" href="https://github.com/GuillaumePressiat/stringfix" target="_blank">
          github.com/GuillaumePressiat/stringfix
        </a>
      </div>
      <div class="row">
        <span class="label">Basé sur</span>
        <span class="value">Shiny app — stringfix</span>
      </div>
    </div>

    <div class="divider"></div>

    <div class="stack">
      <span class="chip">Tauri 2</span>
      <span class="chip">Rust</span>
      <span class="chip">SvelteKit 2</span>
      <span class="chip">Svelte 5</span>
    </div>

  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }
  .modal {
    background: #161820;
    border: 1px solid #252830;
    border-radius: 12px;
    padding: 32px 36px;
    width: 400px;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }
  .close {
    position: absolute;
    top: 14px; right: 16px;
    background: none;
    border: none;
    color: #5a5f75;
    font-size: 14px;
    cursor: pointer;
    padding: 4px 6px;
    border-radius: 4px;
  }
  .close:hover { background: #1e2130; color: #9098b8; }

  .logo-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .logo {
    font-size: 28px;
    color: #7c8ef7;
    line-height: 1;
  }
  h1 {
    font-size: 20px;
    font-weight: 600;
    color: #f0f2f8;
    letter-spacing: -0.01em;
  }
  .version {
    font-size: 11px;
    background: #7c8ef720;
    color: #7c8ef7;
    border: 1px solid #7c8ef740;
    border-radius: 4px;
    padding: 2px 7px;
    font-family: monospace;
  }
  .tagline {
    font-size: 12.5px;
    color: #6b7294;
    line-height: 1.6;
  }
  .divider {
    height: 1px;
    background: #1e2130;
  }
  .meta {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .row {
    display: flex;
    align-items: baseline;
    gap: 12px;
  }
  .label {
    font-size: 11px;
    color: #5a5f75;
    text-transform: uppercase;
    letter-spacing: .05em;
    font-weight: 600;
    width: 70px;
    flex-shrink: 0;
  }
  .value {
    font-size: 13px;
    color: #c0c4d8;
  }
  .link {
    color: #7c8ef7;
    text-decoration: none;
    font-size: 12px;
    font-family: monospace;
  }
  .link:hover { text-decoration: underline; }
  .stack {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }
  .chip {
    font-size: 11px;
    background: #1a1d28;
    border: 1px solid #252830;
    border-radius: 5px;
    padding: 3px 10px;
    color: #6b7294;
    font-family: monospace;
  }
</style>
