use inquire::Select;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
struct MyConfig {
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

impl Default for MyConfig {
    fn default() -> Self {
        let mut extra = HashMap::new();
        extra.insert("Hello".to_string(), "echo Hello, World!".to_string());
        extra.insert("PingGoogle".to_string(), "ping 8.8.8.8".to_string());

        Self { extra }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: MyConfig = confy::load("s", "config")?;
    let mut options = settings.extra.keys().collect::<Vec<&String>>();
    options.sort();
    let choice = Select::new("Select your quick command", options).prompt()?;

    if let Some(command) = settings.extra.get(choice) {
        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        child.wait()?;
    }

    Ok(())
}
