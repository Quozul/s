use inquire::Select;
use std::collections::HashMap;
use std::process::{Command, Stdio};

type MyConfig = HashMap<String, String>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: MyConfig = confy::load("s", "config")?;
    let options = settings.keys().collect();
    let choice = Select::new("Select your quick command", options).prompt()?;

    if let Some(command) = settings.get(choice) {
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
