import subprocess
from pathlib import Path
from typing import TypeVar

import pandas as pd
from prefect import flow, task
from prefect.concurrency.sync import rate_limit
from prefect.logging import get_run_logger
from prefect.states import Failed

T = TypeVar("T")

LANGUAGES = {
    "ActionScript": "ActionScript",
    "C": "C",
    "CSharp": "C#",
    "CPP": "C++",
    "Clojure": "Clojure",
    "CoffeeScript": "CoffeeScript",
    "CSS": "CSS",
    "Dart": "Dart",
    "DM": "DM",
    "Elixir": "Elixir",
    "Go": "Go",
    "Groovy": "Groovy",
    "Haskell": "Haskell",
    "HTML": "HTML",
    "Java": "Java",
    "JavaScript": "JavaScript",
    "Julia": "Julia",
    "Kotlin": "Kotlin",
    "Lua": "Lua",
    "MATLAB": "MATLAB",
    "Objective-C": "Objective-C",
    "Perl": "Perl",
    "PHP": "PHP",
    "PowerShell": "PowerShell",
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
OUTPUT_FOLDER = "../data"


def human_readable_size(size_kb):
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


@task(tags=["kstars-data-processing"])
def preprocess_data(lang_name: str,input_folder: Path, output_folder: Path):
    logger = get_run_logger()
    fname = f"{lang_name}.csv"
    df: pd.DataFrame = pd.read_csv(Path(input_folder) / fname)
    for col in ("Last Commit", "Created At"):
        df[col] = df[col].apply(pd.to_datetime).dt.date
    for col in ("Stars", "Forks", "Watchers", "Open Issues"):
        df[col] = df[col].apply("{:,}".format)
    df["Size (KB)"] = df["Size (KB)"].apply(human_readable_size)
    fname_out = Path(output_folder) / fname
    fname_out_top10 = Path(output_folder) / f"top10_{fname}"
    df.to_csv(fname_out, index=False)
    df.head(10).to_csv(fname_out_top10, index=False)
    logger.info(f"Stored processed files to '{fname_out}' and '{fname_out_top10}'")

@task(tags=["kstars-api"])
def run_kstars(language: str, lang_name: str, output_folder: str | Path):
    logger = get_run_logger()
    command = f"kstars -t $(cat access_token.txt) -l {language}:{lang_name} -o {output_folder}"
    print(f"Running command: {command}")
    try:
        result = subprocess.run(
            command, shell=True, capture_output=True, text=True, check=True
        )
        logger.info(result.stdout)
        logger.error(result.stderr)

    except subprocess.CalledProcessError as e:
        return Failed(message=f"Error running command: {e}\n{e.stdout}\n{e.stderr}")
    except FileNotFoundError:
        return Failed(
            message=f"Error: The command '{command.split()[0]}' was not found."
        )
    except Exception as e:
        return Failed(message=f"An unexpected error occurred: {e}")

    return lang_name


@flow(log_prints=True)
def run_kstars_flow(languages: dict[str, str], output_folder: str):
    path_data_original = Path(output_folder) / "original"
    path_data_processed = Path(output_folder) / "processed"
    # Create the folders if they still don't exist
    path_data_original.mkdir(parents=True, exist_ok=True)
    path_data_processed.mkdir(parents=True, exist_ok=True)
    for lang, lang_name in languages.items():
        rate_limit("rate-limited-gh-api")
        _ = run_kstars(lang, lang_name, path_data_original)
        preprocess_data(
            lang_name, path_data_original, path_data_processed
        )


if __name__ == "__main__":
    _ = run_kstars_flow.serve(
        name="kstars-load-api",
        parameters={"languages": LANGUAGES, "output_folder": OUTPUT_FOLDER},
        cron="0 1 * * 5",
    )
