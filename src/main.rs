mod generators;

use std::env::args;
use std::{env, io};
use std::process::Command;
use crate::generators::{CommandGenerator, OpenAICommandGenerator};


fn execute_command(command: &str) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    Command::new(command).args(args).spawn()?.wait()?;

    Ok(())
}

fn main() {
    let description = args().skip(1).collect::<Vec<String>>().join(" ");

    let api_key = env::var("OPENAI_API_KEY");
    if api_key.is_err() {
        eprintln!("environment variable OPENAI_API_KEY is not set");
        return;
    }

    let generator = OpenAICommandGenerator::new(api_key.unwrap());

    match generator.generate_command(&description) {
        Ok(description) => {
            let command = generator.generate_command(&description);

            match command {
                Ok(command) => {
                    println!("Generated command: {}", command);
                    execute_command(&command).unwrap();
                }

                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Err(e) => eprintln!("Error: {}", e),
    }
}
