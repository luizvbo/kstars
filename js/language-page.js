const NUMERIC_HEADERS = new Set([
  "Ranking",
  "Stars",
  "Forks",
  "Watchers",
  "Open Issues",
  "Size (KB)",
]);
const HEADER_TO_CLASS_MAP = {
  Ranking: "td-ranking",
  Stars: "td-stars",
  Forks: "td-forks",
  Watchers: "td-watchers",
  "Open Issues": "td-open-issues",
  "Created At": "td-created-at",
  "Last Commit": "td-last-commit",
  Size: "td-size",
  "Size (KB)": "td-size-kb",
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

function createTable(data) {
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
  for (let i = 1; i < data.length; i++) {
    const rowData = data[i];
    if (!rowData || rowData.length < headers.length) continue;

    const row = document.createElement("tr");

    if (repoUrlIndex !== -1 && rowData[repoUrlIndex]) {
      row.style.cursor = "pointer";
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

document.addEventListener("DOMContentLoaded", () => {
  const languageContentDiv = document.getElementById("language-content");
  const loadingMessage = document.getElementById("loading-message");
  const languageTitle = document.getElementById("language-title");
  const themeToggle = document.getElementById("themeToggle");
  const themeIcon = document.getElementById("themeIcon");

  const params = new URLSearchParams(window.location.search);
  const language = params.get("lang");

  if (!language) {
    loadingMessage.textContent = "Error: Language not specified in URL.";
    return;
  }

  const pageTitle = `kstars: Top 1000 GitHub Repos for ${language}`;
  languageTitle.textContent = `Top 1000 GitHub Repos for ${language}`;
  document.title = pageTitle;

  const csvPath = `../data/processed/${language}.csv`;

  Papa.parse(csvPath, {
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      loadingMessage.style.display = "none";
      if (results.data && results.data.length > 1) {
        const tableContainer = document.createElement("div");
        tableContainer.className = "table-container";
        const table = createTable(results.data);
        tableContainer.appendChild(table);
        languageContentDiv.appendChild(tableContainer);
        Sortable.init();
      } else {
        languageContentDiv.innerHTML = `<p>No repository data found for ${language}.</p>`;
      }
    },
    error: function (err) {
      loadingMessage.style.display = "none";
      console.error(`Error loading CSV for ${language} from ${csvPath}:`, err);
      languageContentDiv.innerHTML = `<p>Could not load repository data for ${language}.</p>`;
    },
  });

  function applyTheme(isDark) {
    document.body.classList.toggle("dark", isDark);
    themeIcon.textContent = isDark ? "☀️" : "🌙";
  }
  const savedTheme = localStorage.getItem("theme");
  applyTheme(savedTheme === "dark");
  themeToggle.addEventListener("click", function () {
    const isDark = !document.body.classList.contains("dark");
    applyTheme(isDark);
    localStorage.setItem("theme", isDark ? "dark" : "light");
  });
});
