use crate::model::AlbumResponse;

#[derive(Default)]
pub struct Client {
    client: Option<reqwest::Client>,
}

impl Client {
    pub fn init(&mut self) -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;
        self.client = Some(client);
        Ok(())
    }

    pub async fn find_album(&self, query: &str) -> anyhow::Result<AlbumResponse> {
        if self.client.is_none() {
            return Err(anyhow::anyhow!("Client not initialized"));
        }

        let url = format!(
            "https://itunes.apple.com/search?media=music&entity=album&limit=200&country=US&term={}",
            query,
        );
        let res = reqwest::get(url).await?.text().await?;

        tracing::debug!("Album response: {}", res);

        serde_json::from_str::<AlbumResponse>(res.as_str()).map_err(|err| anyhow::anyhow!(err))
    }
}
