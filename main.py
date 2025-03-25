import os
from collections.abc import Iterable
from typing import TypeVar

from prefect import flow, task
from prefect.client import get_client
from prefect.concurrency.sync import rate_limit

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


def chunk_iterable(iterable: Iterable[T], chunk_size: int) -> list[list[T]]:
    """Convert iterable items to chunks of specified size."""
    items = list(iterable)
    return [items[i : i + chunk_size] for i in range(0, len(items), chunk_size)]


async def setup_rate_limit():
    async with get_client() as client:
        await client.create_concurrency_limit(
            tag="kstars-api", concurrency_limit=3, slot_decay_per_second=0.1
        )


@task
def run_kstars(language: str, lang_name: str):
    rate_limit("kstars-api")
    command = f"kstars -t $(cat access_token.txt) -l {language}:{lang_name}"
    print(f"Running command: {command}")
    _ = os.system(command)
    return lang_name


@flow(log_prints=True)
def run_kstars_flow():
    for lang, lang_name in LANGUAGES.items():
        _ = run_kstars.submit(lang, lang_name)


if __name__ == "__main__":
    run_kstars_flow()
