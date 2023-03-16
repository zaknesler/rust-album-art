use bson::serde_helpers::bson_datetime_as_rfc3339_string;
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
    pub wrapper_type: WrapperType,
    pub collection_type: String,
    pub artist_id: i64,
    pub collection_id: i64,
    pub amg_artist_id: Option<i64>,
    pub artist_name: String,
    pub collection_name: String,
    pub collection_censored_name: String,
    pub artist_view_url: Option<String>,
    pub collection_view_url: String,
    pub artwork_url60: Option<String>,
    pub artwork_url100: Option<String>,
    pub collection_price: Option<f64>,
    pub collection_explicitness: Explicitness,
    pub track_count: i64,
    pub copyright: String,
    pub country: String,
    pub currency: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub release_date: bson::DateTime,
    pub primary_genre_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WrapperType {
    ArtistFor,
    Collection,
    Track,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Explicitness {
    Cleaned,
    Explicit,
    NotExplicit,
}
