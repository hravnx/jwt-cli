use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use std::fmt;

/// Errors that can occur while decoding the payload segment of a JWT.
#[derive(Debug)]
pub enum DecodeJwtError {
    /// The token did not include a header segment.
    MissingHeader,
    /// The token did not include a payload segment.
    MissingPayload,
    /// The token did not include a signature segment.
    MissingSignature,
    /// The token contained more than three dot-separated segments.
    InvalidSegmentCount,
    /// The payload segment could not be decoded as base64url.
    InvalidBase64(base64::DecodeError),
    /// The decoded payload bytes were not valid UTF-8 text.
    InvalidUtf8(std::string::FromUtf8Error),
}

impl fmt::Display for DecodeJwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "JWT is missing a header"),
            Self::MissingPayload => write!(f, "JWT is missing a payload"),
            Self::MissingSignature => write!(f, "JWT is missing a signature"),
            Self::InvalidSegmentCount => {
                write!(f, "JWT must contain exactly three dot-separated segments")
            }
            Self::InvalidBase64(error) => write!(f, "payload is not valid base64url: {error}"),
            Self::InvalidUtf8(error) => write!(f, "payload is not valid UTF-8: {error}"),
        }
    }
}

/// Decodes the payload segment from a JWT and returns it as UTF-8 text.
///
/// The input must contain exactly three dot-separated segments. The payload
/// segment is decoded using base64url without padding.
pub fn decode_jwt_payload(token: &str) -> Result<String, DecodeJwtError> {
    let mut parts = token.split('.');

    let _header = parts
        .next()
        .ok_or(DecodeJwtError::MissingHeader)?;
    let payload = parts
        .next()
        .ok_or(DecodeJwtError::MissingPayload)?;
    let _signature = parts
        .next()
        .ok_or(DecodeJwtError::MissingSignature)?;

    if parts.next().is_some() {
        return Err(DecodeJwtError::InvalidSegmentCount);
    }

    let decoded = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(DecodeJwtError::InvalidBase64)?;

    String::from_utf8(decoded).map_err(DecodeJwtError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use super::{DecodeJwtError, decode_jwt_payload};

    #[test]
    fn decodes_payload_segment() {
        let token = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIn0.";

        let payload = decode_jwt_payload(token).expect("payload should decode");

        assert_eq!(payload, r#"{"sub":"1234567890","name":"John Doe"}"#);
    }

    #[test]
    fn rejects_wrong_number_of_segments() {
        let error = decode_jwt_payload("a.b").expect_err("token should be rejected");

        assert!(matches!(error, DecodeJwtError::MissingSignature));
    }
}
