const NUMERIC_HEADERS = new Set([
  "Ranking",
  "Stars",
  "Forks",
  "Open Issues",
  "Size",
]);
const HEADER_TO_CLASS_MAP = {
  Ranking: "td-ranking",
  Stars: "td-stars",
  Forks: "td-forks",
  "Open Issues": "td-open-issues",
  "Created At": "td-created-at",
  "Last Commit": "td-last-commit",
  Size: "td-size",
  Description: "td-description",
  "Project Name": "td-project-name",
  "Repo URL": "td-repo-url",
  Repository: "td-repo-url",
  Language: "td-language",
};

function truncateStringAtWord(str, maxChars) {
  if (!str || str.length <= maxChars) return str;
  const truncated = str.slice(0, maxChars);
  const lastSpaceIndex = truncated.lastIndexOf(" ");
  return (
    (lastSpaceIndex === -1 ? truncated : truncated.slice(0, lastSpaceIndex)) +
    "..."
  );
}

function sizeToKb(str) {
  const match = /^([\d.]+)\s*(KB|MB|GB|TB)$/i.exec(str || "");
  const multiplier = { KB: 1, MB: 1024, GB: 1024 ** 2, TB: 1024 ** 3 };
  return match ? parseFloat(match[1]) * multiplier[match[2].toUpperCase()] : null;
}

function createTable(data, maxRows) {
  const table = document.createElement("table");
  table.setAttribute("data-sortable", "");

  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  const headers = data[0];

  const repoUrlIndex =
    headers.indexOf("Repository") !== -1
      ? headers.indexOf("Repository")
      : headers.indexOf("Repo URL");

  headers.forEach((colText) => {
    const th = document.createElement("th");
    th.textContent = colText;
    if (NUMERIC_HEADERS.has(colText)) {
      th.setAttribute("data-sortable-type", "numeric");
    }
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  const tbody = document.createElement("tbody");
  const rowsToRender = maxRows
    ? Math.min(data.length, maxRows + 1)
    : data.length;

  for (let i = 1; i < rowsToRender; i++) {
    const rowData = data[i];
    if (!rowData || rowData.length < headers.length) continue;

    const row = document.createElement("tr");

    if (repoUrlIndex !== -1 && rowData[repoUrlIndex]) {
      row.style.cursor = "pointer"; // Add visual feedback
      row.addEventListener("click", () => {
        window.open(rowData[repoUrlIndex], "_blank");
      });
    }

    rowData.forEach((cellText, colIndex) => {
      const td = document.createElement("td");
      const headerText = headers[colIndex];

      if (HEADER_TO_CLASS_MAP[headerText]) {
        td.classList.add(HEADER_TO_CLASS_MAP[headerText]);
      }

      if (headerText === "Size") {
        const kb = sizeToKb(cellText);
        if (kb !== null) td.setAttribute("data-value", kb);
      }

      if (colIndex === repoUrlIndex && cellText) {
        const link = document.createElement("a");
        link.href = cellText;
        link.target = "_blank";
        link.textContent = cellText.replace("https://github.com/", "");
        link.addEventListener("click", (e) => e.stopPropagation());
        td.appendChild(link);
      } else {
        td.textContent = truncateStringAtWord(cellText, 150);
      }
      row.appendChild(td);
    });
    tbody.appendChild(row);
  }
  table.appendChild(tbody);
  return table;
}

function initTheme() {
  const themeToggle = document.getElementById("themeToggle");
  const themeIcon = document.getElementById("themeIcon");

  function applyTheme(isDark) {
    document.documentElement.classList.toggle("dark", isDark);
    themeIcon.textContent = isDark ? "☀️" : "🌙";
  }

  applyTheme(localStorage.getItem("theme") === "dark");

  themeToggle.addEventListener("click", function () {
    const isDark = !document.documentElement.classList.contains("dark");
    applyTheme(isDark);
    localStorage.setItem("theme", isDark ? "dark" : "light");
  });
}
