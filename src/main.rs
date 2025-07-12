mod config;
mod option_name_validator;

use crate::config::Config;
use crate::option_name_validator::OptionNameValidator;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    const EDIT_COMMAND: &str = "Edit options";

    let settings = Config::load()?;
    let mut options = settings.keys();
    if options.contains(&EDIT_COMMAND.to_string()) {
        println!("You have one option named as '{EDIT_COMMAND}', it'll be overwritten. You can delete it by selecting 'Edit options'.");
    }
    options.sort();
    options.push(EDIT_COMMAND.to_string());
    let choice = Select::new("Select your quick command", options).prompt()?;

    if let Some(command) = settings.get(&choice) {
        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        child.wait()?;
    } else if choice == EDIT_COMMAND {
        edit_form()?;
    }

    Ok(())
}

fn edit_form() -> Result<()> {
    const NEW_COMMAND: &str = "Create new option";
    const EDIT_COMMAND: &str = "Edit existing option";
    const DELETE_COMMAND: &str = "Delete existing option";
    let options = vec![NEW_COMMAND, EDIT_COMMAND, DELETE_COMMAND];
    let choice = Select::new("Select your quick command", options).prompt()?;

    match choice {
        NEW_COMMAND => new_form()?,
        EDIT_COMMAND => edit_entry_form()?,
        DELETE_COMMAND => delete_form()?,
        &_ => {}
    }

    Ok(())
}

fn new_form() -> Result<()> {
    let mut settings = Config::load()?;
    let validator = OptionNameValidator::new(&settings)?;

    let name = Text::new("Name")
        .with_help_message("The name of the option.")
        .with_validator(validator)
        .prompt()?;

    let command = Text::new("Command")
        .with_help_message("The command to run when selected.")
        .prompt()?;

    settings.insert(name, command);
    settings.store()?;

    Ok(())
}

fn edit_entry_form() -> Result<()> {
    let mut settings = Config::load()?;
    let mut options = settings.keys();
    options.sort();
    let current_name = Select::new("Select the command to edit", options).prompt()?;

    if let Some(current_command) = settings.get(&current_name) {
        let new_name = Text::new("New name")
            .with_placeholder(&current_name)
            .with_help_message("Leave empty to keep the current name.")
            .with_default(&current_name)
            .prompt()?;

        let new_command = Text::new("New command")
            .with_placeholder(&current_command)
            .with_help_message("Leave empty to keep the current command.")
            .with_default(&current_command)
            .prompt()?;

        if new_name != current_name {
            settings.remove(current_name);
        }
        settings.insert(new_name, new_command);
        settings.store()?;
    }

    Ok(())
}

fn delete_form() -> Result<()> {
    let mut settings = Config::load()?;
    let mut options = settings.keys();
    options.sort();
    let choice = Select::new("Select the command to delete", options).prompt()?;

    if let Some(command) = settings.get(&choice) {
        let confirm = Confirm::new(&format!(
            "Are you sure you want to delete the command '{choice}'?"
        ))
        .with_default(false)
        .with_help_message(&format!("The associated command is '{command}'"))
        .prompt()?;
        if confirm {
            settings.remove(&choice);
            settings.store()?;
        }
    }

    Ok(())
}
