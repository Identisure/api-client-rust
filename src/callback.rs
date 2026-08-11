use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// Verify the `X-Signature-SHA256` header of a callback request.
///
/// `body` must be the raw request body as received; verify before parsing and never re-serialize.
/// Returns `false` for a missing, malformed or mismatching signature.
pub fn verify_callback_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let Ok(expected) = STANDARD.decode(signature) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(body);

    mac.verify_slice(&expected).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &[u8] = br#"{"id":"abc","status":"COMPLETED"}"#;
    const SECRET: &str = "test-secret";
    const SIGNATURE: &str = "Ag2m3etnW31OP585jvcrvOupK7Vgsrzcq17KxKvkIOs=";

    #[test]
    fn accepts_valid_signature() {
        assert!(verify_callback_signature(BODY, SIGNATURE, SECRET));
    }

    #[test]
    fn rejects_tampered_body() {
        assert!(!verify_callback_signature(
            br#"{"id":"abd","status":"COMPLETED"}"#,
            SIGNATURE,
            SECRET
        ));
    }

    #[test]
    fn rejects_malformed_signature() {
        assert!(!verify_callback_signature(BODY, "not base64!", SECRET));
    }
}
