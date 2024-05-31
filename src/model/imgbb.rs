use std::env;

use serde::Deserialize;

use super::error::AppError;

const IMGBB_URL: &str = "https://api.imgbb.com/1/upload";
const MONTH_TO_SECOND: u64 = 60 * 60 * 24 * 30;

#[derive(Debug, Deserialize, Clone)]
pub struct ImgbbResponse {
    pub data: ImgbbData,
    pub success: Option<bool>,
    pub status: Option<u16>,
}

/// An struct for *data* field in ImgBB API response
#[derive(Debug, Deserialize, Clone)]
pub struct ImgbbData {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url_viewer: Option<String>,
    pub url: Option<String>,
    pub display_url: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub size: Option<u32>,
    pub time: Option<u64>,
    pub expiration: Option<u64>,
    pub image: Option<ImgbbImage>,
    pub thumb: Option<ImgbbImage>,
    pub medium: Option<ImgbbImage>,
    pub delete_url: Option<String>,
}

/// An struct for image value in ImgBB API response
#[derive(Debug, Deserialize, Clone)]
pub struct ImgbbImage {
    pub filename: Option<String>,
    pub name: Option<String>,
    pub mime: Option<String>,
    pub extension: Option<String>,
    pub url: Option<String>,
}

/// An struct that holds the data (base64) to be uploaded
pub struct ImgbbUploader {
    /// ImgBB API key
    pub api_key: String,
    /// Base64 data to be uploaded
    pub data: String,
    /// Expiration time in seconds
    pub expiration: u64,
}

impl ImgbbUploader {
    /// Creates a new Uploader struct with the given API key
    pub fn new(data: String) -> Self {
        let api_key = env::var("IMGBB_API_KEY").expect("IMGBB_API_KEY must be set!");
        let expiration = MONTH_TO_SECOND * 12;
        Self {
            api_key,
            data,
            expiration,
        }
    }

    /// Upload [data](Uploader::data) to ImgBB
    pub async fn upload(self) -> Result<ImgbbResponse, AppError> {
        let query = vec![
            ("key", self.api_key),
            ("expiration", self.expiration.to_string()),
        ];

        let form = [("image", self.data)];

        let imgbb_res = reqwest::Client::new()
            .post(IMGBB_URL)
            .query(&query)
            .form(&form)
            .send()
            .await
            .map_err(|_| AppError::new("Failed upload file to server!".to_string()))?;
        if imgbb_res.status().is_success() {
            let res = imgbb_res.json::<ImgbbResponse>().await?;
            Ok(res)
        } else {
            let err_message = imgbb_res.text().await?;
            let err = AppError::new(err_message);
            Err(err)
        }
    }
}
