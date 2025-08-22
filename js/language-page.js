// Function to truncate a string to maxChars, making sure that it stops at the last word.
// E.g., truncateStringAtWord("I love birds", 10) returns "I love..." and not "I love bir..."
function truncateStringAtWord(str, maxChars) {
  if (str.length <= maxChars) {
    return str;
  }

  const truncated = str.slice(0, maxChars);
  const lastSpaceIndex = truncated.lastIndexOf(" ");

  if (lastSpaceIndex === -1) {
    return truncated + "...";
  }

  return truncated.slice(0, lastSpaceIndex) + "...";
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
