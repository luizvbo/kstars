use crate::schema::Repo;

fn parse_repos(csv_content: &str) -> Vec<Repo> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    reader.deserialize().filter_map(Result::ok).collect()
}

pub fn get_repo_data(file_name: &str) -> Vec<Repo> {
    let csv_content = match file_name {
        "ActionScript.csv" => include_str!("../../../../data/processed/ActionScript.csv"),
        "C.csv" => include_str!("../../../../data/processed/C.csv"),
        "Clojure.csv" => include_str!("../../../../data/processed/Clojure.csv"),
        "CoffeeScript.csv" => include_str!("../../../../data/processed/CoffeeScript.csv"),
        "CPP.csv" => include_str!("../../../../data/processed/CPP.csv"),
        "CSharp.csv" => include_str!("../../../../data/processed/CSharp.csv"),
        "CSS.csv" => include_str!("../../../../data/processed/CSS.csv"),
        "Dart.csv" => include_str!("../../../../data/processed/Dart.csv"),
        "DM.csv" => include_str!("../../../../data/processed/DM.csv"),
        "Elixir.csv" => include_str!("../../../../data/processed/Elixir.csv"),
        "Go.csv" => include_str!("../../../../data/processed/Go.csv"),
        "Groovy.csv" => include_str!("../../../../data/processed/Groovy.csv"),
        "Haskell.csv" => include_str!("../../../../data/processed/Haskell.csv"),
        "HTML.csv" => include_str!("../../../../data/processed/HTML.csv"),
        "Java.csv" => include_str!("../../../../data/processed/Java.csv"),
        "JavaScript.csv" => include_str!("../../../../data/processed/JavaScript.csv"),
        "Julia.csv" => include_str!("../../../../data/processed/Julia.csv"),
        "Kotlin.csv" => include_str!("../../../../data/processed/Kotlin.csv"),
        "Lua.csv" => include_str!("../../../../data/processed/Lua.csv"),
        "MATLAB.csv" => include_str!("../../../../data/processed/MATLAB.csv"),
        "Objective-C.csv" => include_str!("../../../../data/processed/Objective-C.csv"),
        "Perl.csv" => include_str!("../../../../data/processed/Perl.csv"),
        "PHP.csv" => include_str!("../../../../data/processed/PHP.csv"),
        "PowerShell.csv" => include_str!("../../../../data/processed/PowerShell.csv"),
        "Prolog.csv" => include_str!("../../../../data/processed/Prolog.csv"),
        "Python.csv" => include_str!("../../../../data/processed/Python.csv"),
        "R.csv" => include_str!("../../../../data/processed/R.csv"),
        "Ruby.csv" => include_str!("../../../../data/processed/Ruby.csv"),
        "Rust.csv" => include_str!("../../../../data/processed/Rust.csv"),
        "Scala.csv" => include_str!("../../../../data/processed/Scala.csv"),
        "Shell.csv" => include_str!("../../../../data/processed/Shell.csv"),
        "Swift.csv" => include_str!("../../../../data/processed/Swift.csv"),
        "TeX.csv" => include_str!("../../../../data/processed/TeX.csv"),
        "TypeScript.csv" => include_str!("../../../../data/processed/TypeScript.csv"),
        "Vim-script.csv" => include_str!("../../../../data/processed/Vim-script.csv"),

        "top10_ActionScript.csv" => {
            include_str!("../../../../data/processed/top10_ActionScript.csv")
        }
        "top10_C.csv" => include_str!("../../../../data/processed/top10_C.csv"),
        "top10_Clojure.csv" => include_str!("../../../../data/processed/top10_Clojure.csv"),
        "top10_CoffeeScript.csv" => {
            include_str!("../../../../data/processed/top10_CoffeeScript.csv")
        }
        "top10_CPP.csv" => include_str!("../../../../data/processed/top10_CPP.csv"),
        "top10_CSharp.csv" => include_str!("../../../../data/processed/top10_CSharp.csv"),
        "top10_CSS.csv" => include_str!("../../../../data/processed/top10_CSS.csv"),
        "top10_Dart.csv" => include_str!("../../../../data/processed/top10_Dart.csv"),
        "top10_DM.csv" => include_str!("../../../../data/processed/top10_DM.csv"),
        "top10_Elixir.csv" => include_str!("../../../../data/processed/top10_Elixir.csv"),
        "top10_Go.csv" => include_str!("../../../../data/processed/top10_Go.csv"),
        "top10_Groovy.csv" => include_str!("../../../../data/processed/top10_Groovy.csv"),
        "top10_Haskell.csv" => include_str!("../../../../data/processed/top10_Haskell.csv"),
        "top10_HTML.csv" => include_str!("../../../../data/processed/top10_HTML.csv"),
        "top10_Java.csv" => include_str!("../../../../data/processed/top10_Java.csv"),
        "top10_JavaScript.csv" => include_str!("../../../../data/processed/top10_JavaScript.csv"),
        "top10_Julia.csv" => include_str!("../../../../data/processed/top10_Julia.csv"),
        "top10_Kotlin.csv" => include_str!("../../../../data/processed/top10_Kotlin.csv"),
        "top10_Lua.csv" => include_str!("../../../../data/processed/top10_Lua.csv"),
        "top10_MATLAB.csv" => include_str!("../../../../data/processed/top10_MATLAB.csv"),
        "top10_Objective-C.csv" => include_str!("../../../../data/processed/top10_Objective-C.csv"),
        "top10_Perl.csv" => include_str!("../../../../data/processed/top10_Perl.csv"),
        "top10_PHP.csv" => include_str!("../../../../data/processed/top10_PHP.csv"),
        "top10_PowerShell.csv" => include_str!("../../../../data/processed/top10_PowerShell.csv"),
        "top10_Prolog.csv" => include_str!("../../../../data/processed/top10_Prolog.csv"),
        "top10_Python.csv" => include_str!("../../../../data/processed/top10_Python.csv"),
        "top10_R.csv" => include_str!("../../../../data/processed/top10_R.csv"),
        "top10_Ruby.csv" => include_str!("../../../../data/processed/top10_Ruby.csv"),
        "top10_Rust.csv" => include_str!("../../../../data/processed/top10_Rust.csv"),
        "top10_Scala.csv" => include_str!("../../../../data/processed/top10_Scala.csv"),
        "top10_Shell.csv" => include_str!("../../../../data/processed/top10_Shell.csv"),
        "top10_Swift.csv" => include_str!("../../../../data/processed/top10_Swift.csv"),
        "top10_TeX.csv" => include_str!("../../../../data/processed/top10_TeX.csv"),
        "top10_TypeScript.csv" => include_str!("../../../../data/processed/top10_TypeScript.csv"),
        "top10_Vim-script.csv" => include_str!("../../../../data/processed/top10_Vim-script.csv"),
        _ => "",
    };

    parse_repos(csv_content)
}
