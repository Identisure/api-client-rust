use crate::models::DocumentUploadResponse;

#[cfg(feature = "async")]
use crate::client::IdentisureClient;

#[cfg(feature = "async")]
pub struct DocumentEndpoint<'a>(pub(crate) &'a IdentisureClient);

#[cfg(feature = "async")]
impl DocumentEndpoint<'_> {
    /// Upload a PDF document. Returns the document ID for use in sign requests.
    pub async fn upload(
        &self,
        data: Vec<u8>,
        filename: &str,
        content_type: &str,
    ) -> Result<DocumentUploadResponse, IdentisureError> {
        let part = reqwest::multipart::Part::bytes(data)
            .file_name(filename.to_owned())
            .mime_str(content_type)?;
        let form = reqwest::multipart::Form::new().part("file", part);

        self.0.post_multipart("/v3/document", form).await
    }

    /// Download a signed PDF by document ID. Returns raw PDF bytes.
    pub async fn download(&self, document_id: &str) -> Result<Vec<u8>, IdentisureError> {
        self.0
            .get_bytes(&format!("/v3/document/{}", document_id))
            .await
    }
}

use crate::IdentisureError;
#[cfg(feature = "blocking")]
use crate::client::IdentisureBlockingClient;

#[cfg(feature = "blocking")]
pub struct DocumentBlockingEndpoint<'a>(pub(crate) &'a IdentisureBlockingClient);

#[cfg(feature = "blocking")]
impl DocumentBlockingEndpoint<'_> {
    /// Upload a PDF document. Returns the document ID for use in sign requests.
    pub fn upload(
        &self,
        data: Vec<u8>,
        filename: &str,
        content_type: &str,
    ) -> Result<DocumentUploadResponse, IdentisureError> {
        let part = reqwest::blocking::multipart::Part::bytes(data)
            .file_name(filename.to_owned())
            .mime_str(content_type)?;
        let form = reqwest::blocking::multipart::Form::new().part("file", part);

        self.0.post_multipart("/v3/document", form)
    }

    /// Download a signed PDF by document ID. Returns raw PDF bytes.
    pub fn download(&self, document_id: &str) -> Result<Vec<u8>, IdentisureError> {
        self.0.get_bytes(&format!("/v3/document/{}", document_id))
    }
}
