use std::{path::PathBuf, thread, time::Duration};

use clap::{Parser, Subcommand};
use console::style;
use fyg::{toml::FygToml, Fyg};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Parser)]
#[command(name = "fyg")]
#[command(version, author)]
#[command(about = "A build tool for Kotlin based projects", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    New {
        name: String,

        #[arg(short, long, default_value = "com.example")]
        group: String,

        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    Init {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long, default_value = "com.example")]
        group: String,
    },

    Build {
        #[arg(short, long)]
        release: bool,

        #[arg(short, long)]
        target: Option<String>,
    },

    Clean,

    Run {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    Test {
        #[arg(short, long)]
        filter: Option<String>,
    },

    Info,

}

fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::New { name, group, path } => {
            let base_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
            let project_path = base_path.join(&name);
            let fyg_toml = FygToml::new(&name, &group);

            println!();
            println!(
                "Creating new project {}",
                style(&name).cyan().bold()
            );
            println!(
                "   {} {}",
                style("Location:").dim(),
                style(project_path.display()).dim()
            );
            println!();

            let spinner = create_spinner("Creating project directory...");
            spinner.finish_and_clear();
            println!(
                "   {} Creating project directory",
                style("✓").green().bold()
            );

            let spinner = create_spinner("Generating fyg.toml...");
            Fyg::new(&base_path, fyg_toml)?;
            spinner.finish_and_clear();
            println!("   {} Generating fyg.toml", style("✓").green().bold());

            let spinner = create_spinner("Setting up source directories...");
            thread::sleep(Duration::from_millis(150));
            spinner.finish_and_clear();
            println!(
                "   {} Setting up source directories",
                style("✓").green().bold()
            );

            println!();
            println!(
                "{} {} created successfully!",
                style("Done!").green().bold(),
                style(&name).cyan()
            );
            println!();
            println!("To get started:");
            println!();
            println!("   {} {}", style("$").dim(), style(format!("cd {}", name)).yellow());
            println!("   {} {}", style("$").dim(), style("fyg build").yellow());
            println!();
        }

        Commands::Init { name, group } => {
            let current_dir = std::env::current_dir()?;
            let project_name = name.unwrap_or_else(|| {
                current_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("my-project")
                    .to_string()
            });

            if current_dir.join("fyg.toml").exists() {
                println!();
                println!(
                    "   {} {}",
                    style("✗").red().bold(),
                    style("fyg.toml already exists in this directory").red()
                );
                println!();
                anyhow::bail!("Project already initialised");
            }

            let fyg_toml = FygToml::new(&project_name, &group);

            println!();
            println!(
                "Initialising project {}",
                style(&project_name).cyan().bold()
            );
            println!(
                "   {} {}",
                style("Location:").dim(),
                style(current_dir.display()).dim()
            );
            println!();

            let spinner = create_spinner("Generating fyg.toml...");
            Fyg::init(&current_dir, fyg_toml)?;
            spinner.finish_and_clear();
            println!("   {} Generating fyg.toml", style("✓").green().bold());

            let spinner = create_spinner("Setting up source directories...");
            thread::sleep(Duration::from_millis(150));
            spinner.finish_and_clear();
            println!(
                "   {} Setting up source directories",
                style("✓").green().bold()
            );

            println!();
            println!(
                "{} Project {} initialised successfully!",
                style("Done!").green().bold(),
                style(&project_name).cyan()
            );
            println!();
            println!("To get started:");
            println!();
            println!("   {} {}", style("$").dim(), style("fyg build").yellow());
            println!();
        }        
        
        Commands::Build { release, target } => {
            let current_dir = std::env::current_dir()?;
            let config_path = current_dir.join("fyg.toml");
            
            if !config_path.exists() {
                anyhow::bail!("No fyg.toml found. Run 'fyg init' first.");
            }

            let config = FygToml::from_file(&config_path)?;
            let mode = if release { "release" } else { "debug" };
            
            println!("Building {} ({} mode)...", config.project.name, mode);
            if let Some(t) = &target {
                println!("  Target: {}", t);
            }
            
            Fyg::build(&config_path)?;
        }

        Commands::Clean => {
            let current_dir = std::env::current_dir()?;
            let build_dir = current_dir.join("build");

            println!();
            if build_dir.exists() {
                let spinner = create_spinner("Cleaning build directory...");
                std::fs::remove_dir_all(&build_dir)?;
                thread::sleep(Duration::from_millis(200));
                spinner.finish_and_clear();
                println!(
                    "   {} Removed build directory",
                    style("✓").green().bold()
                );
                println!();
                println!(
                    "{} {}",
                    style("Done!").green().bold(),
                    style("Clean complete").white()
                );
            } else {
                println!(
                    "   {} {}",
                    style("ℹ").blue().bold(),
                    style("Nothing to clean - build directory does not exist").dim()
                );
            }
            println!();
        }

        Commands::Run { args } => {
            let current_dir = std::env::current_dir()?;
            let config_path = current_dir.join("fyg.toml");
            
            if !config_path.exists() {
                anyhow::bail!("No fyg.toml found. Run 'fyg init' first.");
            }

            let config = FygToml::from_file(&config_path)?;
            println!("Running {}...", config.project.name);
            if !args.is_empty() {
                println!("  Args: {:?}", args);
            }
            
            // todo: create run command
        }

        Commands::Test { filter } => {
            let current_dir = std::env::current_dir()?;
            let config_path = current_dir.join("fyg.toml");
            
            if !config_path.exists() {
                anyhow::bail!("No fyg.toml found. Run 'fyg init' first.");
            }

            let config = FygToml::from_file(&config_path)?;
            println!("Running tests for {}...", config.project.name);
            if let Some(f) = &filter {
                println!("  Filter: {}", f);
            }
            
            // todo: create test command
        }

        Commands::Info => {
            let current_dir = std::env::current_dir()?;
            let config_path = current_dir.join("fyg.toml");

            if !config_path.exists() {
                println!();
                println!(
                    "   {} {}",
                    style("✗").red().bold(),
                    style("No fyg.toml found in current directory").red()
                );
                println!();
                anyhow::bail!("No fyg.toml found");
            }

            let config = FygToml::from_file(&config_path)?;

            println!();
            println!(
                "{} Project Information",
                style("ℹ").blue().bold()
            );
            println!();
            println!(
                "   {} {}",
                style("Name:").dim(),
                style(&config.project.name).cyan().bold()
            );
            println!(
                "   {} {}",
                style("Group:").dim(),
                style(&config.project.group).white()
            );
            println!(
                "   {} {}",
                style("Version:").dim(),
                style(&config.project.version).white()
            );

            if let Some(desc) = &config.project.description {
                println!(
                    "   {} {}",
                    style("Description:").dim(),
                    style(desc).white()
                );
            }

            if let Some(authors) = &config.project.authors {
                println!(
                    "   {} {}",
                    style("Authors:").dim(),
                    style(authors.join(", ")).white()
                );
            }

            if let Some(build) = &config.build {
                println!();
                println!("   {}", style("Build Configuration:").dim());
                if let Some(mp) = build.multiplatform {
                    println!(
                        "      {} {}",
                        style("Multiplatform:").dim(),
                        if mp {
                            style("yes").green()
                        } else {
                            style("no").yellow()
                        }
                    );
                }
                if let Some(langs) = &build.languages {
                    println!(
                        "      {} {}",
                        style("Languages:").dim(),
                        style(langs.join(", ")).white()
                    );
                }
            }

            if let Some(targets) = &config.targets {
                println!();
                println!("   {}", style("Targets:").dim());
                if let Some(jvm) = &targets.jvm {
                    if jvm.enabled {
                        let ver = jvm.target.as_deref().unwrap_or("default");
                        println!(
                            "      {} JVM (target: {})",
                            style("✓").green(),
                            style(ver).cyan()
                        );
                    }
                }
                if targets.linux_x64.as_ref().map(|t| t.enabled).unwrap_or(false) {
                    println!("      {} Linux x64", style("✓").green());
                }
                if targets.macos_arm64.as_ref().map(|t| t.enabled).unwrap_or(false) {
                    println!("      {} macOS ARM64", style("✓").green());
                }
                if targets.windows_x64.as_ref().map(|t| t.enabled).unwrap_or(false) {
                    println!("      {} Windows x64", style("✓").green());
                }
                if targets.ios_arm64.as_ref().map(|t| t.enabled).unwrap_or(false) {
                    println!("      {} iOS ARM64", style("✓").green());
                }
            }
            println!();
        }
    }

    Ok(())
}