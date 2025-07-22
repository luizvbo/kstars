#!/bin/bash
# Define the list of languages
languages=(
    "actionscript"
    "c"
    "c++"
    "css"
    "c#"
    "clojure"
    "coffeescript"
    "dm"
    "dart"
    "elixir"
    "go"
    "groovy"
    "html"
    "haskell"
    "java"
    "javascript"
    "julia"
    "kotlin"
    "lua"
    "matlab"
    "objective-c"
    "php"
    "perl"
    "powershell"
    "python"
    "r"
    "ruby"
    "rust"
    "scala"
    "shell"
    "swift"
    "tex"
    "typescript"
    "vim script"
)

mkdir -p /tmp/kstars

for lang in "${languages[@]}"; do
    fs_friendly_lang=""

    case "$lang" in
        "c++")
            fs_friendly_lang="cpp"
            ;;
        "c#")
            fs_friendly_lang="csharp"
            ;;
        "objective-c")
            fs_friendly_lang="objective_c"
            ;;
        "vim script")
            fs_friendly_lang="vim_script"
            ;;
        *)
            # Default conversion for other languages
            fs_friendly_lang=$(echo "$lang" | sed -r 's/[^a-zA-Z0-9]+/_/g' | tr '[:upper:]' '[:lower:]')
            ;;
    esac

    echo "Searching for repositories in language: $lang (Filesystem-friendly: $fs_friendly_lang)"

    gh search repos --language "$lang" -L 1000 --sort stars --json \
        createdAt,description,forksCount,fullName,homepage,isArchived,license,name,openIssuesCount,pushedAt,size,stargazersCount,updatedAt,url,watchersCount > "/tmp/kstars/$fs_friendly_lang.json"

    if [ $? -eq 0 ]; then
        echo "Successfully saved data for $lang to /tmp/kstars/$fs_friendly_lang.json"
    else
        echo "Error searching for $lang. Please check your gh CLI configuration and internet connection."
    fi
    echo "--------------------------------------------------"
done

echo "All language searches completed."
