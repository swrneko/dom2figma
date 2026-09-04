// dom2figma: строит секции и фреймы Figma из JSON, снятого командой `dom2figma capture`.
// Блок между маркерами LIB используется также для варианта под Scripter (`dom2figma scripter`).
figma.showUI(__html__, { width: 420, height: 380 });
function log(text) { figma.ui.postMessage({ type: 'log', text }); }

// --- LIB START ---
figma.skipInvisibleInstanceChildren = true;

let FONT_FAMILY = 'Inter';
const WEIGHT_STYLE = { 100: 'Thin', 200: 'Extra Light', 300: 'Light', 400: 'Regular', 500: 'Medium', 600: 'Semi Bold', 700: 'Bold', 800: 'Extra Bold', 900: 'Black' };
const fontCache = new Map();

const tick = () => new Promise((r) => setTimeout(r, 0));

async function font(weight, italic) {
  const w = Math.min(900, Math.max(100, Math.round((weight || 400) / 100) * 100));
  let style = WEIGHT_STYLE[w] || 'Regular';
  if (italic) style = style === 'Regular' ? 'Italic' : style + ' Italic';
  const key = style;
  if (fontCache.has(key)) return fontCache.get(key);
  let name = { family: FONT_FAMILY, style };
  try { await figma.loadFontAsync(name); }
  catch (e) {
    name = { family: FONT_FAMILY, style: 'Regular' };
    await figma.loadFontAsync(name);
  }
  fontCache.set(key, name);
  return name;
}

function solid(c) { return { type: 'SOLID', color: { r: c.r, g: c.g, b: c.b }, opacity: c.a == null ? 1 : c.a }; }

function gradientPaint(g) {
  const th = ((parseFloat(g.angle) || 180) * Math.PI) / 180;
  const dx = Math.sin(th), dy = -Math.cos(th);
  const len = Math.abs(dx) + Math.abs(dy);
  const sx = 0.5 - (dx * len) / 2, sy = 0.5 - (dy * len) / 2;
  const vx = dx * len, vy = dy * len;
  const det = vx * vx + vy * vy || 1;
  const gradientTransform = [
    [vx / det, vy / det, -(vx * sx + vy * sy) / det],
    [-vy / det, vx / det, (vy * sx - vx * sy) / det],
  ];
  const n = g.stops.length;
  return {
    type: 'GRADIENT_LINEAR',
    gradientTransform,
    gradientStops: g.stops.map((c, i) => ({ position: n === 1 ? 0 : i / (n - 1), color: { r: c.r, g: c.g, b: c.b, a: c.a == null ? 1 : c.a } })),
  };
}

function base64ToBytes(b64) {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
  const clean = b64.replace(/[^A-Za-z0-9+/]/g, '');
  const out = new Uint8Array(Math.floor((clean.length * 3) / 4));
  let buf = 0, bits = 0, k = 0;
  for (const ch of clean) {
    buf = (buf << 6) | chars.indexOf(ch); bits += 6;
    if (bits >= 8) { bits -= 8; out[k++] = (buf >> bits) & 0xff; }
  }
  return out.slice(0, k);
}

function imagePaint(src) {
  const m = /^data:image\/[a-z+]+;base64,(.*)$/i.exec(src || '');
  if (!m) return null;
  try { const img = figma.createImage(base64ToBytes(m[1])); return { type: 'IMAGE', scaleMode: 'FILL', imageHash: img.hash }; }
  catch (e) { return null; }
}

function isPlainWrapper(n) {
  return n.type === 'frame' && !(n.fills && n.fills.length) && !n.stroke && !n.gradient && !n.bgImage && !(n.shadows && n.shadows.length) && !n.clip && (n.opacity == null || n.opacity >= 0.999) && n.type !== 'image';
}

function applyFrameStyle(f, n) {
  const fills = [];
  if (n.fills) for (const c of n.fills) fills.push(solid(c));
  if (n.gradient) fills.push(gradientPaint(n.gradient));
  if (n.bgImage) { const p = imagePaint(n.bgImage); if (p) fills.push(p); }
  if (n.type === 'image' && n.src) { const p = imagePaint(n.src); if (p) fills.push(p); }
  f.fills = fills;
  if (n.stroke) {
    f.strokes = [solid(n.stroke.color)];
    f.strokeAlign = 'INSIDE';
    f.strokeTopWeight = n.stroke.top; f.strokeRightWeight = n.stroke.right;
    f.strokeBottomWeight = n.stroke.bottom; f.strokeLeftWeight = n.stroke.left;
    if (n.stroke.dashed) f.dashPattern = [4, 4];
  }
  if (n.radius) {
    const [tl, tr, br, bl] = n.radius;
    f.topLeftRadius = tl; f.topRightRadius = tr; f.bottomRightRadius = br; f.bottomLeftRadius = bl;
  }
  f.clipsContent = !!n.clip;
  if (n.opacity != null && n.opacity < 1) f.opacity = n.opacity;
  if (n.shadows && n.shadows.length) {
    f.effects = n.shadows.map((s) => ({
      type: s.inset ? 'INNER_SHADOW' : 'DROP_SHADOW',
      color: { r: s.color.r, g: s.color.g, b: s.color.b, a: s.color.a == null ? 1 : s.color.a },
      offset: { x: s.x, y: s.y }, radius: s.blur, spread: s.spread || 0, visible: true, blendMode: 'NORMAL',
    }));
  }
}

async function buildText(n, parent, ox, oy) {
  const t = figma.createText();
  const fn = await font(n.fontWeight, n.italic);
  t.fontName = fn;
  t.characters = n.text || ' ';
  t.fontSize = Math.max(1, n.fontSize || 14);
  t.lineHeight = n.lineHeight ? { value: n.lineHeight, unit: 'PIXELS' } : { unit: 'AUTO' };
  if (n.letterSpacing) t.letterSpacing = { value: n.letterSpacing, unit: 'PIXELS' };
  t.fills = [solid(n.color || { r: 0, g: 0, b: 0, a: 1 })];
  t.textAlignHorizontal = n.align === 'center' ? 'CENTER' : n.align === 'right' ? 'RIGHT' : 'LEFT';
  if (n.transform === 'uppercase') t.textCase = 'UPPER';
  else if (n.transform === 'lowercase') t.textCase = 'LOWER';
  else if (n.transform === 'capitalize') t.textCase = 'TITLE';
  if (/underline/.test(n.decoration || '')) t.textDecoration = 'UNDERLINE';
  else if (/line-through/.test(n.decoration || '')) t.textDecoration = 'STRIKETHROUGH';
  t.name = n.name || (n.text || 'text').slice(0, 40);
  const lineH = n.lineHeight || (n.fontSize || 14) * 1.25;
  const singleLine = n.h < lineH * 1.6;
  parent.appendChild(t);
  if (singleLine) {
    // Одна строка: автоширина, чтобы текст не переносился из-за разницы метрик, позиция по выравниванию.
    t.textAutoResize = 'WIDTH_AND_HEIGHT';
    const dx = n.align === 'center' ? (n.w - t.width) / 2 : n.align === 'right' ? n.w - t.width : 0;
    t.x = n.x - ox + dx; t.y = n.y - oy;
  } else {
    t.textAutoResize = 'HEIGHT';
    t.resize(Math.max(1, n.w + 2), Math.max(1, n.h));
    t.x = n.x - ox; t.y = n.y - oy;
  }
  return t;
}

function buildSvg(n, parent, ox, oy) {
  let node;
  try { node = figma.createNodeFromSvg(n.svg); }
  catch (e) { node = figma.createFrame(); node.fills = []; }
  node.name = n.name || 'icon';
  if (n.w > 0 && n.h > 0) node.resize(n.w, n.h);
  node.x = n.x - ox; node.y = n.y - oy;
  parent.appendChild(node);
  return node;
}

async function build(n, parent, ox, oy, opts, counter) {
  counter.n++;
  if (counter.n % 400 === 0) await tick();
  if (n.type === 'text') return buildText(n, parent, ox, oy);
  if (n.type === 'svg') return buildSvg(n, parent, ox, oy);

  if (opts.collapse && isPlainWrapper(n) && n.children && n.children.length === 1) {
    return build(n.children[0], parent, ox, oy, opts, counter);
  }

  const f = figma.createFrame();
  f.name = n.name || 'frame';
  f.resize(Math.max(0.01, n.w), Math.max(0.01, n.h));
  applyFrameStyle(f, n);
  f.x = n.x - ox; f.y = n.y - oy;
  parent.appendChild(f);
  for (const c of n.children || []) await build(c, f, n.x, n.y, opts, counter);
  return f;
}

function makeContainer(name, opts) {
  if (opts.separatePages) { const page = figma.createPage(); page.name = name; return { node: page, isPage: true }; }
  let sec = null;
  try { sec = figma.createSection(); } catch (e) { sec = null; }
  if (!sec) { sec = figma.createFrame(); sec.fills = []; sec.clipsContent = false; }
  sec.name = name;
  figma.currentPage.appendChild(sec);
  return { node: sec, isPage: false };
}

async function importAll(data, opts) {
  const COLS = 6, GAP_X = 120, GAP_Y = 160, PAD = 80, SECTION_GAP = 400;
  if (data.font) FONT_FAMILY = data.font;
  fontCache.clear();
  let total = 0;
  // Секции ставим ниже уже существующего содержимого страницы.
  let sectionY = figma.currentPage.children.reduce((m, c) => Math.max(m, c.y + c.height), 0);
  if (sectionY > 0) sectionY += SECTION_GAP;
  const groups = data.groups || data.personas || [];
  for (const persona of groups) {
    const { node: page, isPage } = makeContainer(persona.name, opts);
    const screens = persona.screens.filter((s) => !(opts.skipDup && s.sameAs));
    log(`${persona.name}: ${screens.length} экранов`);
    let x = PAD, y = PAD, rowH = 0, col = 0, maxX = 0, maxY = 0;
    for (const s of screens) {
      const label = figma.createText();
      await figma.loadFontAsync({ family: FONT_FAMILY, style: 'Semi Bold' });
      label.fontName = { family: FONT_FAMILY, style: 'Semi Bold' };
      label.fontSize = 24;
      label.characters = s.route + (s.redirected ? `   (запрошено ${s.requested})` : '');
      label.x = x; label.y = y;
      page.appendChild(label);

      const root = figma.createFrame();
      root.name = s.route;
      root.resize(s.w, s.h);
      root.fills = [{ type: 'SOLID', color: { r: 1, g: 1, b: 1 } }];
      root.clipsContent = true;
      root.x = x; root.y = y + 48;
      page.appendChild(root);
      const counter = { n: 0 };
      for (const c of s.tree.children || []) await build(c, root, s.tree.x, s.tree.y, opts, counter);
      total++;
      rowH = Math.max(rowH, s.h + 48);
      maxX = Math.max(maxX, x + s.w); maxY = Math.max(maxY, y + rowH);
      col++;
      if (col >= COLS) { col = 0; x = PAD; y += rowH + GAP_Y; rowH = 0; } else { x += s.w + GAP_X; }
      await tick();
    }
    if (!isPage) {
      page.resizeWithoutConstraints(maxX + PAD, maxY + PAD);
      page.x = 0; page.y = sectionY;
      sectionY += maxY + PAD + SECTION_GAP;
    }
  }
  return total;
}

// --- LIB END ---

figma.ui.onmessage = async (msg) => {
  if (msg.type !== 'import') return;
  try {
    const n = await importAll(msg.data, { skipDup: msg.skipDup, collapse: msg.collapse, separatePages: !!msg.separatePages });
    log(`Импортировано экранов: ${n}`);
    figma.notify(`Импортировано экранов: ${n}`);
  } catch (e) {
    log('Ошибка: ' + (e && e.message ? e.message : String(e)));
    figma.notify('Ошибка импорта, см. лог плагина', { error: true });
  }
  figma.ui.postMessage({ type: 'done' });
};
