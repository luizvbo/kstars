from collections.abc import Iterable
from typing import  Any, TypeVar
import os

from prefect import flow, task

T = TypeVar('T')

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
    return [items[i:i + chunk_size] for i in range(0, len(items), chunk_size)]

@task
def run_kstars(language: str, lang_name: str):
    command = f"kstars -t $(cat access_token.txt) -l {language}:{lang_name}"
    print(f"Running command: {command}")
    _ = os.system(command)
    return lang_name


@flow(log_prints=True)
def run_kstars_flow():
    wait_for  = []
    for lang_trio in chunk_iterable(LANGUAGES.items(), 3):
        wait_for = [run_kstars.submit(lang, lang_name, wait_for=wait_for) for lang, lang_name in lang_trio]


if __name__ == "__main__":
    run_kstars_flow()
