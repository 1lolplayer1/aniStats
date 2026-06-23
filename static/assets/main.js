const INTERVAL = 30000;

function statusClass(status) {
  if (!status) return 'unknown';
  return status.toLowerCase();
}

function latencyClass(ms) {
  if (ms == null) return '';
  if (ms < 200) return 'fast';
  if (ms < 600) return 'med';
  return 'slow';
}

function formatTime(iso) {
  if (!iso) return '—';
  return new Date(iso).toTimeString().slice(0, 8);
}

function renderSparkline(history) {
  if (!history || history.length === 0) {
    return '<div class="sparkline"><span style="color:#222;font-size:11px">no data</span></div>';
  }
  const bars = history.slice(-12).map(point => {
    const s      = statusClass(point.status);
    const color  = s === 'up' ? '#4ade80' : s === 'timeout' ? '#fbbf24' : s === 'down' ? '#f87171' : '#2a2a2a';
    const height = s === 'up' ? 14 : s === 'timeout' ? 10 : s === 'down' ? 8 : 5;
    return `<div class="spark-bar" style="height:${height}px;background:${color}"></div>`;
  }).join('');
  return `<div class="sparkline">${bars}</div>`;
}

function renderRow(name, site) {
  const sc   = statusClass(site.status);
  const lat  = site.latency_ms != null ? `${site.latency_ms}ms` : '—';
  const code = site.http_code  != null ? site.http_code : '—';
  const host = (() => { try { return new URL(site.url).hostname; } catch { return site.url; } })();

  return `
    <tr class="${sc}">
      <td>
        <div class="site-name">${site.name}</div>
        <div class="site-url">${host}</div>
      </td>
      <td>
        <span class="status-dot ${sc}"></span>
        <span class="status-label ${sc}">${sc}</span>
      </td>
      <td><span class="latency ${latencyClass(site.latency_ms)}">${lat}</span></td>
      <td><span class="code">${code}</span></td>
      <td>${renderSparkline(site.history)}</td>
      <td><span class="timestamp">${formatTime(site.last_checked)}</span></td>
    </tr>`;
}

function animateRefreshBar() {
  const fill = document.getElementById('rfill');
  fill.style.transition = 'none';
  fill.style.width = '0%';
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      fill.style.transition = `width ${INTERVAL}ms linear`;
      fill.style.width = '100%';
    });
  });
}

async function fetchAndRender() {
  const errorEl = document.getElementById('error-msg');
  try {
    const res  = await fetch('/api/status');
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();

    errorEl.style.display = 'none';

    const order = { up: 0, timeout: 1, down: 2, unknown: 3 };
    const sites = Object.entries(data).sort(([, a], [, b]) =>
      (order[statusClass(a.status)] ?? 3) - (order[statusClass(b.status)] ?? 3)
    );

    let up = 0, down = 0, unknown = 0;
    sites.forEach(([, s]) => {
      const sc = statusClass(s.status);
      if (sc === 'up') up++;
      else if (sc === 'down') down++;
      else unknown++;
    });

    document.getElementById('count-up').textContent      = up;
    document.getElementById('count-down').textContent    = down;
    document.getElementById('count-unknown').textContent = unknown;
    document.getElementById('last-updated').textContent  = new Date().toTimeString().slice(0, 8);

    const half = Math.ceil(sites.length / 2);
    document.getElementById('table-body').innerHTML   = sites.slice(0, half).map(([n, s]) => renderRow(n, s)).join('');
    document.getElementById('table-body-2').innerHTML = sites.slice(half).map(([n, s]) => renderRow(n, s)).join('');

  } catch (err) {
    errorEl.style.display = 'block';
    console.error('fetch failed:', err);
  }

  animateRefreshBar();
}

fetchAndRender();
setInterval(fetchAndRender, INTERVAL);
