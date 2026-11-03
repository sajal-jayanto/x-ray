use dialoguer::{Select, theme::ColorfulTheme};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Runtime {
  NodeJS24,
  Java25,
  Go,
}

impl fmt::Display for Runtime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Runtime::NodeJS24 => write!(f, "Node.js 24.x"),
      Runtime::Java25 => write!(f, "Java 25"),
      Runtime::Go => write!(f, "Go"),
    }
  }
}

impl FromStr for Runtime {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Node.js 24.x" => Ok(Runtime::NodeJS24),
      "Java 25" => Ok(Runtime::Java25),
      "Go" => Ok(Runtime::Go),
      _ => Err(format!("Unsupported runtime: {}", s)),
    }
  }
}

pub fn select_runtime(runtime_list: &[Runtime]) -> Runtime {
  let selected_idx = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Choose a runtime:")
    .default(0)
    .items(runtime_list)
    .interact()
    .unwrap_or(0);

  runtime_list[selected_idx]
}

pub fn get_runtimes() -> Vec<Runtime> {
  vec![Runtime::NodeJS24, Runtime::Java25, Runtime::Go]
}
