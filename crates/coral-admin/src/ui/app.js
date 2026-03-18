const state = {
  page: "members",
  detail: null,
  data: null,
  offset: 0,
  limit: 50,
  search: "",
  tagFilter: "",
};

const tagColors = {
  sniper: "sniper",
  blatant_cheater: "blatant-cheater",
  closet_cheater: "closet-cheater",
  replays_needed: "replays-needed",
  confirmed_cheater: "confirmed-cheater",
};

async function api(path) {
  const res = await fetch(`/api${path}`);
  return res.json();
}

function formatDate(iso) {
  if (!iso) return "-";
  return new Date(iso).toLocaleString();
}

function formatDiscordId(id) {
  return `<span class="mono">${id}</span>`;
}

function formatUuid(uuid) {
  if (!uuid) return "-";
  return `<span class="mono">${uuid}</span>`;
}

function renderBadges(member) {
  const badges = [];
  if (member.is_admin)
    badges.push('<span class="badge badge-admin">Admin</span>');
  if (member.is_mod) badges.push('<span class="badge badge-mod">Mod</span>');
  if (member.is_private)
    badges.push('<span class="badge badge-private">Private</span>');
  if (member.is_beta) badges.push('<span class="badge badge-beta">Beta</span>');
  if (member.key_locked)
    badges.push('<span class="badge badge-locked">Locked</span>');
  return badges.join(" ") || '<span class="text-muted">-</span>';
}

function renderTagBadge(tagType) {
  const colorClass = tagColors[tagType] || "info";
  return `<span class="badge badge-${colorClass}">${tagType}</span>`;
}

function navigate(page, detail = null) {
  state.page = page;
  state.detail = detail;
  state.offset = 0;
  state.search = "";
  state.tagFilter = "";
  render();
  loadData();
}

function setOffset(offset) {
  state.offset = Math.max(0, offset);
  loadData();
}

function setSearch(value) {
  state.search = value;
  state.offset = 0;
}

function setTagFilter(value) {
  state.tagFilter = value;
  state.offset = 0;
}

async function loadData() {
  const main = document.getElementById("main");
  main.innerHTML = '<div class="loading">Loading...</div>';

  try {
    if (state.detail) {
      await loadDetail();
    } else {
      await loadList();
    }
  } catch (e) {
    main.innerHTML = `<div class="empty">Error: ${e.message}</div>`;
  }
}

async function loadList() {
  if (state.page === "diagnostics") {
    state.data = await api("/diagnostics");
    renderList();
    return;
  }

  const params = new URLSearchParams();
  params.set("limit", state.limit);
  params.set("offset", state.offset);
  if (state.search) params.set("search", state.search);
  if (state.page === "snapshots" && state.search) {
    if (state.search.length === 32) {
      params.set("uuid", state.search);
      params.delete("search");
    } else {
      params.set("username", state.search);
      params.delete("search");
    }
  }
  if (state.tagFilter) params.set("tag_type", state.tagFilter);

  const endpoint =
    state.page === "rate-limits" ? "/rate-limits" : `/${state.page}`;
  state.data = await api(`${endpoint}?${params}`);
  renderList();
}

async function loadDetail() {
  state.data = await api(`/${state.page}/${state.detail}`);
  renderDetail();
}

function renderList() {
  const main = document.getElementById("main");

  switch (state.page) {
    case "members":
      renderMembersList(main);
      break;
    case "blacklist":
      renderBlacklistList(main);
      break;
    case "snapshots":
      renderSnapshotsList(main);
      break;
    case "rate-limits":
      renderRateLimitsList(main);
      break;
    case "diagnostics":
      renderDiagnostics(main);
      break;
  }
}

function renderMembersList(main) {
  const { total, members } = state.data;

  main.innerHTML = `
        <div class="header">
            <h2>Members</h2>
            <div class="controls">
                <input type="text" id="search" placeholder="Search Discord ID or UUID..." value="${state.search}">
                <button onclick="doSearch()">Search</button>
            </div>
        </div>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Discord ID</th>
                    <th>UUID</th>
                    <th>Access</th>
                    <th>Requests</th>
                    <th>Joined</th>
                </tr>
            </thead>
            <tbody>
                ${members
                  .map(
                    (m) => `
                    <tr class="clickable" onclick="navigate('members', ${m.id})">
                        <td>${m.id}</td>
                        <td>${formatDiscordId(m.discord_id)}</td>
                        <td>${formatUuid(m.uuid)}</td>
                        <td>${renderBadges(m)}</td>
                        <td>${m.request_count.toLocaleString()}</td>
                        <td>${formatDate(m.join_date)}</td>
                    </tr>
                `,
                  )
                  .join("")}
            </tbody>
        </table>
        ${renderPagination(total)}
    `;

  document.getElementById("search").addEventListener("keypress", (e) => {
    if (e.key === "Enter") doSearch();
  });
}

function renderBlacklistList(main) {
  const { total, players } = state.data;
  const tagTypes = [
    "Sniper",
    "BlatantCheater",
    "ClosetCheater",
    "ReplaysNeeded",
    "ConfirmedCheater",
  ];

  main.innerHTML = `
        <div class="header">
            <h2>Blacklist</h2>
            <div class="controls">
                <input type="text" id="search" placeholder="Search UUID..." value="${state.search}">
                <select id="tagFilter" onchange="setTagFilter(this.value); loadData()">
                    <option value="">All Tags</option>
                    ${tagTypes.map((t) => `<option value="${t}" ${state.tagFilter === t ? "selected" : ""}>${t}</option>`).join("")}
                </select>
                <button onclick="doSearch()">Search</button>
            </div>
        </div>
        <table>
            <thead>
                <tr>
                    <th>UUID</th>
                    <th>Tags</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                ${players
                  .map(
                    (p) => `
                    <tr class="clickable" onclick="navigate('blacklist', '${p.uuid}')">
                        <td>${formatUuid(p.uuid)}</td>
                        <td>${p.tags.map((t) => renderTagBadge(t.tag_type)).join(" ") || '<span class="text-muted">-</span>'}</td>
                        <td>${p.is_locked ? '<span class="badge badge-locked">Locked</span>' : ""}</td>
                    </tr>
                `,
                  )
                  .join("")}
            </tbody>
        </table>
        ${renderPagination(total)}
    `;

  document.getElementById("search").addEventListener("keypress", (e) => {
    if (e.key === "Enter") doSearch();
  });
}

function renderSnapshotsList(main) {
  const { total, snapshots } = state.data;

  main.innerHTML = `
        <div class="header">
            <h2>Snapshots</h2>
            <div class="controls">
                <input type="text" id="search" placeholder="Search UUID or username..." value="${state.search}">
                <button onclick="doSearch()">Search</button>
            </div>
        </div>
        <table>
            <thead>
                <tr>
                    <th>Type</th>
                    <th>UUID</th>
                    <th>Username</th>
                    <th>Source</th>
                    <th>Timestamp</th>
                </tr>
            </thead>
            <tbody>
                ${snapshots
                  .map(
                    (s) => `
                    <tr class="clickable" onclick="navigate('snapshots', ${s.id})">
                        <td><span class="baseline-indicator ${s.is_baseline ? "is-baseline" : "is-delta"}"></span>${s.is_baseline ? "Baseline" : "Delta"}</td>
                        <td>${formatUuid(s.uuid)}</td>
                        <td>${s.username || '<span class="text-muted">-</span>'}</td>
                        <td>${s.source || "-"}</td>
                        <td>${formatDate(s.timestamp)}</td>
                    </tr>
                `,
                  )
                  .join("")}
            </tbody>
        </table>
        ${renderPagination(total)}
    `;

  document.getElementById("search").addEventListener("keypress", (e) => {
    if (e.key === "Enter") doSearch();
  });
}

function renderRateLimitsList(main) {
  const { total, rate_limits } = state.data;

  main.innerHTML = `
        <div class="header">
            <h2>Rate Limits</h2>
        </div>
        <table>
            <thead>
                <tr>
                    <th>API Key</th>
                    <th>Requests (window)</th>
                    <th>Created</th>
                </tr>
            </thead>
            <tbody>
                ${rate_limits
                  .map(
                    (r) => `
                    <tr>
                        <td><span class="mono">${r.api_key}...</span></td>
                        <td>${r.request_count || 0}</td>
                        <td>${formatDate(r.created_at)}</td>
                    </tr>
                `,
                  )
                  .join("")}
            </tbody>
        </table>
        <div class="pagination">
            <div class="pagination-info">Total: ${total}</div>
        </div>
    `;
}

function renderDiagnostics(main) {
  const { storage, players } = state.data;

  main.innerHTML = `
        <div class="header">
            <h2>Cache Diagnostics</h2>
        </div>

        <div class="detail-panel">
            <div class="detail-grid">
                <div class="detail-item">
                    <label>Total Snapshots</label>
                    <div class="value">${storage.total_snapshots.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Baselines</label>
                    <div class="value">${storage.total_baselines.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Deltas</label>
                    <div class="value">${storage.total_deltas.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Unique Players</label>
                    <div class="value">${storage.total_players.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Auto-Promotions</label>
                    <div class="value">${storage.total_promotions.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Avg Deltas/Baseline</label>
                    <div class="value">${storage.avg_deltas_per_baseline.toFixed(2)}</div>
                </div>
                <div class="detail-item">
                    <label>Storage Efficiency</label>
                    <div class="value">${storage.storage_efficiency.toFixed(1)}% deltas</div>
                </div>
            </div>
        </div>

        <h3 class="section-title">Top 50 Players by Delta Count</h3>
        <table>
            <thead>
                <tr>
                    <th>UUID</th>
                    <th>Username</th>
                    <th>Baselines</th>
                    <th>Deltas</th>
                    <th>Chain Length</th>
                    <th>Reconstruct Time</th>
                    <th>Baseline Age</th>
                </tr>
            </thead>
            <tbody>
                ${players
                  .map(
                    (p) => `
                    <tr>
                        <td>${formatUuid(p.uuid)}</td>
                        <td>${p.username || '<span class="text-muted">-</span>'}</td>
                        <td>${p.baseline_count}</td>
                        <td>${p.delta_count}</td>
                        <td>${p.delta_chain_length}</td>
                        <td>${formatReconstructTime(p.reconstruct_time_us)}</td>
                        <td>${formatBaselineAge(p.latest_baseline_age_hours)}</td>
                    </tr>
                `,
                  )
                  .join("")}
            </tbody>
        </table>
    `;
}

function formatReconstructTime(us) {
  if (us === null || us === undefined)
    return '<span class="text-muted">-</span>';
  if (us === 0) return '<span class="text-muted">0</span>';
  if (us < 1000) return `${us}µs`;
  if (us < 1000000) return `${(us / 1000).toFixed(2)}ms`;
  return `${(us / 1000000).toFixed(2)}s`;
}

function formatBaselineAge(hours) {
  if (hours === null || hours === undefined)
    return '<span class="text-muted">-</span>';
  if (hours < 1) return `${Math.round(hours * 60)}m ago`;
  if (hours < 24) return `${hours.toFixed(1)}h ago`;
  return `${(hours / 24).toFixed(1)}d ago`;
}

function renderDetail() {
  const main = document.getElementById("main");

  if (!state.data) {
    main.innerHTML = `
            <button class="back-btn" onclick="navigate('${state.page}')">← Back</button>
            <div class="empty">Not found</div>
        `;
    return;
  }

  switch (state.page) {
    case "members":
      renderMemberDetail(main);
      break;
    case "blacklist":
      renderBlacklistDetail(main);
      break;
    case "snapshots":
      renderSnapshotDetail(main);
      break;
  }
}

function renderMemberDetail(main) {
  const m = state.data;

  main.innerHTML = `
        <button class="back-btn" onclick="navigate('members')">← Back</button>
        <div class="detail-panel">
            <div class="detail-grid">
                <div class="detail-item">
                    <label>ID</label>
                    <div class="value">${m.id}</div>
                </div>
                <div class="detail-item">
                    <label>Discord ID</label>
                    <div class="value">${formatDiscordId(m.discord_id)}</div>
                </div>
                <div class="detail-item">
                    <label>UUID</label>
                    <div class="value">${formatUuid(m.uuid)}</div>
                </div>
                <div class="detail-item">
                    <label>API Key</label>
                    <div class="value"><span class="mono">${m.api_key_preview || "-"}${m.api_key_preview ? "..." : ""}</span></div>
                </div>
                <div class="detail-item">
                    <label>Access Level</label>
                    <div class="value">${renderBadges(m)}</div>
                </div>
                <div class="detail-item">
                    <label>Total Requests</label>
                    <div class="value">${m.request_count.toLocaleString()}</div>
                </div>
                <div class="detail-item">
                    <label>Joined</label>
                    <div class="value">${formatDate(m.join_date)}</div>
                </div>
                <div class="detail-item">
                    <label>Updated</label>
                    <div class="value">${formatDate(m.updated_at)}</div>
                </div>
            </div>
        </div>

        ${
          m.ips.length > 0
            ? `
            <h3 class="section-title">IP History (${m.ips.length})</h3>
            <table>
                <thead>
                    <tr>
                        <th>IP Address</th>
                        <th>First Seen</th>
                        <th>Last Seen</th>
                    </tr>
                </thead>
                <tbody>
                    ${m.ips
                      .map(
                        (ip) => `
                        <tr>
                            <td><span class="mono">${ip.ip_address}</span></td>
                            <td>${formatDate(ip.first_seen)}</td>
                            <td>${formatDate(ip.last_seen)}</td>
                        </tr>
                    `,
                      )
                      .join("")}
                </tbody>
            </table>
        `
            : ""
        }

        ${
          m.alt_accounts.length > 0
            ? `
            <h3 class="section-title">Alt Accounts (${m.alt_accounts.length})</h3>
            <table>
                <thead>
                    <tr>
                        <th>UUID</th>
                        <th>Added</th>
                    </tr>
                </thead>
                <tbody>
                    ${m.alt_accounts
                      .map(
                        (a) => `
                        <tr>
                            <td>${formatUuid(a.uuid)}</td>
                            <td>${formatDate(a.added_at)}</td>
                        </tr>
                    `,
                      )
                      .join("")}
                </tbody>
            </table>
        `
            : ""
        }

        <h3 class="section-title">Config</h3>
        <div class="json-viewer">${JSON.stringify(m.config, null, 2)}</div>
    `;
}

function renderBlacklistDetail(main) {
  const { player, tags, tag_history } = state.data;

  main.innerHTML = `
        <button class="back-btn" onclick="navigate('blacklist')">← Back</button>
        <div class="detail-panel">
            <div class="detail-grid">
                <div class="detail-item">
                    <label>UUID</label>
                    <div class="value">${formatUuid(player.uuid)}</div>
                </div>
                <div class="detail-item">
                    <label>Status</label>
                    <div class="value">${player.is_locked ? '<span class="badge badge-locked">Locked</span>' : '<span class="text-muted">Active</span>'}</div>
                </div>
                ${
                  player.lock_reason
                    ? `
                    <div class="detail-item">
                        <label>Lock Reason</label>
                        <div class="value">${player.lock_reason}</div>
                    </div>
                `
                    : ""
                }
            </div>
        </div>

        <h3 class="section-title">Active Tags (${tags.length})</h3>
        ${
          tags.length > 0
            ? `
            <div class="tags-list">
                ${tags
                  .map(
                    (t) => `
                    <div class="tag-card">
                        <div class="tag-type">${renderTagBadge(t.tag_type)}</div>
                        <div class="tag-reason">${t.reason}</div>
                        <div class="tag-meta">
                            Added by ${t.added_by} on ${formatDate(t.added_on)}
                            ${t.hide_username ? " • Username hidden" : ""}
                        </div>
                    </div>
                `,
                  )
                  .join("")}
            </div>
        `
            : '<div class="empty">No active tags</div>'
        }

        ${
          tag_history.length > 0
            ? `
            <h3 class="section-title">Tag History (${tag_history.length} removed)</h3>
            <div class="tags-list">
                ${tag_history
                  .map(
                    (t) => `
                    <div class="tag-card" style="opacity: 0.6">
                        <div class="tag-type">${renderTagBadge(t.tag_type)}</div>
                        <div class="tag-reason">${t.reason}</div>
                        <div class="tag-meta">
                            Added by ${t.added_by} on ${formatDate(t.added_on)}<br>
                            Removed by ${t.removed_by} on ${formatDate(t.removed_on)}
                        </div>
                    </div>
                `,
                  )
                  .join("")}
            </div>
        `
            : ""
        }
    `;
}

function renderSnapshotDetail(main) {
  const s = state.data;

  main.innerHTML = `
        <button class="back-btn" onclick="navigate('snapshots')">← Back</button>
        <div class="detail-panel">
            <div class="detail-grid">
                <div class="detail-item">
                    <label>Type</label>
                    <div class="value"><span class="baseline-indicator ${s.is_baseline ? "is-baseline" : "is-delta"}"></span>${s.is_baseline ? "Baseline" : "Delta"}</div>
                </div>
                <div class="detail-item">
                    <label>UUID</label>
                    <div class="value">${formatUuid(s.uuid)}</div>
                </div>
                <div class="detail-item">
                    <label>Username</label>
                    <div class="value">${s.username || "-"}</div>
                </div>
                <div class="detail-item">
                    <label>Source</label>
                    <div class="value">${s.source || "-"}</div>
                </div>
                <div class="detail-item">
                    <label>Discord ID</label>
                    <div class="value">${s.discord_id ? formatDiscordId(s.discord_id) : "-"}</div>
                </div>
                <div class="detail-item">
                    <label>Timestamp</label>
                    <div class="value">${formatDate(s.timestamp)}</div>
                </div>
            </div>
        </div>

        <h3 class="section-title">Data</h3>
        <div class="json-viewer">${JSON.stringify(s.data, null, 2)}</div>
    `;
}

function renderPagination(total) {
  const start = state.offset + 1;
  const end = Math.min(state.offset + state.limit, total);
  const hasPrev = state.offset > 0;
  const hasNext = state.offset + state.limit < total;

  return `
        <div class="pagination">
            <div class="pagination-info">Showing ${start}-${end} of ${total}</div>
            <div class="pagination-buttons">
                <button ${!hasPrev ? "disabled" : ""} onclick="setOffset(${state.offset - state.limit})">Previous</button>
                <button ${!hasNext ? "disabled" : ""} onclick="setOffset(${state.offset + state.limit})">Next</button>
            </div>
        </div>
    `;
}

function doSearch() {
  const input = document.getElementById("search");
  if (input) {
    state.search = input.value;
    state.offset = 0;
    loadData();
  }
}

function render() {
  document.querySelectorAll(".nav-item").forEach((el) => {
    el.classList.toggle("active", el.dataset.page === state.page);
  });
}

document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll(".nav-item").forEach((el) => {
    el.addEventListener("click", () => navigate(el.dataset.page));
  });

  render();
  loadData();
});
