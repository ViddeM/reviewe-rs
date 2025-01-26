use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OFFResponse<T> {
    code: String,
    product: T,
    // Should really only ever be 0 or 1?
    status: u8,
    status_verbose: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductResponse {
    #[serde(rename = "_id")]
    id: String,
    brands: String,
    product_name: String,
    image_url: String,
}
