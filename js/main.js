// js/main.js

// --- NEW: A list of headers that should be treated as numbers for sorting ---
const NUMERIC_HEADERS = new Set([
  "Ranking",
  "Stars",
  "Forks",
  "Watchers",
  "Open Issues",
  "Size (KB)",
]);

// --- NEW: A map for adding CSS classes to table cells for specific styling ---
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
  Language: "td-language",
};

function loadCSV(language, folder, prefix) {
  Papa.parse(`${folder}/${prefix}${language[0]}.csv`, {
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      const sectionDiv = document.createElement("div");
      sectionDiv.classList.add("language-section");
      sectionDiv.id = language[0];

      const headerDiv = document.createElement("div");
      headerDiv.classList.add("language-header");
      const h2 = document.createElement("h2");
      h2.textContent = language[1];
      headerDiv.appendChild(h2);
      const link = document.createElement("a");
      link.href = `pages/language.html?lang=${encodeURIComponent(language[0])}`;
      link.textContent = "View full list (Top 1000)";
      link.classList.add("cta-link");
      headerDiv.appendChild(link);
      sectionDiv.appendChild(headerDiv);

      if (results.data && results.data.length > 1) {
        // --- FIX: Wrap table in a container for responsiveness ---
        const tableContainer = document.createElement("div");
        tableContainer.className = "table-container";
        const table = createTable(results.data, 10); // Show top 10
        tableContainer.appendChild(table);
        sectionDiv.appendChild(tableContainer);
      } else {
        sectionDiv.appendChild(
          document.createTextNode("Could not load preview data."),
        );
      }

      contentDiv.appendChild(sectionDiv);
      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init();
      }
    },
  });
}

function truncateStringAtWord(str, maxChars) {
  if (!str || str.length <= maxChars) return str;
  const truncated = str.slice(0, maxChars);
  const lastSpaceIndex = truncated.lastIndexOf(" ");
  return (
    (lastSpaceIndex === -1 ? truncated : truncated.slice(0, lastSpaceIndex)) +
    "..."
  );
}

// --- UPDATED: The new, smarter createTable function ---
function createTable(data, maxRows) {
  const table = document.createElement("table");
  table.setAttribute("data-sortable", "");

  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  const headers = data[0];

  headers.forEach((colText) => {
    const th = document.createElement("th");
    th.textContent = colText;
    // --- FIX: Activate numeric sorting for specific columns ---
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
    if (!rowData || rowData.length === 0) continue;

    const row = document.createElement("tr");
    rowData.forEach((cellText, colIndex) => {
      const td = document.createElement("td");
      const headerText = headers[colIndex];

      // --- FIX: Add specific classes for styling ---
      if (HEADER_TO_CLASS_MAP[headerText]) {
        td.classList.add(HEADER_TO_CLASS_MAP[headerText]);
      }

      // --- FIX: Custom rendering for Repo URL ---
      if (headerText === "Repo URL" && cellText) {
        const link = document.createElement("a");
        link.href = cellText;
        link.target = "_blank";
        link.textContent = cellText.replace("https://github.com/", "");
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

// --- Theme Toggle Logic (updated for new button) ---
document.addEventListener("DOMContentLoaded", function () {
  const themeToggle = document.getElementById("themeToggle");
  const themeIcon = document.getElementById("themeIcon");

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

const languages = [
  ["ActionScript", "ActionScript"],
  ["C", "C"],
  ["CSharp", "C#"],
  ["CPP", "C++"],
  ["Clojure", "Clojure"],
  ["CoffeeScript", "CoffeeScript"],
  ["CSS", "CSS"],
  ["Dart", "Dart"],
  ["DM", "DM"],
  ["Elixir", "Elixir"],
  ["Go", "Go"],
  ["Groovy", "Groovy"],
  ["Haskell", "Haskell"],
  ["HTML", "HTML"],
  ["Java", "Java"],
  ["JavaScript", "JavaScript"],
  ["Julia", "Julia"],
  ["Kotlin", "Kotlin"],
  ["Lua", "Lua"],
  ["MATLAB", "MATLAB"],
  ["Objective-C", "Objective-C"],
  ["Perl", "Perl"],
  ["PHP", "PHP"],
  ["PowerShell", "PowerShell"],
  ["Prolog", "Prolog"],
  ["Python", "Python"],
  ["R", "R"],
  ["Ruby", "Ruby"],
  ["Rust", "Rust"],
  ["Scala", "Scala"],
  ["Shell", "Shell"],
  ["Swift", "Swift"],
  ["TeX", "TeX"],
  ["TypeScript", "TypeScript"],
  ["Vim-script", "Vim script"],
];

const contentDiv = document.getElementById("content");
const navLinksDiv = document.getElementById("language-nav-links");
let loadedLanguagesCount = 0;

// --- NEW: Populate the navigation bar ---
languages.forEach((lang) => {
  const link = document.createElement("a");
  link.href = `#${lang[0]}`;
  link.textContent = lang[1];
  navLinksDiv.appendChild(link);
});

languages.forEach((language) => loadCSV(language, "data/processed", "top10_"));
