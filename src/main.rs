mod generators;

use std::env::args;
use std::{env, io};
use std::process::Command;
use crate::generators::{CommandGenerator, OpenAICommandGenerator};


fn collect_description() -> String {
    args().skip(1).collect::<Vec<String>>().join(" ")
}

fn generate_command(description: &str) -> Result<String, io::Error> {
    let api_key = env::var("OPENAI_API_KEY");

    if api_key.is_err() {
        eprintln!("OPENAI_API_KEY: {}", api_key.unwrap_err());
        return Err(io::Error::new(io::ErrorKind::NotFound, "OPENAI_API_KEY is not set"));
    }

    let generator = OpenAICommandGenerator::new(api_key.unwrap());
    generator.generate_command(description)
}

fn execute_command(command: &str) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    Command::new(command).args(args).spawn()?.wait()?;

    Ok(())
}

fn main() {
    let description = collect_description();
    let command = generate_command(&description);

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
