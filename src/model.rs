use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResponse {
    pub result_count: i64,
    pub results: Vec<Album>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub wrapper_type: String,
    pub collection_type: String,
    pub artist_id: i64,
    pub collection_id: i64,
    pub amg_artist_id: Option<i64>,
    pub artist_name: String,
    pub collection_name: String,
    pub collection_censored_name: String,
    pub artist_view_url: Option<String>,
    pub collection_view_url: String,
    pub artwork_url60: String,
    pub artwork_url100: String,
    pub collection_price: Option<f64>,
    pub collection_explicitness: Explicitness,
    pub track_count: i64,
    pub copyright: String,
    pub country: String,
    pub currency: String,
    pub release_date: String,
    pub primary_genre_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Explicitness {
    Explicit,
    NotExplicit,
}
