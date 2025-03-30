document.addEventListener('DOMContentLoaded', () => {
    const languageContentDiv = document.getElementById('language-content');
    const loadingMessage = document.getElementById('loading-message');
    const languageTitle = document.getElementById('language-title');
    const themeToggle = document.getElementById('themeToggle');

    // Helper Function to Create Table (similar to main.js)
    function createTable(data, fullTable = false) {
        const table = document.createElement("table");
        table.setAttribute("data-sortable", ""); // Enable sortable.js
        // Add dark class if body has it initially
        if (document.body.classList.contains('dark')) {
             table.classList.add('dark', 'sortable-theme-dark'); // Add classes for sortable dark theme
        } else {
             table.classList.add('sortable-theme-light'); // Add classes for sortable light theme
        }


        // Create table header
        const thead = document.createElement("thead");
        const headerRow = document.createElement("tr");
        if (data.length > 0) {
             data[0].forEach((colText) => {
                 const th = document.createElement("th");
                 th.textContent = colText;
                 headerRow.appendChild(th);
             });
        }
        thead.appendChild(headerRow);
        table.appendChild(thead);

        // Create table body
        const tbody = document.createElement("tbody");
        // Start from index 1 to skip header row in data
        // Iterate up to data.length - 1 to potentially skip trailing empty line from PapaParse
        const endRow = data.length > 1 && data[data.length - 1].length === 1 && data[data.length - 1][0] === '' ? data.length - 1 : data.length;
        for (let i = 1; i < endRow; i++) {
            const row = document.createElement("tr");
            // Ensure we have data for the row before iterating
            if(data[i]) {
                 data[i].forEach((cellText) => {
                     const td = document.createElement("td");
                     td.textContent = cellText;
                     row.appendChild(td);
                 });
                 tbody.appendChild(row);
            }
        }
        table.appendChild(tbody);
        return table;
    }

    // --- Get Language from URL Query Parameter ---
    const params = new URLSearchParams(window.location.search);
    const language = params.get('lang'); // Get the 'lang' parameter

    if (!language) {
        loadingMessage.textContent = 'Error: Language not specified in URL.';
        languageTitle.textContent = 'Error';
        document.title = 'kstars: Error';
        return; // Stop execution if no language is specified
    }

    // --- Update Page Titles ---
    const pageTitle = `kstars: Top 1000 GitHub Repos for ${language}`;
    languageTitle.textContent = `Top 1000 GitHub Repos for ${language}`;
    document.title = pageTitle;

    // --- Construct CSV Path and Load Data ---
    // Full CSVs are loaded from data/processed/ and named like 'Python.csv', 'C++.csv'
    const csvPath = `../data/processed/${language}.csv`;

    Papa.parse(csvPath, {
        download: true,
        skipEmptyLines: 'greedy', // Important for potentially inconsistent CSV endings
        complete: function(results) {
            loadingMessage.style.display = 'none'; // Hide loading message

            if (results.data && results.data.length > 1) { // Check if data exists and has more than just header
                const table = createTable(results.data, true); // Pass true for full table styling/logic if needed later
                languageContentDiv.appendChild(table);
                Sortable.init(); // Initialize sorting *after* table is in the DOM
            } else if (results.errors.length > 0) {
                 console.error(`Errors parsing CSV for ${language}:`, results.errors);
                 languageContentDiv.innerHTML = `<p>Error parsing CSV data for ${language}. Check console for details.</p>`;
            }
             else {
                 languageContentDiv.innerHTML = `<p>No repository data found for ${language}.</p>`;
            }
        },
        error: function(err) {
            loadingMessage.style.display = 'none'; // Hide loading message
            console.error(`Error loading CSV for ${language} from ${csvPath}:`, err);
            languageContentDiv.innerHTML = `<p>Could not load repository data for ${language}. Please check if the file exists at <code>${csvPath}</code> and that the language name is correct.</p>`;
        }
    });

     // --- Theme Toggle Functionality (same as main.js) ---
     function applyTheme(isDark) {
         if (isDark) {
             document.body.classList.add('dark');
             document.querySelectorAll('table[data-sortable]').forEach(table => {
                table.classList.remove('sortable-theme-light');
                table.classList.add('dark', 'sortable-theme-dark');
             });
             document.querySelector('header')?.classList.add('dark'); // Optional: Add dark class to header if needed
         } else {
             document.body.classList.remove('dark');
             document.querySelectorAll('table[data-sortable]').forEach(table => {
                table.classList.remove('dark', 'sortable-theme-dark');
                table.classList.add('sortable-theme-light');
             });
             document.querySelector('header')?.classList.remove('dark'); // Optional: Remove dark class from header
         }
     }

     // Check localStorage for saved theme preference
    const savedTheme = localStorage.getItem('theme');
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    // Initialize theme based on saved preference or system preference
    applyTheme(savedTheme === 'dark' || (!savedTheme && prefersDark));


    themeToggle.addEventListener('click', function() {
        const isDark = document.body.classList.toggle('dark');
        applyTheme(isDark);
         // Save preference
        localStorage.setItem('theme', isDark ? 'dark' : 'light');
    });

});
