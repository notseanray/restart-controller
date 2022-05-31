use std::time::Duration;
use std::process::Command;
use std::fs;
use std::thread::sleep;
use serde::Deserialize;

#[derive(Deserialize)]
struct Tmux {
    name: String,
    commands: Vec<String>,
    delay: Option<u32>,
}

#[derive(Deserialize)]
struct Script {
    command: String
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Entry {
    Tmux(Tmux),
    Script(Script),
}

trait Runnable {
    fn run(&self) {}
}

impl Runnable for Tmux {
    fn run(&self) {
        let delay = self.delay.unwrap_or(0) as u64;
        let _ = Command::new("tmux")
            .args(["new-session", "-d", "-s", &self.name])
            .status();
        for command in &self.commands {
            sleep(Duration::from_millis(delay));
            let _ = Command::new("tmux")
                .args(["send-keys", "-t", &self.name, command, "C-m"])
                .status();
        }
        sleep(Duration::from_millis(delay));
        let _ = Command::new("tmux")
            .args(["detach", "-s", &self.name])
            .status();
     } 
}

impl Runnable for Script {
    fn run(&self) {
        let cmd: Vec<&str> = self.command.split_whitespace().collect();
        let _ = Command::new(cmd[0])
            .args(&cmd[1..])
            .status();
    }
}

impl Runnable for Entry {
    fn run(&self) {
        match self {
            Entry::Tmux(v) => v.run(),
            Entry::Script(v) => v.run()
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let config = fs::read_to_string("/etc/restart-controller/config.json")?;
    let entries: Vec<Entry> = serde_json::from_str(&config)?;
    for entry in entries {
        entry.run();
    }
    Ok(())
}
