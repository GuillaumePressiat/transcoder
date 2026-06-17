<script>
  import { transcodeLine, transcodeColumn, convertFormat, copyToClipboard } from '$lib/api.js';
  import { history, addHistory, clearHistory } from '$lib/history.js';
  import About from '$lib/About.svelte';
  
  let showAbout = $state(false);

  let theme = $state('dark');

  if (typeof localStorage !== 'undefined') {
    const saved = localStorage.getItem('transcoder-theme');
    if (saved === 'light' || saved === 'dark') theme = saved;
  }

  $effect(() => {
    document.documentElement.classList.toggle('light', theme === 'light');
    try { localStorage.setItem('transcoder-theme', theme); } catch {}
  });

  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
  }

  // ── Global options ──────────────────────────────────────────────────────
  let quote = $state('single');
  let separator = $state('comma_space');

  // ── Tab state ───────────────────────────────────────────────────────────
  let activeTab = $state('line');

  // Tab 1 – line
  let lineInput = $state('F072, G430, G431, G432, G433, G438, G439, G440, G441, G442, G443, G444, G448, G932, G971, R51');
  let lineOutputType = $state('list');
  let lineResult = $state(null);

  // Tab 2 – column
  let colInput = $state('BGBA001\nBGDA001\nBGDA002\nBGDA003\nBGDA004\nBGDA005\nBGDA006\nBGDA007\nBGDA008\nBGFA002\nBGFA003\nBGFA004');
  let colOutputType = $state('list');
  let colResult = $state(null);

  // Tab 3 – convert
  let cvtInput = $state('BGBA001\nBGDA001\nBGDA002\nBGDA003\nBGDA004\nBGDA005\nBGDA006\nBGDA007\nBGDA008\nBGFA002\nBGFA003');
  let cvtDirection = $state('column_to_line');
  let cvtResult = $state(null);

  // ── Feedback ────────────────────────────────────────────────────────────
  let copied = $state(false);
  let copiedId = $state(null);
  let errorMsg = $state('');

  // ── Show/hide history panel ─────────────────────────────────────────────
  let showHistory = $state(false);

  // ── Helpers ─────────────────────────────────────────────────────────────
  function currentResult() {
    if (activeTab === 'line') return lineResult;
    if (activeTab === 'col') return colResult;
    return cvtResult;
  }

  async function flashCopied() {
    copied = true;
    await new Promise(r => setTimeout(r, 1400));
    copied = false;
  }

  async function doCopy(text) {
    try {
      await copyToClipboard(text);
      flashCopied();
    } catch {
      // fallback
      await navigator.clipboard.writeText(text);
      flashCopied();
    }
  }

  async function copyHistoryEntry(entry) {
    copiedId = entry.id;
    await doCopy(entry.output);
    await new Promise(r => setTimeout(r, 1400));
    copiedId = null;
  }

  // ── Transcode actions ───────────────────────────────────────────────────
  async function runLine() {
    errorMsg = '';
    try {
      const res = await transcodeLine(lineInput, lineOutputType, quote, separator);
      lineResult = res;
      addHistory({ tab: 'Ligne', input: lineInput, output: res.output, count: res.count });
      await doCopy(res.output);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function runColumn() {
    errorMsg = '';
    try {
      const res = await transcodeColumn(colInput, colOutputType, quote, separator);
      colResult = res;
      addHistory({ tab: 'Colonne', input: colInput, output: res.output, count: res.count });
      await doCopy(res.output);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function runConvert() {
    errorMsg = '';
    try {
      const res = await convertFormat(cvtInput, cvtDirection, separator);
      cvtResult = res;
      addHistory({ tab: 'Conversion', input: cvtInput, output: res.output, count: res.count });
      await doCopy(res.output);
    } catch (e) {
      errorMsg = String(e);
    }
  }


  function selectActiveField() {
    const el = document.querySelector('.panel input[type="text"], .panel textarea');
    if (el) {
      el.focus();
      el.select();
    }
  }

  // ── Keyboard shortcuts ──────────────────────────────────────────────────
  function handleKeydown(e) {
    // Ctrl/Cmd + Enter → run current tab
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      if (activeTab === 'line') runLine();
      else if (activeTab === 'col') runColumn();
      else runConvert();
    }
    // Ctrl/Cmd + 1/2/3 → switch tabs
    if ((e.ctrlKey || e.metaKey) && (e.key === '1' || e.key === '&')) { e.preventDefault(); activeTab = 'line'; }
    if ((e.ctrlKey || e.metaKey) && (e.key === '2' || e.key === 'é')) { e.preventDefault(); activeTab = 'col'; }
    if ((e.ctrlKey || e.metaKey) && (e.key === '3' || e.key === '"')) { e.preventDefault(); activeTab = 'cvt'; }
    // Ctrl/Cmd + H → toggle history
    if ((e.ctrlKey || e.metaKey) && e.key === 'h') { e.preventDefault(); showHistory = !showHistory; }

    // Ctrl/Cmd + Alt → entrer dans le champ de l'onglet actif et tout sélectionner
    if ((e.ctrlKey || e.metaKey) && e.key == 'l') {
      e.preventDefault();
      selectActiveField();
    }
  }

  function formatTime(d) {
    return d.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="app">

  <!-- ── Header ─────────────────────────────────────────────────────────── -->
  <header>
    <div class="header-left">
      <span class="logo">⌗</span>
      <h1>Transcoder</h1>
      <span class="tagline">codes CCAM · CIM-10 · FINESS · ETCetera</span>
    </div>
    <div class="header-right" style="display:flex;gap:8px">
      <button class="btn-icon" onclick={() => showAbout = true} title="À propos">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4"/>
          <path d="M8 6v5M8 4.5v.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
        </svg>
      </button>
      <button class="btn-icon" onclick={toggleTheme} title={theme === 'dark' ? 'Thème clair' : 'Thème sombre'}>
    {#if theme === 'dark'}
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <circle cx="8" cy="8" r="3" stroke="currentColor" stroke-width="1.4"/>
        <path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.5 3.5l1.4 1.4M11.1 11.1l1.4 1.4M3.5 12.5l1.4-1.4M11.1 4.9l1.4-1.4" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
      </svg>
    {:else}
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M13 9.5A5.5 5.5 0 1 1 6.5 3a4.3 4.3 0 0 0 6.5 6.5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>
      </svg>
    {/if}
  </button>
      <button
        class="btn-icon"
        class:active={showHistory}
        onclick={() => showHistory = !showHistory}
        title="Historique (Ctrl+H)">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4"/>
          <path d="M8 4.5V8l2.5 1.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
        </svg>
        {#if $history.length > 0}
          <span class="badge">{$history.length}</span>
        {/if}
      </button>
    </div>
  </header>

  <!-- ── Global options ─────────────────────────────────────────────────── -->
  <div class="global-opts">
    <div class="opt-group">
      <label>Guillemets</label>
      <div class="seg">
        <button class:sel={quote==='single'} onclick={()=>quote='single'}>Simple '  '</button>
        <button class:sel={quote==='double'} onclick={()=>quote='double'}>Double "  "</button>
        <button class:sel={quote==='none'}   onclick={()=>quote='none'}>Aucun</button>
      </div>
    </div>
    <div class="opt-group">
      <label>Séparateur</label>
      <div class="seg">
        <button class:sel={separator==='comma_space'} onclick={()=>separator='comma_space'}><code>, </code></button>
        <button class:sel={separator==='comma'}       onclick={()=>separator='comma'}><code>,</code></button>
        <button class:sel={separator==='space'}       onclick={()=>separator='space'}>espace</button>
      </div>
    </div>
  </div>

  <!-- ── Main layout ────────────────────────────────────────────────────── -->
  <div class="main" class:with-history={showHistory}>

    <!-- ── Tabs + content ───────────────────────────────────────────────── -->
    <div class="workspace">
      <div class="tabs">
        <button class:active={activeTab==='line'} onclick={()=>activeTab='line'}>
          Ligne <kbd>⌃1</kbd>
        </button>
        <button class:active={activeTab==='col'}  onclick={()=>activeTab='col'}>
          Colonne <kbd>⌃2</kbd>
        </button>
        <button class:active={activeTab==='cvt'}  onclick={()=>activeTab='cvt'}>
          Ligne ⇔ Colonne <kbd>⌃3</kbd>
        </button>
      </div>

      <div class="panel">

        <!-- ── Tab 1 – Transcode line ────────────────────────────────────── -->
        {#if activeTab === 'line'}
          <div class="tab-content">
            <div class="field">
              <label for="line-input">Codes séparés (une ligne)</label>
              <input
                id="line-input"
                type="text"
                bind:value={lineInput}
                placeholder="F072, G430, G431…"
                spellcheck="false"
                onclick={(e) => { if (e.ctrlKey || e.metaKey) e.target.select(); }}
              />

            </div>
            <div class="field">
              <label>Format de sortie</label>
              <div class="seg">
                <button class:sel={lineOutputType==='list'} onclick={()=>lineOutputType='list'}>Liste</button>
                <button class:sel={lineOutputType==='sql'}  onclick={()=>lineOutputType='sql'}>SQL %like%</button>
                <button class:sel={lineOutputType==='pipe'} onclick={()=>lineOutputType='pipe'}>Pipe Regexp</button>
              </div>
            </div>
            <button class="run-btn" onclick={runLine}>
              Transposer &amp; Copier
              <span class="shortcut">⌃↵</span>
            </button>
            {#if lineResult}
              <div class="result-block">
                <div class="result-meta">
                  <span>{lineResult.count} code{lineResult.count > 1 ? 's' : ''}</span>
                  <button class="copy-btn" onclick={()=>doCopy(lineResult.output)}>
                    {copied ? '✓ Copié' : 'Copier'}
                  </button>
                </div>
                <pre class="result">{lineResult.output}</pre>
              </div>
            {/if}
          </div>

        <!-- ── Tab 2 – Transcode column ──────────────────────────────────── -->
        {:else if activeTab === 'col'}
          <div class="tab-content two-col">
            <div class="left">
              <div class="field">
                <label for="col-input">Colonne de codes (un par ligne)</label>
                <textarea
                  id="col-input"
                  bind:value={colInput}
                  rows="12"
                  placeholder="BGBA001&#10;BGDA001&#10;…"
                  spellcheck="false"
                  onclick={(e) => { if (e.ctrlKey || e.metaKey) e.target.select(); }}
                ></textarea>
              </div>
              <div class="field">
                <label>Format de sortie</label>
                <div class="seg">
                  <button class:sel={colOutputType==='list'} onclick={()=>colOutputType='list'}>Liste</button>
                  <button class:sel={colOutputType==='sql'}  onclick={()=>colOutputType='sql'}>SQL %like%</button>
                  <button class:sel={colOutputType==='pipe'} onclick={()=>colOutputType='pipe'}>Pipe Regexp</button>
                </div>
              </div>
              <button class="run-btn" onclick={runColumn}>
                Transposer &amp; Copier
                <span class="shortcut">⌃↵</span>
              </button>
            </div>
            <div class="right">
              {#if colResult}
                <div class="result-block full">
                  <div class="result-meta">
                    <span>{colResult.count} code{colResult.count > 1 ? 's' : ''}</span>
                    <button class="copy-btn" onclick={()=>doCopy(colResult.output)}>
                      {copied ? '✓ Copié' : 'Copier'}
                    </button>
                  </div>
                  <pre class="result">{colResult.output}</pre>
                </div>
              {:else}
                <div class="empty-right">Le résultat apparaîtra ici</div>
              {/if}
            </div>
          </div>

        <!-- ── Tab 3 – Convert ───────────────────────────────────────────── -->
        {:else}
          <div class="tab-content two-col">
            <div class="left">
              <div class="field">
                <label for="cvt-input">Entrée</label>
                <textarea
                  id="cvt-input"
                  bind:value={cvtInput}
                  rows="12"
                  placeholder="Collez votre liste ici…"
                  spellcheck="false"
                  onclick={(e) => { if (e.ctrlKey || e.metaKey) e.target.select(); }}
                ></textarea>
              </div>
              <div class="field">
                <label>Direction</label>
                <div class="seg">
                  <button class:sel={cvtDirection==='column_to_line'} onclick={()=>cvtDirection='column_to_line'}>
                    Colonne → Ligne
                  </button>
                  <button class:sel={cvtDirection==='line_to_column'} onclick={()=>cvtDirection='line_to_column'}>
                    Ligne → Colonne
                  </button>
                </div>
              </div>
              <button class="run-btn" onclick={runConvert}>
                Convertir &amp; Copier
                <span class="shortcut">⌃↵</span>
              </button>
            </div>
            <div class="right">
              {#if cvtResult}
                <div class="result-block full">
                  <div class="result-meta">
                    <button class="copy-btn" onclick={()=>doCopy(cvtResult.output)}>
                      {copied ? '✓ Copié' : 'Copier'}
                    </button>
                  </div>
                  <pre class="result">{cvtResult.output}</pre>
                </div>
              {:else}
                <div class="empty-right">Le résultat apparaîtra ici</div>
              {/if}
            </div>
          </div>
        {/if}

        {#if errorMsg}
          <div class="error">{errorMsg}</div>
        {/if}
      </div>
    </div>

    <!-- ── History panel ─────────────────────────────────────────────────── -->
    {#if showHistory}
      <aside class="history-panel">
        <div class="history-header">
          <span>Historique</span>
          <button class="btn-clear" onclick={clearHistory} title="Effacer">✕ Vider</button>
        </div>
        {#if $history.length === 0}
          <div class="history-empty">Aucune transformation encore</div>
        {:else}
          <ul class="history-list">
            {#each $history as entry (entry.id)}
              <li class="history-item">
                <div class="hi-meta">
                  <span class="hi-tab">{entry.tab}</span>
                  <span class="hi-count">{entry.count} codes</span>
                  <span class="hi-time">{formatTime(entry.ts)}</span>
                </div>
                <div class="hi-input" title={entry.input}>{entry.input.replace(/\n/g, ' · ')}</div>
                <div class="hi-output-row">
                  <pre class="hi-output">{entry.output}</pre>
                  <button
                    class="copy-btn small"
                    onclick={() => copyHistoryEntry(entry)}
                  >
                    {copiedId === entry.id ? '✓' : '⎘'}
                  </button>
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </aside>
    {/if}

  </div>

  <!-- ── Footer ─────────────────────────────────────────────────────────── -->
  <footer>
    <span>⌃↵ transposer</span>
    <span>⌃1/2/3 onglets</span>
    <span>⌃H historique</span>
    <span>⌃L select input</span>
    <span>⌃+clic select input</span>
  </footer>
    
  {#if showAbout}
    <About onclose={() => showAbout = false} />
  {/if}

</div>

<style>
/* ── Theme tokens ────────────────────────────────────────────────────── */
:global(:root) {
  --bg: #0f1117;
  --bg-elevated: #161820;
  --bg-surface: #1a1d28;
  --bg-surface-2: #13151e;
  --bg-hover: #1e2130;
  --border: #252830;
  --border-soft: #1e2130;
  --border-strong: #2a2d3a;
  --text: #e2e4ea;
  --text-strong: #f0f2f8;
  --text-secondary: #c0c4d8;
  --text-muted: #9098b8;
  --text-subtle: #6b7294;
  --text-faint: #7a82a0;
  --text-dim: #8088ac;
  --text-hover: #b0b8d4;
  --accent: #7c8ef7;
  --accent-hover: #6070e8;
  --accent-soft: #7c8ef720;
  --on-accent: #ffffff;
  --result-text: #a8c0ff;
  --hi-output-text: #8090c0;
  --copy-btn-bg: #2a2d3a;
  --copy-btn-border: #353848;
  --error-bg: #2d1a1a;
  --error-border: #5a2222;
  --error-text: #ff8080;
}

:global(:root.light) {
  --bg: #f4f5f9;
  --bg-elevated: #ffffff;
  --bg-surface: #ffffff;
  --bg-surface-2: #eceef4;
  --bg-hover: #e7e9f2;
  --border: #dadde6;
  --border-soft: #e3e5ee;
  --border-strong: #cfd2e2;
  --text: #20232c;
  --text-strong: #11131a;
  --text-secondary: #454a5e;
  --text-muted: #5b6178;
  --text-subtle: #6f7592;
  --text-faint: #7d839c;
  --text-dim: #6a7090;
  --text-hover: #3d4260;
  --accent: #7c8ef7;
  --accent-hover: #6070e8;
  --accent-soft: #7c8ef71a;
  --on-accent: #ffffff;
  --result-text: #3548c2;
  --hi-output-text: #41508a;
  --copy-btn-bg: #eef0f6;
  --copy-btn-border: #d7dae6;
  --error-bg: #fdecec;
  --error-border: #f3b9b9;
  --error-text: #c4302f;
}

/* ── Reset / base ────────────────────────────────────────────────────── */
:global(*, *::before, *::after) { box-sizing: border-box; margin: 0; padding: 0; }
:global(body) {
  font-family: 'Inter', 'Segoe UI', system-ui, sans-serif;
  font-size: 13px;
  background: var(--bg);
  color: var(--text);
  height: 100vh;
  overflow: hidden;
  transition: background .2s ease, color .2s ease;
}

/* ── App shell ───────────────────────────────────────────────────────── */
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

/* ── Header ──────────────────────────────────────────────────────────── */
header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 18px;
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}
.logo {
  font-size: 20px;
  color: var(--accent);
  line-height: 1;
}
h1 {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: -0.01em;
  color: var(--text-strong);
}
.tagline {
  font-size: 11px;
  color: var(--text-muted);
  margin-left: 4px;
}
.btn-icon {
  position: relative;
  background: none;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  color: var(--text-muted);
  padding: 5px 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 5px;
  transition: all .15s;
}
.btn-icon:hover, .btn-icon.active {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--accent);
}
.badge {
  position: absolute;
  top: -5px; right: -5px;
  background: var(--accent);
  color: var(--on-accent);
  font-size: 10px;
  font-weight: 700;
  border-radius: 10px;
  padding: 1px 4px;
  min-width: 16px;
  text-align: center;
}

/* ── Global options ──────────────────────────────────────────────────── */
.global-opts {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 8px 18px;
  background: var(--bg-surface-2);
  border-bottom: 1px solid var(--border-soft);
  flex-shrink: 0;
}
.global-opts .seg { width: auto; }
.opt-group { display: flex; align-items: center; gap: 8px; }
.opt-group > label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: .05em;
  font-weight: 600;
  white-space: nowrap;
}

/* ── Segmented controls ──────────────────────────────────────────────── */
.seg {
  display: flex;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  overflow: hidden;
  width: 100%;
}
.seg button {
  background: none;
  border: none;
  flex: 1;
  padding: 4px 10px;
  font-size: 12px;
  color: var(--text-subtle);
  cursor: pointer;
  transition: all .12s;
  white-space: nowrap;
}
.seg button + button { border-left: 1px solid var(--border); }
.seg button.sel {
  background: var(--accent);
  color: var(--on-accent);
  font-weight: 500;
}
.seg button:hover:not(.sel) { background: var(--bg-hover); color: var(--text-hover); }
.seg code {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 11px;
}

/* ── Main area ───────────────────────────────────────────────────────── */
.main { display: flex; flex: 1; overflow: hidden; }
.workspace { flex: 1; display: flex; flex-direction: column; overflow: hidden; }

/* ── Tabs ─────────────────────────────────────────────────────────────── */
.tabs {
  display: flex;
  padding: 0 18px;
  gap: 2px;
  border-bottom: 1px solid var(--border-soft);
  background: var(--bg-elevated);
  flex-shrink: 0;
}
.tabs button {
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  padding: 9px 14px;
  font-size: 12.5px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all .15s;
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: -1px;
}
.tabs button:hover { color: var(--text-hover); }
.tabs button.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
  font-weight: 500;
}
kbd {
  font-size: 10px;
  background: var(--bg-hover);
  border: 1px solid var(--border-strong);
  border-radius: 3px;
  padding: 1px 4px;
  color: var(--text-dim);
  font-family: inherit;
}

/* ── Panel / tab content ─────────────────────────────────────────────── */
.panel { flex: 1; overflow-y: auto; padding: 18px; }
.tab-content { display: flex; flex-direction: column; gap: 14px; }
.tab-content.two-col { flex-direction: row; gap: 18px; align-items: flex-start; }
.left { flex: 0 0 320px; display: flex; flex-direction: column; gap: 14px; }
.right { flex: 1; min-width: 0; }
.empty-right {
  color: var(--text-faint);
  font-style: italic;
  padding: 24px 0;
  text-align: center;
}

/* ── Fields ──────────────────────────────────────────────────────────── */
.field { display: flex; flex-direction: column; gap: 5px; }
.field > label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: .05em;
  font-weight: 600;
}
input[type=text], textarea {
  width: 100%;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 12px;
  padding: 8px 10px;
  outline: none;
  transition: border-color .15s;
  resize: vertical;
}
input[type=text]:focus, textarea:focus { border-color: var(--accent); }

/* ── Run button ──────────────────────────────────────────────────────── */
.run-btn {
  align-self: flex-start;
  background: var(--accent);
  border: none;
  border-radius: 7px;
  color: var(--on-accent);
  font-size: 13px;
  font-weight: 600;
  padding: 8px 18px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background .15s, transform .08s;
}
.run-btn:hover { background: var(--accent-hover); }
.run-btn:active { transform: scale(.97); }
.shortcut {
  font-size: 10px;
  opacity: .65;
  background: rgba(255,255,255,.15);
  border-radius: 4px;
  padding: 1px 5px;
}

/* ── Result block ────────────────────────────────────────────────────── */
.result-block {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 7px;
  overflow: hidden;
}
.result-block.full { height: 100%; }
.result-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 10px;
  border-bottom: 1px solid var(--border);
  font-size: 11px;
  color: var(--text-muted);
}
pre.result {
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 11.5px;
  line-height: 1.6;
  color: var(--result-text);
  padding: 12px;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 260px;
  overflow-y: auto;
}

/* ── Copy button ─────────────────────────────────────────────────────── */
.copy-btn {
  background: var(--copy-btn-bg);
  border: 1px solid var(--copy-btn-border);
  border-radius: 5px;
  color: var(--text-muted);
  font-size: 11px;
  padding: 3px 8px;
  cursor: pointer;
  transition: all .12s;
}
.copy-btn:hover { background: var(--copy-btn-border); color: var(--accent); }
.copy-btn.small { padding: 2px 6px; }

/* ── Error ───────────────────────────────────────────────────────────── */
.error {
  background: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 6px;
  color: var(--error-text);
  padding: 8px 12px;
  font-size: 12px;
}

/* ── History panel ───────────────────────────────────────────────────── */
.history-panel {
  width: 300px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--bg-surface-2);
  border-left: 1px solid var(--border-soft);
  overflow: hidden;
}
.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  border-bottom: 1px solid var(--border-soft);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-subtle);
}
.btn-clear {
  background: none;
  border: none;
  color: var(--text-dim);
  font-size: 11px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
}
.btn-clear:hover { color: var(--error-text); background: var(--error-bg); }
.history-empty {
  padding: 24px;
  text-align: center;
  color: var(--text-faint);
  font-style: italic;
  font-size: 12px;
}
.history-list { list-style: none; overflow-y: auto; flex: 1; }
.history-item {
  padding: 10px 14px;
  border-bottom: 1px solid var(--border-soft);
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.history-item:hover { background: var(--bg-elevated); }
.hi-meta { display: flex; align-items: center; gap: 6px; font-size: 10px; }
.hi-tab {
  background: var(--accent-soft);
  color: var(--accent);
  border-radius: 3px;
  padding: 1px 5px;
  font-weight: 600;
}
.hi-count { color: var(--text-muted); }
.hi-time { color: var(--text-faint); margin-left: auto; }
.hi-input {
  font-size: 10px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: 'JetBrains Mono', monospace;
}
.hi-output-row { display: flex; align-items: flex-start; gap: 6px; }
pre.hi-output {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 10px;
  color: var(--hi-output-text);
  white-space: pre-wrap;
  word-break: break-all;
  flex: 1;
  max-height: 60px;
  overflow: hidden;
  line-height: 1.5;
}

/* ── Footer ──────────────────────────────────────────────────────────── */
footer {
  display: flex;
  gap: 18px;
  padding: 6px 18px;
  background: var(--bg);
  border-top: 1px solid var(--border-soft);
  font-size: 11.5px;
  color: var(--text-faint);
  flex-shrink: 0;
}
footer span::before { content: ''; }</style>
