function getDisplayName(lang) {
  const nameMap = {
    CSharp: "C#",
    CPP: "C++",
  };
  return nameMap[lang] || lang;
}

document.addEventListener("DOMContentLoaded", () => {
  const languageContentDiv = document.getElementById("language-content");
  const loadingMessage = document.getElementById("loading-message");
  const languageTitle = document.getElementById("language-title");

  initTheme();

  const params = new URLSearchParams(window.location.search);
  const language = params.get("lang");

  if (!language) {
    loadingMessage.textContent = "Error: Language not specified in URL.";
    return;
  }

  const displayName = getDisplayName(language);

  const pageTitle = `kstars: Top 1000 GitHub Repos for ${displayName}`;
  languageTitle.textContent = `kstars ${displayName}`;
  document.title = pageTitle;

  const csvPath = `../data/processed/${language}.csv`;

  Papa.parse(csvPath, {
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      if (results.data && results.data.length > 1) {
        loadingMessage.style.display = "none";
        const tableContainer = document.createElement("div");
        tableContainer.className = "table-container";
        const table = createTable(results.data);
        tableContainer.appendChild(table);
        languageContentDiv.appendChild(tableContainer);
        Sortable.init();
      } else {
        loadingMessage.textContent = `No repository data found for ${language}.`;
      }
    },
    error: function (err) {
      console.error(`Error loading CSV for ${language} from ${csvPath}:`, err);
      loadingMessage.textContent = `Could not load repository data for ${language}.`;
    },
  });
});
