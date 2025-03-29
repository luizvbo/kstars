// FILE: ./js/main.js
// ... (keep existing code above loadCSV) ...

// Function to load a CSV file and add its table to the page.
function loadCSV(language, folder, prefix) {
  Papa.parse(`${folder}/${prefix}${language}.csv`, {
    // Use template literal for clarity
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      const sectionDiv = document.createElement("div");
      sectionDiv.classList.add("language-section");

      // Create header for the language section.
      const headerDiv = document.createElement("div");
      headerDiv.classList.add("language-header");
      const h2 = document.createElement("h2");
      h2.textContent = language;
      headerDiv.appendChild(h2);

      // --- MODIFIED LINK ---
      // Create a link to the new single language page, passing the language as a query parameter.
      const link = document.createElement("a");
      // Use encodeURIComponent for languages like 'C++' or 'Vim script'
      link.href = `pages/language.html?lang=${encodeURIComponent(language)}`;
      link.textContent = "View full list (Top 1000)"; // Update link text slightly
      headerDiv.appendChild(link);
      // --- END MODIFIED LINK ---

      sectionDiv.appendChild(headerDiv);

      // Create and append the table (only top 10 for index page).
      if (results.data && results.data.length > 1) {
        // Pass the createTable function from this file (assuming it's defined here)
        const table = createTable(results.data); // createTable needs to be defined/accessible here
        sectionDiv.appendChild(table);
      } else {
        sectionDiv.appendChild(
          document.createTextNode("Could not load preview data."),
        );
        console.error(`No data or only header found for ${language} preview.`);
      }

      contentDiv.appendChild(sectionDiv);

      // Increment the counter and check if all languages are loaded.
      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init(); // Initialize sortable for all preview tables
      }
    },
    error: function (err) {
      console.error(`Error loading CSV for ${language} preview:`, err);
      // Display error in the section
      const sectionDiv = document.createElement("div");
      sectionDiv.classList.add("language-section");
      sectionDiv.innerHTML = `<h2>${language}</h2><p>Error loading preview data.</p>`;
      contentDiv.appendChild(sectionDiv);

      // Even if there's an error, increment count to eventually init Sortable
      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init();
      }
    },
  });
}

// Function to create a sortable table from CSV data.
// Make sure this function exists and is accessible by loadCSV
function createTable(data) {
  const table = document.createElement("table");
  table.setAttribute("data-sortable", "");
  // Add theme classes based on current body class
  if (document.body.classList.contains("dark")) {
    table.classList.add("dark", "sortable-theme-dark");
  } else {
    table.classList.add("sortable-theme-light");
  }

  // Create table header.
  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  // Check if data[0] exists before iterating
  if (data && data[0]) {
    data[0].forEach((col) => {
      const th = document.createElement("th");
      th.textContent = col;
      headerRow.appendChild(th);
    });
  }
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Create table body.
  const tbody = document.createElement("tbody");
  // Iterate up to data.length - 1 to potentially skip trailing empty line from PapaParse
  const endRow =
    data.length > 1 &&
    data[data.length - 1].length === 1 &&
    data[data.length - 1][0] === ""
      ? data.length - 1
      : data.length;
  // Limit rows to 11 (1 header + 10 data) for the index page preview
  const maxRows = Math.min(endRow, 11);
  // Start from index 1 to skip header row.
  for (let i = 1; i < maxRows; i++) {
    // Use maxRows here
    const row = document.createElement("tr");
    if (data[i]) {
      // Check if row data exists
      data[i].forEach((cell) => {
        const td = document.createElement("td");
        td.textContent = cell;
        row.appendChild(td);
      });
      tbody.appendChild(row);
    }
  }
  table.appendChild(tbody);
  return table;
}

// --- Theme Toggle Functionality (ensure it applies theme to tables correctly) ---
const themeToggle = document.getElementById("themeToggle");

function applyTheme(isDark) {
  if (isDark) {
    document.body.classList.add("dark");
    // Apply dark theme to existing and future tables
    document.querySelectorAll("table[data-sortable]").forEach((table) => {
      table.classList.remove("sortable-theme-light");
      table.classList.add("dark", "sortable-theme-dark");
    });
    document.querySelector("header")?.classList.add("dark");
  } else {
    document.body.classList.remove("dark");
    // Apply light theme to existing and future tables
    document.querySelectorAll("table[data-sortable]").forEach((table) => {
      table.classList.remove("dark", "sortable-theme-dark");
      table.classList.add("sortable-theme-light");
    });
    document.querySelector("header")?.classList.remove("dark");
  }
}

// Check localStorage for saved theme preference on initial load
const savedTheme = localStorage.getItem("theme");
const prefersDark =
  window.matchMedia &&
  window.matchMedia("(prefers-color-scheme: dark)").matches;
applyTheme(savedTheme === "dark" || (!savedTheme && prefersDark));

themeToggle.addEventListener("click", function () {
  const isDark = document.body.classList.toggle("dark");
  applyTheme(isDark);
  // Save preference
  localStorage.setItem("theme", isDark ? "dark" : "light");
});

// --- Initialization ---
// Ensure languages array is defined before this loop
const languages = [
  "ActionScript",
  "C",
  "C#",
  "C++",
  "Clojure",
  "CoffeeScript",
  "CSS",
  "Dart",
  "DM",
  "Elixir",
  "Go",
  "Groovy",
  "Haskell",
  "HTML",
  "Java",
  "JavaScript",
  "Julia",
  "Kotlin",
  "Lua",
  "MATLAB",
  "Objective-C",
  "Perl",
  "PHP",
  "PowerShell",
  "Python",
  "R",
  "Ruby",
  "Rust",
  "Scala",
  "Shell",
  "Swift",
  "TeX",
  "TypeScript",
  "Vim script",
];
const contentDiv = document.getElementById("content");
let loadedLanguagesCount = 0; // Ensure this is defined

// Load each language’s top-10 CSV.
languages.forEach((language) => loadCSV(language, "data/processed", "top10_"));

// Initialize Sortable.js after the initial tables might be loaded.
// The check inside loadCSV ensures it runs only once after all attempts.
// document.addEventListener('DOMContentLoaded', () => { Sortable.init() }); // This might run too early, the check in loadCSV is safer.
