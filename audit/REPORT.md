# Repository Audit Report — kstars

**Date:** 2026-10-09
**Branch:** `fix/repo-audit-2026-10`
**Auditor:** Devin (automated repository analysis)

---

## 1. Executive Summary

`kstars` is a small data-driven static site: a Rust CLI (`kstars/`) fetches the
top-1000-starred GitHub repositories per programming language via the GitHub
Search API, a Python/cron script (`kstars/main.py`) post-processes the CSVs and
regenerates `README.md`, and a dependency-free front end (`index.html`,
`pages/language.html`, `js/`, `css/`) renders the tables on GitHub Pages.

The audit found and fixed **20 verified bugs**: 3 security issues (including a
GitHub token leaked into logs and a reflected DOM XSS), 1 data-corruption bug
(duplicated `Size` column in every published CSV and the README), 1 broken
homepage behavior (all content rendered twice), 1 failing unit test, and
several reliability/robustness issues. All fixes were validated (`cargo test`
3/3 pass — previously 1 failed, `node --check`, `py_compile`, and an end-to-end
regeneration of `data/processed/` + `README.md`).

Machine-readable versions: [`report.json`](report.json), [`report.csv`](report.csv).

## 2. Repository Map

```
index.html  pages/language.html   Static site (GitHub Pages)
css/style.css                     Theme + table styling
js/main.js                        Homepage: theme, nav, top-10 tables (Papa Parse + Sortable)
js/language-page.js               Per-language top-1000 table page
js/papaparse.min.js               Vendored CSV parser
js/sortable.min.js                Vendored HubSpot sortable.js 0.8.0
kstars/Cargo.toml  src/main.rs    Rust CLI: GitHub Search API -> CSV (with page cache)
kstars/main.py                    Cron pipeline: API load -> post-process -> README
kstars/Makefile  requirements.txt Build/run helpers; pandas + tabulate
data/original/                    Raw CLI output (gitignored)
data/processed/                   Published CSVs consumed by the site
README.md                         Generated fallback tables
```

Entry points: `index.html` (site), `kstars` binary via `main.py` (data
pipeline, intended for cron). No CI/CD workflows exist.

## 3. Findings (all fixed)

### Critical / Security

| ID | Severity | Component | Issue |
|----|----------|-----------|-------|
| BUG-01 | High | `kstars/src/main.rs` | `info!("Parsed arguments: {:?}", args)` printed the GitHub access token to logs/stdout on every run. Fixed by logging only non-sensitive fields. |
| BUG-02 | High | `js/language-page.js` | Reflected DOM XSS: the `?lang=` URL parameter was interpolated into `innerHTML` in two places. `?lang=<img src=x onerror=...>` executed arbitrary JS. Fixed by building DOM nodes with `textContent`. |
| BUG-03 | High | `kstars/main.py` | Token read via `$(cat access_token.txt)` inside a `shell=True` command: the secret appeared in the child process argv (visible in `ps`) and the string-interpolated command was an injection surface. Fixed by invoking the binary with an argv list and passing `GITHUB_TOKEN` via the environment (the Rust CLI already supports it). |

### Functional / Data

| ID | Severity | Component | Issue |
|----|----------|-----------|-------|
| BUG-04 | High | `js/main.js` | Duplicate initialization: nav-link creation and CSV loading existed both inside `DOMContentLoaded` and again at top level, so every nav entry and every language section was rendered twice and each CSV was fetched twice. `Sortable.init()` also fired while the second batch was still loading. Fixed by deleting the duplicated top-level block and a dead counter variable. |
| BUG-05 | High | `kstars/main.py` | `preprocess_data` appended `df["Size"]` at the end of the frame and then built `new_columns` from the *post-assignment* column list, so the new column was included twice → every processed CSV had a trailing duplicated `Size` column (70 files) and `README.md` rendered a `Size.1` column. Fixed with `df.insert()`/`drop()`; all CSVs and README regenerated. |
| BUG-06 | Medium | `kstars/src/main.rs` | Unit test `test_parse_languages_with_default_list` **failed**: default map used `"CSharp"`/`"CPP"` as display names while the test (and the site's semantics) expect `"C#"`/`"C++"`. Fixed the default list; suite is now 3/3 green. |
| BUG-07 | Medium | `kstars/src/main.rs` | `Prolog` missing from the default language list (35 languages in `main.py`/`main.js`, only 34 in Rust). Standalone runs silently skipped Prolog. |
| BUG-08 | Medium | `kstars/main.py` | README table-of-contents anchors were broken: links used raw display text (`#C++`, `#Vim-script`) while GitHub slugifies headings to lowercase, strips punctuation and de-duplicates (`c-1`, `c-2`, `vim-script`). Added a `github_slugger()` replica; TOC links now match the generated headings. |
| BUG-09 | Medium | `kstars/Makefile` | `deploy`/`schedule` targets invoked a Prefect flow (`main.py:run_kstars_flow`) that no longer exists; `prefect` isn't even in `requirements.txt`. Removed dead targets and made `run` depend on `build-crate`. |
| BUG-10 | Medium | `kstars/main.py` | `run_kstars_task` retried `while True:` on *any* non-zero exit — a deterministic failure (bad args, API error) would hang the cron job forever. Now bounded by `max_attempts` (default 24 ≈ 2 h) and then raises. |
| BUG-11 | Medium | `js/main.js` | `Papa.parse` had no `error` callback: a failed CSV download silently dropped the section *and* prevented `Sortable.init()` from ever running (counter could never reach `languages.length`) → all tables unsortable. Added an `error` handler that shows a message and completes the counter. |
| BUG-12 | Medium | `kstars/main.py` | `make run` called bare `kstars`, but nothing installed the binary on `PATH` → `FileNotFoundError`. Added `find_kstars_binary()` (PATH → `target/release/kstars` fallback) and made `make run` build the crate first. |

### Robustness / Edge cases

| ID | Severity | Component | Issue |
|----|----------|-----------|-------|
| BUG-13 | Low | `kstars/src/main.rs` | Output CSV name was derived from `display_name`, so `-l "CPP:C++"` produced `C++.csv`, breaking the site's expected `CPP.csv` name. Filename now derived from `api_name`. |
| BUG-14 | Low | `js/language-page.js` | `getDisplayName` lacked the `Vim-script → "Vim script"` mapping used elsewhere; the page title rendered "kstars Vim-script". |
| BUG-15 | Low | `kstars/src/main.rs` | `HeaderValue::from_str(...).expect(...)` panicked on a token containing non-header-legal characters → crash. Now returns a contextual `anyhow` error. |
| BUG-16 | Low | `kstars/src/main.rs`, `js/*` | Language names were interpolated unencoded into the GitHub query URL and the CSV path. Now uses reqwest `.query()` (percent-encoded) and `encodeURIComponent`. |
| BUG-17 | Low | `js/*`, `*.html` | `window.open(..., "_blank")` and `target="_blank"` links lacked `rel="noopener"` → reverse-tabnabbing exposure. Added `noopener noreferrer` everywhere. |
| BUG-18 | Low | `js/main.js`, `js/language-page.js` | `truncateStringAtWord` called `str.slice` on non-string input (e.g. numeric cells) → `TypeError`. Now coerces to `String`. |
| BUG-19 | Low | `kstars/main.py` | `df[col].apply(pd.to_datetime)` parsed dates element-wise and crashed the whole cron run on a single unparseable value. Now vectorized `pd.to_datetime(..., errors="coerce", utc=True)` with a warning count. |
| BUG-20 | Low | repo hygiene | 70 Syncthing `*.sync-conflict-*.csv` files were tracked in git and deployed to Pages (duplicated/conflicted data). Removed and gitignored via `*.sync-conflict-*`. |

### Code quality (also cleaned)

- Removed no-op `use serde_json;` and a duplicated doc comment in `main.rs`.
- Removed duplicate `csv` from `[dev-dependencies]` (already a normal dep).
- Removed unreachable `if "Size (KB)" in new_columns` check in `main.py`.

## 4. Verified but not changed (documented for follow-up)

| Item | Notes |
|------|-------|
| Hover-CSS for `.language-nav` (style.css:99-104) | Sets `opacity`/`pointer-events` but not `max-height`, so hover does nothing; the Menu button (`.nav-visible`) is the working mechanism. Left as-is to avoid changing UX; recommend deleting the dead rule or wiring it to also set `max-height`. |
| Cache has no TTL (`src/main.rs`) | Cached pages are only refreshed after a successful full run; a mid-run failure can leave stale pages. Acceptable for a weekly job; consider an mtime check. |
| Empty page-1 result | Fetches page 2 before stopping — one extra API call; cosmetic. |
| No CI / no lint config | Recommend adding a GitHub Actions job (`cargo test`, `node --check`, `py_compile`, data freshness check) and a `prettier`/`eslint` pass. |
| No CSP / SRI | GitHub Pages site; consider a `Content-Security-Policy` meta tag and SRI for the gtag script. |
| `access_token.txt` | Present locally (gitignored — correctly). Verify the token has minimal scopes (`public_repo` read is enough). |

## 5. Validation performed

- `cargo test` — **3/3 pass** (was 1 failing before the fix).
- `cargo build` / `cargo build --release` — clean, no warnings.
- `node --check js/main.js js/language-page.js` — OK.
- `python3 -m py_compile kstars/main.py` — OK.
- `run_post_processing` end-to-end over `data/original/` (35 languages):
  every processed CSV now has exactly one `Size` column (was duplicated),
  `README.md` contains no `Size.1`, and TOC anchors resolve to valid
  GitHub slugs (`#c`, `#c-1`, `#c-2`, `#vim-script`, ...).
- File-existence check: all 35 `top10_*.csv` and `*.csv` files referenced by
  the JS exist.

## 6. Recommended next steps

1. Re-deploy the site (push `data/processed/`, `README.md`, JS/HTML) and verify
   the homepage renders each language once.
2. Rotate the GitHub token — it has appeared in historical logs (BUG-01) and
   process argv (BUG-03).
3. Add a minimal CI workflow with the validation commands in §5.
4. Consider a unit test asserting the processed CSV header has no duplicates
   (regression guard for BUG-05).
