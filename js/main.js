// List of languages with corresponding CSV file names.
// For each language, you should have a "top10_<Language>.csv" (for the homepage)
// and a "<Language>.csv" (for the expanded page).
const languages = [
  { name: "C", top10: "top10_C.csv", full: "C.csv" },
];

const contentDiv = document.getElementById("content");
let loadedLanguagesCount = 0;

// Function to create a sortable table from CSV data.
function createTable(data) {
  const table = document.createElement("table");
  table.setAttribute("data-sortable", "");

  // Create table header.
  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  data[0].forEach((col) => {
    const th = document.createElement("th");
    th.textContent = col;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Create table body.
  const tbody = document.createElement("tbody");
  // Start from index 1 to skip header row.
  for (let i = 1; i < data.length - 1; i++) {
    const row = document.createElement("tr");
    data[i].forEach((cell) => {
      const td = document.createElement("td");
      td.textContent = cell;
      row.appendChild(td);
    });
    tbody.appendChild(row);
  }
  table.appendChild(tbody);
  return table;
}

// Function to load a CSV file and add its table to the page.
function loadCSV(language) {
  Papa.parse("data/processed/" + language.top10, {
    download: true,
    complete: function (results) {
      const sectionDiv = document.createElement("div");
      sectionDiv.classList.add("language-section");

      // Create header for the language section.
      const headerDiv = document.createElement("div");
      headerDiv.classList.add("language-header");
      const h2 = document.createElement("h2");
      h2.textContent = language.name;
      headerDiv.appendChild(h2);

      // Create a link to the expanded table page.
      const link = document.createElement("a");
      link.href = language.full.replace(".csv", ".html");
      link.textContent = "View full list";
      headerDiv.appendChild(link);

      sectionDiv.appendChild(headerDiv);

      // Create and append the table.
      const table = createTable(results.data);
      sectionDiv.appendChild(table);

      contentDiv.appendChild(sectionDiv);

      // Increment the counter and check if all languages are loaded.
      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init();
      }
    },
    error: function (err) {
      console.error("Error loading CSV for " + language.name, err);
      // Even if there's an error, you might want to check if all expected loads are attempted
      loadedLanguagesCount++;
      if (loadedLanguagesCount === languages.length) {
        Sortable.init();
      }
    },
  });
}

// Load each language’s top-10 CSV.
languages.forEach((language) => loadCSV(language));
document.addEventListener("DOMContentLoaded", Sortable.init);

// Theme toggle functionality.
const themeToggle = document.getElementById("themeToggle");
themeToggle.addEventListener("click", function () {
  document.body.classList.toggle("dark");
});
