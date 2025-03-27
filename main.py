import subprocess
from prefect.logging import get_run_logger
from collections.abc import Iterable
from typing import TypeVar

from prefect import flow, task
from prefect.concurrency.sync import rate_limit

T = TypeVar("T")
logger = get_run_logger()

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


def chunk_iterable(iterable: Iterable[T], chunk_size: int) -> list[list[T]]:
    """Convert iterable items to chunks of specified size."""
    items = list(iterable)
    return [items[i : i + chunk_size] for i in range(0, len(items), chunk_size)]


@task(tags=["kstars-api"])
def run_kstars(language: str, lang_name: str):
    rate_limit("rate-limited-gh-api")
    command = f"kstars -t $(cat access_token.txt) -l {language}:{lang_name}"
    print(f"Running command: {command}")
    try:
        result = subprocess.run(
            command,
            shell=True,
            capture_output=True,
            text=True,
            check=True
        )
        logger.info(result.stdout)
        logger.error(result.stderr)

    except subprocess.CalledProcessError as e:
        logger.error(f"Error running command: {e}\n{e.stdout}\n{e.stderr}")
    except FileNotFoundError:
        logger.error(f"Error: The command '{command.split()[0]}' was not found.")
    except Exception as e:
        logger.error(f"An unexpected error occurred: {e}")

    return lang_name


@flow(log_prints=True)
def run_kstars_flow(languages: dict[str, str]):
    for lang, lang_name in languages.items():
        _ = run_kstars.submit(lang, lang_name)


if __name__ == "__main__":
    _ = run_kstars_flow.serve(
        name="kstars-load-api",
        parameters={"languages": LANGUAGES},
        # work_pool_name="local",
        cron="0 1 * * 5",
    )
