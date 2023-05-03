use crate::model::AlbumResponse;

const BASE_URL: &str = "https://itunes.apple.com/search";

pub struct Client {
    client: reqwest::Client,
    limit: u32,
}

pub struct ClientOptions {
    pub limit: u32,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self { limit: 200 }
    }
}

impl Client {
    pub fn new(opts: ClientOptions) -> anyhow::Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder().build()?,
            limit: opts.limit,
        })
    }

    /// Make a GET request to the API and deserialize the response
    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        query: &[(&str, &str)],
    ) -> anyhow::Result<T> {
        let res = self
            .client
            .get(BASE_URL)
            .query(query)
            .send()
            .await?
            .text()
            .await?;

        tracing::debug!("API response: {}", res);

        serde_json::from_str::<T>(res.as_str()).map_err(|err| anyhow::anyhow!(err))
    }

    /// Query for an album
    pub async fn find_album(&self, query: &str) -> anyhow::Result<AlbumResponse> {
        self.get(&[
            ("media", "music"),
            ("entity", "album"),
            ("limit", &self.limit.to_string()),
            ("country", "US"),
            ("term", query),
        ])
        .await
    }
}
