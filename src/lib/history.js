import { writable } from 'svelte/store';
 
export const history = writable([]);
 
let _id = 0;
 
export function addHistory(entry) {
  history.update((h) => {
    // dédoublonnage : même tab + même input + même output → on remonte l'existant
    const existing = h.findIndex(
      (e) => e.tab === entry.tab && e.input === entry.input && e.output === entry.output
    );
    if (existing !== -1) {
      // remettre en tête avec timestamp mis à jour
      const updated = { ...h[existing], ts: new Date() };
      return [updated, ...h.filter((_, i) => i !== existing)];
    }
    return [{ ...entry, id: ++_id, ts: new Date() }, ...h].slice(0, 50);
  });
}
 
export function clearHistory() {
  history.set([]);
}
