use serde::Deserialize;

/// Response from the document upload endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUploadResponse {
    /// Document UUID — pass this as a reference in signing requests.
    pub id: String,
    /// SHA hash of the uploaded document for integrity verification.
    pub hash: String,
}
