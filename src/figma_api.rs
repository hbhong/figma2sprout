use reqwest::Client;

pub async fn fetch_figma_file(
    file_key: &str,
    access_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.figma.com/v1/files/{}", file_key);
    let response = client
        .get(&url)
        .header("X-Figma-Token", access_token)
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
}
