#!/bin/bash

languages=(
    "actionscript" "c" "c++" "css" "c#" "clojure" "coffeescript" "dm" "dart"
    "elixir" "go" "groovy" "html" "haskell" "java" "javascript" "julia"
    "kotlin" "lua" "matlab" "objective-c" "php" "perl" "powershell" "python"
    "r" "ruby" "rust" "scala" "shell" "swift" "tex" "typescript" "vim script"
)

OUTPUT_DIR="/tmp/kstars"
REPO_LIMIT=1000
DELAY_SECONDS=30
FETCH_PERFORMED=0

mkdir -p "$OUTPUT_DIR"

# Converts a language name to a filesystem-friendly name
get_fs_friendly_lang() {
    case "$1" in
        "c++")
            echo "cpp"
            ;;
        "c#")
            echo "csharp"
            ;;
        "objective-c")
            echo "objective_c"
            ;;
        "vim script")
            echo "vim_script"
            ;;
        *)
            echo "$1" | sed -r 's/[^a-zA-Z0-9]+/_/g' | tr '[:upper:]' '[:lower:]'
            ;;
    esac
}

# Fetches repositories for a given language
fetch_repos() {
    local lang="$1"
    local fs_friendly_lang
    fs_friendly_lang=$(get_fs_friendly_lang "$lang")
    local output_file="$OUTPUT_DIR/$fs_friendly_lang.json"
    local start_time
    start_time=$(date +%s)

    echo "Processing language: $lang (Filesystem-friendly: $fs_friendly_lang)"

    # Check if the file already exists
    if [ -f "$output_file" ]; then
        echo "Data for $lang already exists. Skipping."
        # Since we are skipping, no fetch was performed.
        # Ensure the next iteration does NOT sleep.
        FETCH_PERFORMED=0
        return
    fi

    # If a fetch was done in the previous step, wait to avoid rate limits.
    if [ "$FETCH_PERFORMED" -eq 1 ]; then
        echo "Waiting for ${DELAY_SECONDS}s before the next request..."
        sleep "$DELAY_SECONDS"
    fi

    echo "Searching for the top $REPO_LIMIT starred repositories..."

    # Execute the search query
    gh search repos --language "$lang" -L "$REPO_LIMIT" --sort stars --json \
        createdAt,description,forksCount,fullName,homepage,isArchived,license,name,openIssuesCount,pushedAt,size,stargazersCount,updatedAt,url,watchersCount > "$output_file"

    # Verify that the command was successful
    if [ $? -eq 0 ]; then
        local end_time
        end_time=$(date +%s)
        local time_spent=$((end_time - start_time))
        echo "Successfully saved data for $lang to $output_file. Time spent: ${time_spent}s."
        # Record that a fetch was successfully performed.
        FETCH_PERFORMED=1
    else
        echo "Error searching for $lang. Please check your gh CLI configuration and internet connection."
        # Clean up the potentially incomplete file
        rm -f "$output_file"
        # Record that the fetch failed, so we don't sleep on the next iteration.
        FETCH_PERFORMED=0
    fi
}

main() {
    local total_start_time
    total_start_time=$(date +%s)

    for lang in "${languages[@]}"; do
        fetch_repos "$lang"
        echo "--------------------------------------------------"
    done

    local total_end_time
    total_end_time=$(date +%s)
    local total_time_spent=$((total_end_time - total_start_time))

    echo "All language searches completed. Total time spent: ${total_time_spent}s."
}

main
