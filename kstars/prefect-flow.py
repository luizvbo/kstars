import subprocess
from datetime import timedelta
from pathlib import Path
from typing import TypeVar

import pandas as pd
from prefect import State, flow, serve, task
from prefect.cache_policies import DEFAULT
from prefect.concurrency.sync import rate_limit
from prefect.logging import get_run_logger

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
CACHE_POLICY = DEFAULT


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


@task(
    tags=["kstars-data-processing"],
    retries=5,
    retry_delay_seconds=10,
)
def preprocess_data(lang_name: str, input_folder: Path, output_folder: Path):
    logger = get_run_logger()
    fname = f"{lang_name}.csv"
    input_file_path = Path(input_folder) / fname
    output_file_path = Path(output_folder) / fname
    output_top10_file_path = Path(output_folder) / f"top10_{fname}"

    try:
        df: pd.DataFrame = pd.read_csv(input_file_path)
        for col in ("Last Commit", "Created At"):
            df[col] = df[col].apply(pd.to_datetime).dt.strftime("%d/%m/%Y")
        df["Size"] = df["Size (KB)"].apply(human_readable_size)
        new_columns = df.columns.drop("Size").tolist()
        new_columns.insert(new_columns.index("Size (KB)"), "Size")
        df = df[new_columns]
        df.to_csv(output_file_path, index=False)
        df.head(10).to_csv(output_top10_file_path, index=False)
        logger.info(
            f"Stored processed files to '{output_file_path}' and '{output_top10_file_path}'"
        )

    except FileNotFoundError as e:
        logger.error(f"Error: Input file not found at '{input_file_path}'")
        raise e
    except KeyError as e:
        logger.error(f"Error: Column not found in CSV '{input_file_path}': {e}")
        raise e

    except ValueError as e:
        logger.error(f"Error processing data in '{input_file_path}': {e}")
        raise e
    except PermissionError as e:
        logger.error(f"Error: Permission denied while writing to '{output_folder}'")
        raise e
    except Exception as e:
        logger.error(f"An unexpected error occurred: {e}")
        raise e


@task(
    tags=["kstars-api"],
    retries=10,
    retry_delay_seconds=30,
    cache_policy=CACHE_POLICY,
    cache_expiration=timedelta(days=1),
)
def run_kstars_task(
    language: str, lang_name: str, output_folder: str | Path
) -> None | State:
    logger = get_run_logger()
    command = f'kstars -t $(cat access_token.txt) -l "{language}:{lang_name}" -o {output_folder}'
    print(f"Running command: {command}")
    try:
        result = subprocess.run(
            command, shell=True, capture_output=True, text=True, check=True
        )
        logger.info(result.stdout)
        logger.error(result.stderr)

    except subprocess.CalledProcessError as e:
        logger.error(f"Error running command: {e}\n{e.stdout}\n{e.stderr}")
        raise e
    except FileNotFoundError as e:
        logger.error(f"Error: The command '{command.split()[0]}' was not found.")
        raise e
    except Exception as e:
        logger.error(f"An unexpected error occurred: {e}")
        raise e


@flow(log_prints=True)
def run_load_api(languages: dict[str, str], output_folder: str):
    path_data_original = Path(output_folder) / "original"
    path_data_original.mkdir(parents=True, exist_ok=True)

    for lang, lang_name in languages.items():
        rate_limit("rate-limited-gh-api")
        _ = run_kstars_task(lang, lang_name, path_data_original)


@flow(log_prints=True)
def run_post_processing(languages: dict[str, str], output_folder: str):
    path_data_original = Path(output_folder) / "original"
    path_data_processed = Path(output_folder) / "processed"
    path_data_processed.mkdir(parents=True, exist_ok=True)
    for _, lang_name in languages.items():
        _ = preprocess_data(lang_name, path_data_original, path_data_processed)


if __name__ == "__main__":
    flow_run_kstars = run_load_api.to_deployment(
        name="kstars-load-api",
        parameters={"languages": LANGUAGES, "output_folder": OUTPUT_FOLDER},
        cron="0 1 * * 5",
    )
    flow_run_post_processing = run_post_processing.to_deployment(
        name="kstars-data-processing",
        parameters={"languages": LANGUAGES, "output_folder": OUTPUT_FOLDER},
        cron="0 3 * * 5",
    )
    serve(flow_run_kstars, flow_run_post_processing)
