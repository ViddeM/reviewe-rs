use reqwest::Client;
use serde::de::DeserializeOwned;
use thiserror::Error;
use types::ProductResponse;

pub mod types;

const OPEN_FOOD_FACTS_BASE_URL: &'static str = "https://world.openfoodfacts.org/api/v2";
const USER_AGENT: &'static str = "ReviewRS/0.1.0 (reviewers-contact@vidarmagnusson.com)";

pub type OFFResult<T> = Result<T, OpenFoodFactsError>;

#[derive(Error, Debug)]
pub enum OpenFoodFactsError {
    #[error("Encountered reqwest error: `{0:?}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Got an error response from OFF")]
    OffErrorResponse { status: u16, message: String },
}

pub struct OpenFoodFactsClient {
    client: Client,
}

impl OpenFoodFactsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_by_barcode<'a>(&self, barcode: &'a str) -> OFFResult<ProductResponse> {
        self.get(format!("product/{barcode}")).await
    }

    async fn get<T>(&self, ep: String) -> OFFResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{OPEN_FOOD_FACTS_BASE_URL}/{ep}"))
            .query(&("fields", "product_name,brands,_id,image_url"))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(OpenFoodFactsError::OffErrorResponse {
                status: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        let deserialized = response.json().await?;

        Ok(deserialized)
    }
}
