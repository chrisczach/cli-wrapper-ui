#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use tauri::api::{
  config::get as get_config,
  path::{resolve_path, BaseDirectory},
};
use std::io::prelude::*;

fn main() -> anyhow::Result<()> {
  let config = get_config()?;

  let log_dir = resolve_path(
    config.tauri.bundle.identifier.clone(),
    Some(BaseDirectory::Cache),
  )?;

  if !log_dir.exists() {
    std::fs::create_dir_all(&log_dir)?;
  }

  let path = log_dir.join("app.log");
  if path.exists() {
    std::fs::remove_file(&path)?;
  }

  let mut log = std::fs::File::create(path)?;

  log.write_all(b"Hello Log\n")?;

  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      use std::process::Command;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            RunCommand { command } => {
              //  your command code
              let exit = Command::new(if cfg!(target_os = "windows") {
                "cmd"
              } else {
                "sh"
              })
              .args(vec!["-c", &command.to_string()])
              .status()
              .expect("error running command");

              if exit.success() {
                Ok(())
              } else {
                Err("Error running command".to_string())
              }
            }
          }
        }
      }
    })
    .build()
    .run();
    Ok(())
}
