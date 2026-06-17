<script>
  let { onclose } = $props();
</script>

<div class="overlay" onclick={onclose}>
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <button class="close" onclick={onclose}>✕</button>

    <div class="logo-wrap">
      <span class="logo">⌗</span>
      <h1>Transcoder</h1>
      <span class="version">v0.2.0</span>
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
    background: var(--bg-elevated);
    border: 1px solid var(--border);
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
    color: var(--text-muted);
    font-size: 14px;
    cursor: pointer;
    padding: 4px 6px;
    border-radius: 4px;
  }
  .close:hover { background: var(--bg-hover); color: var(--accent); }
  .logo-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .logo {
    font-size: 28px;
    color: var(--accent);
    line-height: 1;
  }
  h1 {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-strong);
    letter-spacing: -0.01em;
  }
  .version {
    font-size: 11px;
    background: var(--accent-soft);
    color: var(--accent);
    border: 1px solid #7c8ef740;
    border-radius: 4px;
    padding: 2px 7px;
    font-family: monospace;
  }
  .tagline {
    font-size: 12.5px;
    color: var(--text-subtle);
    line-height: 1.6;
  }
  .divider {
    height: 1px;
    background: var(--border-soft);
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
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: .05em;
    font-weight: 600;
    width: 70px;
    flex-shrink: 0;
  }
  .value {
    font-size: 13px;
    color: var(--text-secondary);
  }
  .link {
    color: var(--accent);
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
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 3px 10px;
    color: var(--text-subtle);
    font-family: monospace;
  }
</style>
