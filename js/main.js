// Function to load a CSV file and add its table to the page.
function loadCSV(language, folder, prefix) {
  Papa.parse(`${folder}/${prefix}${language[0]}.csv`, {
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      const sectionDiv = document.createElement("div");
      sectionDiv.classList.add("language-section");
      sectionDiv.id = language[0]; // Add ID for linking

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
        const table = createTable(results.data);
        sectionDiv.appendChild(table);
      } else {
        sectionDiv.appendChild(
          document.createTextNode("Could not load preview data.")
        );
      }

      contentDiv.appendChild(sectionDiv);

      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init();
      }
    }
  });
}

// Function to truncate a string to maxChars, making sure that it stops at the last word.
// E.g., truncateStringAtWord("I love birds", 10) returns "I love..." and not "I love bir..."
function truncateStringAtWord(str, maxChars) {
    if (str.length <= maxChars) {
        return str;
    }

    const truncated = str.slice(0, maxChars);
    const lastSpaceIndex = truncated.lastIndexOf(' ');

    if (lastSpaceIndex === -1) {
        return truncated + '...';
    }

    return truncated.slice(0, lastSpaceIndex) + '...';
}

// Function to create a sortable table from CSV data.
// Make sure this function exists and is accessible by loadCSV
function createTable(data) {
  const table = document.createElement("table");
  table.setAttribute("data-sortable", "");
  table.classList.add(document.body.classList.contains("dark") ? "sortable-theme-dark" : "sortable-theme-light");

  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");

  if (data && data[0]) {
    data[0].forEach((col, index) => {
      const th = document.createElement("th");
      th.textContent = col;
      th.setAttribute("data-index", index);
      headerRow.appendChild(th);
    });
  }
  thead.appendChild(headerRow);
  table.appendChild(thead);

  const tbody = document.createElement("tbody");
  for (let i = 1; i < Math.min(data.length, 11); i++) {
    const row = document.createElement("tr");
    data[i].forEach((cell) => {
      const td = document.createElement("td");
      td.textContent = truncateStringAtWord(cell, 150);
      row.appendChild(td);
    });
    tbody.appendChild(row);
  }
  table.appendChild(tbody);
  return table;
}

document.addEventListener("DOMContentLoaded", function () {
  document.querySelectorAll("table[data-sortable] th").forEach(th => {
    th.addEventListener("click", function () {
      const sortedAsc = this.getAttribute("data-sorted-direction") === "ascending";
      document.querySelectorAll("th").forEach(th => th.removeAttribute("data-sorted"));
      this.setAttribute("data-sorted", "true");
      this.setAttribute("data-sorted-direction", sortedAsc ? "descending" : "ascending");
    });
  });
});

// Theme Toggle
const themeToggle = document.getElementById("themeToggle");
const themeIcon = document.getElementById("themeIcon");

function applyTheme(isDark) {
  document.body.classList.toggle("dark", isDark);
  document.querySelectorAll("table[data-sortable]").forEach(table => {
    table.classList.toggle("sortable-theme-dark", isDark);
    table.classList.toggle("sortable-theme-light", !isDark);
  });
  themeIcon.textContent = isDark ? "☀️" : "🌙";
}

const savedTheme = localStorage.getItem("theme");
applyTheme(savedTheme === "dark");

themeToggle.addEventListener("click", function () {
  const isDark = document.body.classList.contains("dark");
  applyTheme(!isDark);
  localStorage.setItem("theme", !isDark ? "dark" : "light");
});

// Ensure languages array is defined before this loop
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
let loadedLanguagesCount = 0; // Ensure this is defined

// Load each language’s top-10 CSV.
languages.forEach((language) => loadCSV(language, "data/processed", "top10_"));
