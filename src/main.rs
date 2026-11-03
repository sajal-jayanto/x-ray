mod runtime;

use clap::{Parser, Subcommand};
use indoc::indoc;
use std::fs;
use std::io;
use std::path::Path;
use runtime::{Runtime, get_runtimes, select_runtime};

#[derive(Parser, Debug)]
#[command(
  name = "create-cli",
  about = "x-ray",
  long_about = "A simple CLI tool for x-ray projects."
)]
struct Args {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Create a new project in the current directory
  Create {
    /// Name of the project to create
    project_name: String,
  },
  /// Initialize a project in an existing directory
  Init,
  /// Run project from 
  Run
}

fn main() {
  let args = Args::parse();
  let runtime_list = get_runtimes();

  match args.command {
    Commands::Create { project_name } => {
      let runtime = select_runtime(&runtime_list);
      let (file_name, content, dockerfile_content) = file_name_and_content(&runtime);
      let result = create_dir(&project_name, &file_name, &content, &dockerfile_content);

      if let Err(e) = result {
        eprintln!("Error: {}", e);
      }
    }
    Commands::Init => {
      let runtime = select_runtime(&runtime_list);
      let (main, main_content, dockerfile_content) = file_name_and_content(&runtime);
      let result = create_dir(".", &main, &main_content, &dockerfile_content);

      if let Err(e) = result {
        eprintln!("Error: {}", e);
      }
    }
    Commands::Run => {
      let image_name = get_current_dir_name();
      let image_exists = std::process::Command::new("docker")
        .args(["image", "inspect", &image_name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

      if !image_exists {
        let build_status = std::process::Command::new("docker")
          .args(["build", "-t", &image_name, "."])
          .status();

        match build_status {
          Ok(status) if status.success() => println!("Build successful"),
          Ok(_) => { eprintln!("Build failed"); return; }
          Err(e) => { eprintln!("Failed to run docker build: {}", e); return; }
        }
      }
      else {
        println!("Image already exists. Skipping build.");
      }

      let run_status = std::process::Command::new("docker")
        .args(["run", &image_name]) // --rm = stop + remove after exit
        .status();

      match run_status {
        Ok(status) if status.success() => println!("Container finished successfully"),
        Ok(_) => eprintln!("Container exited with error"),
        Err(e) => eprintln!("Failed to run container: {}", e),
      }
    }
  }
}

fn create_dir(
  folder_name: &str,
  main: &str,
  main_content: &str,
  dockerfile_content: &str,
) -> io::Result<()> {
  fs::create_dir_all(folder_name)?;

  let path = Path::new(folder_name).join(main);
  fs::write(&path, main_content)?;

  let path = Path::new(folder_name).join("Dockerfile");
  fs::write(&path, dockerfile_content)?;

  Ok(())
}

fn get_current_dir_name() -> String {
  let dir_name = std::env::current_dir()
    .ok()
    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    .unwrap_or_else(|| "unknown".to_string());

  return dir_name;
}

fn file_name_and_content(runtime: &Runtime) -> (String, String, String) {
  match runtime {
    Runtime::NodeJS24 => (
      "index.js".to_string(),
      r#"console.log("Hello, World!");"#.to_string(),
      indoc! {r#"
        FROM node:24-alpine
        WORKDIR /app
        COPY . .
        CMD ["node", "index.js"]
      "#}
      .to_string(),
    ),

    Runtime::Java25 => (
      "Main.java".to_string(),
      indoc! {r#"
        public class Main {
          public static void main(String[] args) {
            System.out.println("Hello, World!");
          }
        }
      "#}
      .to_string(),
      indoc! {r#"
        FROM eclipse-temurin:25-jdk-alpine
        WORKDIR /app
        COPY . .
        RUN javac Main.java
        CMD ["java", "Main"]
      "#}
      .to_string(),
    ),
    
    Runtime::Go => (
      "main.go".to_string(),
      indoc! {r#"
        package main

        import "fmt"

        func main() {
          fmt.Println("Hello, World!")
        }
      "#}
      .to_string(),
      indoc! {r#"
        FROM golang:1.23-alpine
        WORKDIR /app
        COPY . .
        RUN go build -o app main.go
        CMD ["./app"]
      "#}
      .to_string(),
    ),
  }
}
