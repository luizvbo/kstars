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

document.addEventListener("DOMContentLoaded", function () {
    const themeToggle = document.getElementById("themeToggle");
    const themeIcon = document.getElementById("themeIcon");
    const contentDiv = document.getElementById("content");
    const navLinksDiv = document.getElementById("language-nav-links");
    
    const navToggleBtn = document.getElementById("navToggleBtn");
    const languageNav = document.getElementById("language-nav");

    let loadedLanguagesCount = 0;

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
    
    if (navToggleBtn && languageNav) {
        navToggleBtn.addEventListener('click', (e) => {
            e.stopPropagation(); 
            languageNav.classList.toggle('nav-visible');
        });

        navLinksDiv.addEventListener('click', (e) => {
            if (e.target.tagName === 'A') {
                languageNav.classList.remove('nav-visible');
            }
        });
        
        contentDiv.addEventListener('click', () => {
            languageNav.classList.remove('nav-visible');
        });
    }

    languages.forEach(lang => {
        const link = document.createElement('a');
        link.href = `#${lang[0]}`;
        link.textContent = lang[1];
        navLinksDiv.appendChild(link);
    });

    languages.forEach((language) => loadCSV(language, "data/processed", "top10_"));
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

languages.forEach((lang) => {
  const link = document.createElement("a");
  link.href = `#${lang[0]}`;
  link.textContent = lang[1];
  navLinksDiv.appendChild(link);
});

languages.forEach((language) => loadCSV(language, "data/processed", "top10_"));
