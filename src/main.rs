// src/main.rs
use clap::{Parser, Subcommand};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "gcd")]
#[command(about = "Git repository quick navigation tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pattern to match repository name (when no subcommand is provided)
    pattern: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Index repositories in the specified directory
    Index {
        /// Directory to scan for git repositories
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Install shell integration
    Install {
        /// Shell to install for (bash, zsh, fish, ps)
        #[arg(default_value = "bash")]
        shell: String,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    repos: HashMap<String, PathBuf>,
}

impl Config {
    fn load() -> Self {
        let config_path = config_path();
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Config {
                repos: HashMap::new(),
            }
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let config_path = config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, contents)
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("gcd");
    path.push("config.json");
    path
}

fn find_git_repos(path: &Path) -> Vec<PathBuf> {
    let mut repos = Vec::new();
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            e.file_name() != ".git"
                && e.file_name() != "node_modules"
                && e.file_name() != "target"
        })
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if entry.file_type().is_dir() && entry.path().join(".git").is_dir() {
            repos.push(entry.path().to_path_buf());
        }
    }
    repos
}

fn install_shell_integration(shell: &str) -> std::io::Result<()> {
    let script = if shell == "ps" {
        // Handle PowerShell specifically
        let profile_path = if let Ok(output) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "echo $PROFILE"])
            .output()
        {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                PathBuf::from(path_str)
            } else {
                let docs = std::env::var("USERPROFILE")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| dirs::home_dir().expect("Could not find home directory"));
                docs.join("Documents").join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1")
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get PowerShell profile path",
            ));
        };

        // Ensure the directory for the profile path exists
        if let Some(parent) = profile_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        (profile_path, POWERSHELL_INTEGRATION)
    } else {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        match shell {
            "bash" => {
                let script_path = home_dir.join(".bashrc");
                (script_path, BASH_INTEGRATION)
            }
            "zsh" => {
                let script_path = home_dir.join(".zshrc");
                (script_path, ZSH_INTEGRATION)
            }
            "fish" => {
                let mut script_path = home_dir;
                script_path.push(".config");
                script_path.push("fish");
                script_path.push("config.fish");
                (script_path, FISH_INTEGRATION)
            }
            _ => panic!("Unsupported shell"),
        }
    };

    let mut content = std::fs::read_to_string(&script.0).unwrap_or_default();
    if !content.contains("### GCD Integration") {
        content.push_str("\n### GCD Integration\n");
        content.push_str(script.1);
        std::fs::write(script.0, content)?;
    }
    Ok(())
}



const BASH_INTEGRATION: &str = r#"
gcd() {
    if [ "$#" -eq 0 ]; then
        command gcd
    else
        local output
        output=$(command gcd "$@")
        if [ $? -eq 0 ]; then
            cd "$output" || return 1
        else
            echo "$output"
            return 1
        fi
    fi
}
"#;

const ZSH_INTEGRATION: &str = BASH_INTEGRATION;

const FISH_INTEGRATION: &str = r#"
function gcd
    if test (count $argv) -eq 0
        command gcd
    else
        set -l output (command gcd $argv)
        if test $status -eq 0
            cd $output
        else
            echo $output
            return 1
        end
    end
end
"#;

const POWERSHELL_INTEGRATION: &str = r#"
function gcd {
    if ($args.Count -eq 0) {
        & gcd.exe
    } else {
        $output = & gcd.exe $args
        if ($LASTEXITCODE -eq 0) {
            Set-Location $output
        } else {
            Write-Host $output
            return $LASTEXITCODE
        }
    }
}
"#;

fn main() {
    let cli = Cli::parse();
    let mut config = Config::load();

    match cli.command {
        Some(Commands::Index { path }) => {
            let path = path.canonicalize().expect("Invalid path");
            let repos = find_git_repos(&path);
            for repo in repos {
                let name = repo
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                config.repos.insert(name, repo);
            }
            config.save().expect("Failed to save config");
            println!("Indexed repositories successfully");
        }
        Some(Commands::Install { shell }) => {
            install_shell_integration(&shell).expect("Failed to install shell integration");
            println!("Shell integration installed for {}", shell);
        }
        None => {
            if let Some(pattern) = cli.pattern {
                let matcher = SkimMatcherV2::default();
                let mut matches: Vec<_> = config
                    .repos
                    .iter()
                    .filter_map(|(name, path)| {
                        matcher
                            .fuzzy_match(name, &pattern)
                            .map(|score| (score, name, path))
                    })
                    .collect();

                matches.sort_by(|a, b| b.0.cmp(&a.0));

                if let Some((_, _, path)) = matches.first() {
                    println!("{}", path.display());
                } else {
                    eprintln!("No matching repository found");
                    std::process::exit(1);
                }
            } else {
                println!("Available repositories:");
                for (name, path) in config.repos {
                    println!("{}: {}", name, path.display());
                }
            }
        }
    }
}