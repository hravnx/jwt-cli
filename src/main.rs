use std::io::{self, Read};
use std::process::ExitCode;

mod jwt;

use jwt::decode_jwt_payload;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;

    let token = input.trim();
    if token.is_empty() {
        return Err("expected a JWT on stdin".to_string());
    }

    let payload = decode_jwt_payload(token).map_err(|error| error.to_string())?;
    println!("{payload}");

    Ok(())
}
