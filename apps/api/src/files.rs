//! File upload policy — uploads are not enabled in MVP.
//!
//! When ticket attachments ship, enforce MIME allowlist, size caps,
//! random storage keys, and virus scanning outside the request path.

use crate::error::AppError;

pub const MAX_UPLOAD_BYTES: usize = 5 * 1024 * 1024;

const ALLOWED_CONTENT_TYPES: &[&str] =
    &["image/jpeg", "image/png", "image/webp", "application/pdf"];

pub fn assert_uploads_enabled() -> Result<(), AppError> {
    Err(AppError::Validation(
        "File uploads are not enabled yet.".to_owned(),
    ))
}

pub fn validate_upload_meta(content_type: &str, byte_size: usize) -> Result<(), AppError> {
    assert_uploads_enabled()?;

    if byte_size == 0 || byte_size > MAX_UPLOAD_BYTES {
        return Err(AppError::Validation(
            "File size is outside the allowed range.".to_owned(),
        ));
    }

    if !ALLOWED_CONTENT_TYPES.contains(&content_type) {
        return Err(AppError::Validation("File type is not allowed.".to_owned()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{assert_uploads_enabled, validate_upload_meta};

    #[test]
    fn uploads_disabled() {
        assert!(assert_uploads_enabled().is_err());
        assert!(validate_upload_meta("image/png", 100).is_err());
    }
}
