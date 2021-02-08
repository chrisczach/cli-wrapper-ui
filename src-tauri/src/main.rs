#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(move |_webview, arg| {
      use cmd::Cmd::*;
      use std::process::Command;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            RunCommand { command } => {
              //  your command code
              let mut shell = if cfg!(target_os = "windows") {
                Command::new("cmd")
              } else {
                Command::new("sh")
              };

              shell.arg("-c");
              shell.arg(command);

              let exit = shell.status().expect("error running command");
              println!("process exited with: {}", exit);
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
}
