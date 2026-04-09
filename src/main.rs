use std::io::{self, Read};
use std::process::ExitCode;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

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

    let payload = decode_jwt_payload(token)?;
    println!("{payload}");

    Ok(())
}

fn decode_jwt_payload(token: &str) -> Result<String, String> {
    let mut parts = token.split('.');

    let _header = parts
        .next()
        .ok_or_else(|| "JWT is missing a header".to_string())?;
    let payload = parts
        .next()
        .ok_or_else(|| "JWT is missing a payload".to_string())?;
    let _signature = parts
        .next()
        .ok_or_else(|| "JWT is missing a signature".to_string())?;

    if parts.next().is_some() {
        return Err("JWT must contain exactly three dot-separated segments".to_string());
    }

    let decoded = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|error| format!("payload is not valid base64url: {error}"))?;

    String::from_utf8(decoded).map_err(|error| format!("payload is not valid UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
    use super::decode_jwt_payload;

    #[test]
    fn decodes_payload_segment() {
        let token = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIn0.";

        let payload = decode_jwt_payload(token).expect("payload should decode");

        assert_eq!(payload, r#"{"sub":"1234567890","name":"John Doe"}"#);
    }

    #[test]
    fn rejects_wrong_number_of_segments() {
        let error = decode_jwt_payload("a.b").expect_err("token should be rejected");

        assert!(error.contains("missing a signature"));
    }
}
