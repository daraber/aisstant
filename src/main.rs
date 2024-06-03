use std::env::args;
use std::io;
use std::process::Command;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

mod openai;

fn collect_description() -> String {
    args().skip(1).collect::<Vec<String>>().join(" ")
}

fn execute_command(command: &str) -> io::Result<()> {
    if command.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Generated command is empty"));
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Command: {}", command))
        .default(0)
        .items(&["Execute", "Cancel"])
        .interact()
        .unwrap();

    if selection == 1 {
        return Ok(());
    }

    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    Command::new(command).args(args).spawn()?.wait()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let description = collect_description();
    let command = openai::generate_command(&description).await;

    match command {
        Ok(command) => {
            match execute_command(&command) {
                Ok(_) => println!("Command executed successfully"),
                Err(e) => eprintln!("Error executing command: {}", e),
            }
        }
        Err(e) => eprintln!("Error generating command: {}", e),
    }
}
