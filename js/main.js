const contentDiv = document.getElementById("content");
const navLinksDiv = document.getElementById("language-nav-links");
let loadedLanguagesCount = 0;

function loadCSV(language, folder, prefix) {
  Papa.parse(`${folder}/${prefix}${language[0]}.csv`, {
    download: true,
    skipEmptyLines: "greedy",
    complete: function (results) {
      const content =
        results.data && results.data.length > 1
          ? createTable(results.data, 10) // Show top 10
          : "Could not load preview data.";
      finishSection(language, content);
    },
    error: function () {
      finishSection(language, "Could not load preview data.");
    },
  });
}

function finishSection(language, content) {
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

  if (typeof content === "string") {
    sectionDiv.appendChild(document.createTextNode(content));
  } else {
    const tableContainer = document.createElement("div");
    tableContainer.className = "table-container";
    tableContainer.appendChild(content);
    sectionDiv.appendChild(tableContainer);
  }

  contentDiv.appendChild(sectionDiv);
  loadedLanguagesCount++;
  if (loadedLanguagesCount === languages.length) {
    Sortable.init();
  }
}

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

document.addEventListener("DOMContentLoaded", function () {
  const navToggleBtn = document.getElementById("navToggleBtn");
  const languageNav = document.getElementById("language-nav");

  initTheme();

  if (navToggleBtn && languageNav) {
    navToggleBtn.addEventListener("click", (e) => {
      e.stopPropagation();
      languageNav.classList.toggle("nav-visible");
    });

    navLinksDiv.addEventListener("click", (e) => {
      if (e.target.tagName === "A") {
        languageNav.classList.remove("nav-visible");
      }
    });

    contentDiv.addEventListener("click", () => {
      languageNav.classList.remove("nav-visible");
    });
  }

  languages.forEach((lang) => {
    const link = document.createElement("a");
    link.href = `#${lang[0]}`;
    link.textContent = lang[1];
    navLinksDiv.appendChild(link);
  });

  languages.forEach((language) =>
    loadCSV(language, "data/processed", "top10_"),
  );
});
