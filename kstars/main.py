import logging
import subprocess
import sys
import time
from pathlib import Path

import pandas as pd

# --- Configuration ---
# Setup logging for Cron (prints to stdout/stderr)
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)
logger = logging.getLogger(__name__)

# Paths relative to the script location to ensure Cron finds them
BASE_DIR = Path(__file__).parent.resolve()
DATA_FOLDER = BASE_DIR.parent / "data"
README_PATH = BASE_DIR.parent / "README.md"
HOME_PAGE = "https://luizvbo.github.io/kstars"

LANGUAGES = {
    "ActionScript": "ActionScript",
    "C": "C",
    "CPP": "C++",
    "CSS": "CSS",
    "CSharp": "C#",
    "Clojure": "Clojure",
    "CoffeeScript": "CoffeeScript",
    "DM": "DM",
    "Dart": "Dart",
    "Elixir": "Elixir",
    "Go": "Go",
    "Groovy": "Groovy",
    "HTML": "HTML",
    "Haskell": "Haskell",
    "Java": "Java",
    "JavaScript": "JavaScript",
    "Julia": "Julia",
    "Kotlin": "Kotlin",
    "Lua": "Lua",
    "MATLAB": "MATLAB",
    "Objective-C": "Objective-C",
    "PHP": "PHP",
    "Perl": "Perl",
    "PowerShell": "PowerShell",
    "Prolog": "Prolog",
    "Python": "Python",
    "R": "R",
    "Ruby": "Ruby",
    "Rust": "Rust",
    "Scala": "Scala",
    "Shell": "Shell",
    "Swift": "Swift",
    "TeX": "TeX",
    "TypeScript": "TypeScript",
    "Vim-script": "Vim script",
}


def human_readable_size(size_kb: int) -> str:
    """Converts file size in KB to a human-readable format."""
    if size_kb < 1024:
        return f"{size_kb:.2f} KB"
    elif size_kb < (1024 * 1024):
        size_mb = size_kb / 1024
        return f"{size_mb:.2f} MB"
    elif size_kb < (1024 * 1024 * 1024):
        size_gb = size_kb / (1024 * 1024)
        return f"{size_gb:.2f} GB"
    else:
        size_tb = size_kb / (1024 * 1024 * 1024)
        return f"{size_tb:.2f} TB"


def generate_readme(
    languages: dict[str, str], lang_folder: str | Path, readme_path: Path | str
):
    """Generates the README.md file from CSV data."""
    logger.info("Generating README.md...")
    
    content = """# kstars: Top Starred GitHub Repos per Language

This project lists the top 1000 most starred repositories on GitHub for a variety of popular programming languages.

The [kstars page](https://luizvbo.github.io/kstars/) displays the top 10 repositories for each language on the homepage. Each language section also includes a link to a dedicated page where you can explore the top 1000 repositories for that specific language.

Below, you'll find a fallback representation of the top 10 repositories for each language, directly from the data used on the website.

## Top 10 Repositories

"""
    for lang_safe, lang_display in languages.items():
        content += f"1. [{lang_display}](#{lang_display.replace(' ', '-')})\n"
    content += "\n"
    
    lang_folder = Path(lang_folder) if isinstance(lang_folder, str) else lang_folder
    
    for lang_safe, lang_display in languages.items():
        path_csv_file = lang_folder / f"top10_{lang_safe}.csv"
        url_1k = f"{HOME_PAGE}/pages/language.html?lang={lang_safe}"

        try:
            df = pd.read_csv(path_csv_file)
            content += (
                f"### {lang_display}\n[Full list with 1000 most starred repos...]({url_1k}) "
                f"\n\n{df.to_markdown(index=False)}\n"
            )
        except Exception as e:
            logger.error(f"Error processing CSV file for {lang_display}: {e}")
            raise e

    # Write the content to README.md
    try:
        with open(readme_path, "w", encoding="utf-8") as readme_file:
            readme_file.write(content)
        logger.info("README.md file generated successfully!")
    except Exception as e:
        logger.error(f"Failed to write README file: {e}")
        raise e


def preprocess_data(lang_name: str, input_folder: Path, output_folder: Path):
    fname = f"{lang_name}.csv"
    input_file_path = Path(input_folder) / fname
    output_file_path = Path(output_folder) / fname
    output_top10_file_path = Path(output_folder) / f"top10_{fname}"

    try:
        df: pd.DataFrame = pd.read_csv(input_file_path)
        for col in ("Last Commit", "Created At"):
            if col in df.columns:
                df[col] = df[col].apply(pd.to_datetime).dt.strftime("%d/%m/%Y")
        
        if "Size (KB)" in df.columns:
            df["Size"] = df["Size (KB)"].apply(human_readable_size)
            # Reorder columns to put Size where Size (KB) was
            cols = df.columns.tolist()
            idx = cols.index("Size (KB)")
            new_columns = cols[:idx] + ["Size"] + cols[idx+1:]
            # Remove Size (KB) from list if it's still there (it is in cols, not new_columns)
            if "Size (KB)" in new_columns: 
                new_columns.remove("Size (KB)")
            
            df = df[new_columns]

        df.to_csv(output_file_path, index=False)
        df.head(10).to_csv(output_top10_file_path, index=False)
        logger.info(
            f"Stored processed files for {lang_name}"
        )

    except FileNotFoundError:
        logger.error(f"Error: Input file not found at '{input_file_path}'")
        raise
    except Exception as e:
        logger.error(f"Error processing data for {lang_name}: {e}")
        raise


def run_kstars_task(
    language: str, lang_name: str, output_folder: str | Path
) -> None:
    """
    Runs the kstars command. Retries indefinitely if it fails (e.g. API limits).
    """
    # Ensure output folder exists
    Path(output_folder).mkdir(parents=True, exist_ok=True)
    
    # Construct command
    # Note: Assuming .access_token.txt is in the same dir as the script
    token_path = BASE_DIR / "access_token.txt"
    
    if not token_path.exists():
        logger.error(f"Access token not found at {token_path}")
        raise FileNotFoundError("Access token file missing")

    command = f'kstars -t $(cat "{token_path}") -l "{language}:{lang_name}" -o "{output_folder}"'
    
    attempt = 1
    wait_time_seconds = 300  # 5 minutes wait time for API reset

    while True:
        logger.info(f"Running kstars for {language} (Attempt {attempt})...")
        try:
            result = subprocess.run(
                command, shell=True, capture_output=True, text=True, check=True
            )
            # Log stdout if needed, or just success
            if result.stdout:
                logger.info(result.stdout)

            if result.stderr: # Also good to log any stderr on success, if present
                logger.warning(f"kstars produced stderr (though successful): {result.stderr}")
            logger.info(f"Successfully loaded data for {language}")
            break  # Success, exit loop

        except subprocess.CalledProcessError as e:
            logger.warning(f"Failed to run kstars for {language}.")
            logger.warning(f"STDERR: {e.stderr}")
            logger.warning(f"Waiting {wait_time_seconds} seconds before retrying...")
            time.sleep(wait_time_seconds)
            attempt += 1
        except Exception as e:
            logger.error(f"An unexpected error occurred: {e}")
            raise e


def run_load_api(languages: dict[str, str], output_folder: str | Path):
    logger.info("Starting API Load process...")
    path_data_original = Path(output_folder) / "original"
    
    for lang in languages.keys():
        run_kstars_task(lang, lang, path_data_original)
    
    logger.info("API Load process completed successfully.")


def run_post_processing(languages: dict[str, str], output_folder: str | Path):
    logger.info("Starting Post Processing...")
    path_data_original = Path(output_folder) / "original"
    path_data_processed = Path(output_folder) / "processed"
    path_data_processed.mkdir(parents=True, exist_ok=True)

    for lang_name in languages.keys():
        preprocess_data(lang_name, path_data_original, path_data_processed)

    generate_readme(LANGUAGES, path_data_processed, README_PATH)
    logger.info("Post Processing completed successfully.")


if __name__ == "__main__":
    try:
        logger.info("Starting Cron Job...")
        
        # 1. Run Load API (Must succeed fully before moving on)
        run_load_api(LANGUAGES, DATA_FOLDER)
        
        # 2. Run Post Processing
        run_post_processing(LANGUAGES, DATA_FOLDER)
        
        logger.info("Cron Job finished successfully.")
        sys.exit(0)
        
    except Exception as e:
        logger.critical(f"Cron Job failed with error: {e}")
        sys.exit(1)
