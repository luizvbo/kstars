// List of languages with corresponding CSV file names.
// For each language, you should have a "top10_<Language>.csv" (for the homepage)
// and a "<Language>.csv" (for the expanded page).
const languages = [
  { name: "R", top10: "top10_R.csv", full: "R.csv" },
  // Add additional languages here, e.g.:
  // { name: "Python", top10: "top10_Python.csv", full: "Python.csv" },
  // { name: "JavaScript", top10: "top10_JavaScript.csv", full: "JavaScript.csv" },
  // ... up to 34 languages.
];

const contentDiv = document.getElementById("content");

// Function to create a sortable table from CSV data.
function createTable(data) {
  const table = document.createElement("table");
  table.classList.add("sortable");

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
  for (let i = 1; i < data.length; i++) {
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
  Papa.parse("results/" + language.top10, {
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
      // This assumes you have a separate HTML page (e.g., R.html) that displays the full table.
      link.href = language.full.replace(".csv", ".html");
      link.textContent = "View full list";
      headerDiv.appendChild(link);

      sectionDiv.appendChild(headerDiv);

      // Create and append the table.
      const table = createTable(results.data);
      sectionDiv.appendChild(table);

      contentDiv.appendChild(sectionDiv);
    },
    error: function (err) {
      console.error("Error loading CSV for " + language.name, err);
    },
  });
}

// Load each language’s top-10 CSV.
languages.forEach((language) => loadCSV(language));

// Theme toggle functionality.
const themeToggle = document.getElementById("themeToggle");
themeToggle.addEventListener("click", function () {
  document.body.classList.toggle("dark");
});
