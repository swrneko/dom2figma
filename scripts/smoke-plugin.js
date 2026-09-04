// Прогоняет плагин на заглушке Figma API без самой Figma: node scripts/smoke-plugin.js out/screens.json
'use strict';
const fs = require('fs');
const path = require('path');
const vm = require('vm');

const file = process.argv[2] || path.join(__dirname, '..', 'out', 'screens.json');
const data = JSON.parse(fs.readFileSync(file, 'utf8'));

const stats = { frames: 0, texts: 0, svgs: 0, pages: 0, images: 0, errors: [] };
function node(type) {
  const n = { type, children: [], x: 0, y: 0, width: 10, height: 10, appendChild(c) { this.children.push(c); c.parent = this; }, resize(w, h) { if (!(w > 0 && h > 0)) throw new Error(`resize ${w}x${h}`); this.width = w; this.height = h; }, resizeWithoutConstraints(w, h) { this.resize(w, h); } };
  return new Proxy(n, {
    set(t, k, v) {
      if (k === 'characters' && typeof v !== 'string') throw new Error('characters must be string');
      if (k === 'fills' && !Array.isArray(v)) throw new Error('fills must be array');
      if (k === 'fontName' && !(v && v.family && v.style)) throw new Error('bad fontName');
      if (k === 'x' || k === 'y') { if (typeof v !== 'number' || Number.isNaN(v)) throw new Error(`${k} NaN`); }
      t[k] = v; return true;
    },
  });
}
const figma = {
  showUI() {}, skipInvisibleInstanceChildren: false,
  ui: { postMessage(m) { if (m.type === 'log') console.log('  [ui]', m.text); }, onmessage: null },
  notify(m) { console.log('  [notify]', m); },
  createPage() { stats.pages++; const p = node('PAGE'); return p; },
  createFrame() { stats.frames++; return node('FRAME'); },
  createSection() { stats.sections = (stats.sections || 0) + 1; return node('SECTION'); },
  currentPage: null,
  createText() { stats.texts++; return node('TEXT'); },
  createNodeFromSvg(s) { if (!/^<svg/.test(s)) throw new Error('bad svg'); stats.svgs++; return node('FRAME'); },
  createImage(bytes) { if (!(bytes instanceof Uint8Array) || !bytes.length) throw new Error('bad image bytes'); stats.images++; return { hash: 'h' + stats.images }; },
  async loadFontAsync(f) { if (f.family !== 'Inter') throw new Error('no font ' + f.family); if (!/^(Thin|Extra Light|Light|Regular|Medium|Semi Bold|Bold|Extra Bold|Black)( Italic)?$|^Italic$/.test(f.style)) throw new Error('no style ' + f.style); },
};
figma.currentPage = node('PAGE');
const ctx = { figma, __html__: '', setTimeout, console };
vm.createContext(ctx);
vm.runInContext(fs.readFileSync(path.join(__dirname, '..', 'assets', 'figma-plugin', 'code.js'), 'utf8'), ctx);

(async () => {
  const t0 = Date.now();
  await figma.ui.onmessage({ type: 'import', data, skipDup: true, collapse: true });
  console.log(`\nОК: sections=${stats.sections || 0} pages=${stats.pages} frames=${stats.frames} texts=${stats.texts} svgs=${stats.svgs} images=${stats.images} за ${Date.now() - t0} мс`);
})().catch((e) => { console.error('FAIL', e); process.exit(1); });
