// Внедряется в страницу CLI dom2figma. Превращает отрисованный DOM в дерево узлов для Figma-плагина.
// Идемпотентно: повторное внедрение после навигации ничего не ломает.
(function () {
  if (window.__dom2figma) return;

  const r2 = (v) => Math.round(v * 2) / 2;

  function parseColor(c) {
    if (!c) return null;
    const m = c.match(/rgba?\(([^)]+)\)/);
    if (!m) return null;
    const p = m[1].split(/[,/\s]+/).filter(Boolean).map(parseFloat);
    const a = p.length > 3 ? p[3] : 1;
    if (a <= 0) return null;
    return { r: p[0] / 255, g: p[1] / 255, b: p[2] / 255, a };
  }
  function px(v, base) {
    if (!v || v === 'normal' || v === 'none') return null;
    if (v.endsWith('%')) return (parseFloat(v) / 100) * (base || 0);
    const n = parseFloat(v);
    return isNaN(n) ? null : n;
  }
  function parseShadow(s) {
    if (!s || s === 'none') return [];
    const out = [];
    const parts = s.split(/\)\s*,\s*(?=rgb)/).map((p, i, arr) => (i < arr.length - 1 ? p + ')' : p));
    for (const p of parts) {
      const cm = p.match(/rgba?\([^)]+\)/);
      const nums = p.replace(/rgba?\([^)]+\)/, '').trim().split(/\s+/).map(parseFloat).filter((n) => !isNaN(n));
      const color = parseColor(cm ? cm[0] : '');
      if (!color || nums.length < 2) continue;
      out.push({ color, x: nums[0], y: nums[1], blur: nums[2] || 0, spread: nums[3] || 0, inset: /inset/.test(p) });
    }
    return out;
  }
  function textOf(el) {
    if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
      if (el.type === 'checkbox' || el.type === 'radio' || el.type === 'range' || el.type === 'file') return null;
      return { text: el.value || el.placeholder || '', placeholder: !el.value };
    }
    if (el.tagName === 'SELECT') {
      const o = el.options[el.selectedIndex];
      return { text: o ? o.textContent : '', placeholder: false };
    }
    return null;
  }

  function serialize(rootSelector) {
    const root = document.querySelector(rootSelector);
    if (!root) return { error: 'root not found: ' + rootSelector };
    const rootRect = root.getBoundingClientRect();

    function textNode(cs, box, content, align, placeholder) {
      const color = parseColor(cs.color);
      if (!color) return null; // color: transparent — текст скрыт намеренно
      if (placeholder) color.a = Math.min(color.a, 0.5);
      return {
        type: 'text',
        name: content.trim().slice(0, 40),
        x: r2(box.x - rootRect.left),
        y: r2(box.y - rootRect.top),
        w: r2(box.w),
        h: r2(box.h),
        text: content,
        fontFamily: cs.fontFamily,
        fontSize: parseFloat(cs.fontSize),
        fontWeight: parseInt(cs.fontWeight, 10) || 400,
        italic: cs.fontStyle === 'italic',
        lineHeight: px(cs.lineHeight),
        letterSpacing: px(cs.letterSpacing) || 0,
        color,
        align: align || 'left',
        transform: cs.textTransform,
        decoration: cs.textDecorationLine,
        opacity: 1,
      };
    }
    function contentBox(el, r, cs) {
      const pl = parseFloat(cs.paddingLeft) + parseFloat(cs.borderLeftWidth);
      const pr = parseFloat(cs.paddingRight) + parseFloat(cs.borderRightWidth);
      return { x: r.left + pl, w: Math.max(0, r.width - pl - pr) };
    }
    const ALIGN = { start: 'left', left: 'left', right: 'right', end: 'right', center: 'center', justify: 'left' };

    function walk(el, depth) {
      if (!(el instanceof Element)) return null;
      const cs = getComputedStyle(el);
      if (cs.display === 'none' || cs.visibility === 'hidden') return null;
      const r = el.getBoundingClientRect();
      if (r.width <= 0 && r.height <= 0 && !(el instanceof SVGElement)) return null;
      const opacity = parseFloat(cs.opacity);
      if (opacity === 0) return null;

      const cls = typeof el.className === 'string' ? el.className.trim() : '';
      const node = {
        type: 'frame',
        name: cls ? cls.split(/\s+/).join(' ') : el.tagName.toLowerCase(),
        x: r2(r.left - rootRect.left),
        y: r2(r.top - rootRect.top),
        w: r2(r.width),
        h: r2(r.height),
        opacity,
        children: [],
      };

      if (el instanceof SVGSVGElement) {
        let svg = el.outerHTML.replace(/currentColor/g, cs.color);
        if (!/\swidth=/.test(svg)) svg = svg.replace('<svg', `<svg width="${r.width}" height="${r.height}"`);
        node.type = 'svg';
        node.svg = svg;
        return node;
      }
      if (el instanceof SVGElement) return null;

      if (el.tagName === 'IMG' && el.currentSrc) {
        node.type = 'image';
        node.src = el.currentSrc;
      }

      const bg = parseColor(cs.backgroundColor);
      node.fills = bg ? [bg] : [];
      const bgi = cs.backgroundImage;
      if (bgi && bgi !== 'none') {
        const colors = bgi.match(/rgba?\([^)]+\)/g);
        if (/linear-gradient\(/.test(bgi) && colors && colors.length >= 2) {
          node.gradient = { angle: (bgi.match(/(-?\d+(?:\.\d+)?)deg/) || [0, 180])[1], stops: colors.map(parseColor).filter(Boolean) };
        }
        const u = bgi.match(/url\("?([^")]+)"?\)/);
        if (u) node.bgImage = u[1];
      }

      const bw = ['Top', 'Right', 'Bottom', 'Left'].map((s) => parseFloat(cs['border' + s + 'Width']) || 0);
      const bc = parseColor(cs.borderTopColor) || parseColor(cs.borderBottomColor) || parseColor(cs.borderLeftColor);
      if (bc && bw.some((w) => w > 0) && cs.borderTopStyle !== 'none') {
        node.stroke = { color: bc, top: bw[0], right: bw[1], bottom: bw[2], left: bw[3], dashed: /dashed|dotted/.test(cs.borderTopStyle) };
      }
      const minSide = Math.min(r.width, r.height);
      node.radius = [cs.borderTopLeftRadius, cs.borderTopRightRadius, cs.borderBottomRightRadius, cs.borderBottomLeftRadius].map((v) =>
        Math.min(px(v.split(' ')[0], minSide) || 0, minSide / 2),
      );
      node.clip = /hidden|auto|scroll|clip/.test(cs.overflowX + cs.overflowY);
      node.shadows = parseShadow(cs.boxShadow);

      // Псевдоэлементы с текстом.
      for (const pseudo of ['::before', '::after']) {
        const pcs = getComputedStyle(el, pseudo);
        const c = pcs.content;
        if (c && c !== 'none' && c !== 'normal' && /^"/.test(c) && pcs.display !== 'none') {
          const str = c.replace(/^"|"$/g, '').replace(/\\"/g, '"');
          if (str.trim()) {
            const cb = contentBox(el, r, cs);
            const fsz = parseFloat(pcs.fontSize);
            const box = { x: cb.x, y: r.top + parseFloat(cs.paddingTop), w: Math.max(fsz, cb.w), h: px(pcs.lineHeight) || fsz * 1.3 };
            const tn = textNode(pcs, box, str, pseudo === '::after' ? 'right' : 'left', false);
            if (tn) { if (pseudo === '::before') node.children.push(tn); else node._after = tn; }
          }
        }
      }

      // Текст полей форм.
      const formText = textOf(el);
      if (formText && formText.text) {
        const cb = contentBox(el, r, cs);
        const lh = px(cs.lineHeight) || parseFloat(cs.fontSize) * 1.3;
        const pt = parseFloat(cs.paddingTop) + parseFloat(cs.borderTopWidth);
        const pb = parseFloat(cs.paddingBottom) + parseFloat(cs.borderBottomWidth);
        const inner = Math.max(lh, r.height - pt - pb);
        const box = { x: cb.x, y: r.top + pt + (inner - lh) / 2, w: cb.w, h: lh };
        const tn = textNode(cs, box, formText.text, ALIGN[cs.textAlign] || 'left', formText.placeholder);
        if (tn) node.children.push(tn);
      }

      const elementKids = [];
      const textKids = [];
      for (const ch of el.childNodes) {
        if (ch.nodeType === 3) { if (ch.textContent.trim()) textKids.push(ch); }
        else if (ch.nodeType === 1) elementKids.push(ch);
      }

      if (textKids.length) {
        const align = ALIGN[cs.textAlign] || 'left';
        if (textKids.length === 1 && elementKids.length === 0) {
          // Единственный текст: контентный бокс элемента сохраняет выравнивание и переносы.
          const range = document.createRange();
          range.selectNodeContents(textKids[0]);
          const rr = range.getBoundingClientRect();
          const cb = contentBox(el, r, cs);
          if (rr.height > 0) {
            const tn = textNode(cs, { x: cb.x, y: rr.top, w: cb.w, h: rr.height }, textKids[0].textContent.replace(/\s+/g, ' ').trim(), align, false);
            if (tn) node.children.push(tn);
          }
        } else {
          for (const tk of textKids) {
            const range = document.createRange();
            range.selectNodeContents(tk);
            const rects = [...range.getClientRects()];
            if (!rects.length) continue;
            const x = Math.min(...rects.map((q) => q.left));
            const y = Math.min(...rects.map((q) => q.top));
            const x2 = Math.max(...rects.map((q) => q.right));
            const y2 = Math.max(...rects.map((q) => q.bottom));
            const tn = textNode(cs, { x, y, w: x2 - x + 2, h: y2 - y }, tk.textContent.replace(/\s+/g, ' ').trim(), rects.length > 1 ? align : 'left', false);
            if (tn) node.children.push(tn);
          }
        }
      }

      for (const ch of elementKids) {
        const k = walk(ch, depth + 1);
        if (k) node.children.push(k);
      }
      if (node._after) { node.children.push(node._after); delete node._after; }

      const hasVisual = node.fills.length || node.stroke || node.gradient || node.bgImage || node.shadows.length || node.type === 'image';
      if (!hasVisual && !node.children.length && depth > 0) return null;
      return node;
    }

    const tree = walk(root, 0);
    return { w: r2(rootRect.width), h: r2(rootRect.height), tree };
  }

  // Раскрывает внутренний скролл: корень растягивается до высоты содержимого контейнера.
  function fit(rootSelector, scrollSelector, minHeight) {
    const root = document.querySelector(rootSelector);
    if (!root) return false;
    const base = minHeight > 0 ? minHeight : root.getBoundingClientRect().height;
    root.style.height = base + 'px';
    const s = scrollSelector ? document.querySelector(scrollSelector) : null;
    if (!s) return true;
    for (let i = 0; i < 3; i++) {
      const extra = s.scrollHeight - s.clientHeight;
      if (extra <= 1) break;
      root.style.height = root.getBoundingClientRect().height + extra + 'px';
    }
    return true;
  }

  function setCss(id, css) {
    let st = document.getElementById(id);
    if (!css) { if (st) st.remove(); return; }
    if (!st) { st = document.createElement('style'); st.id = id; document.head.appendChild(st); }
    st.textContent = css;
  }

  function settle(ms) {
    return new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(r, ms))));
  }

  function docHeight() {
    return Math.max(document.documentElement.scrollHeight, document.body ? document.body.scrollHeight : 0);
  }

  window.__dom2figma = { serialize, fit, setCss, settle, docHeight, version: 1 };
})();
