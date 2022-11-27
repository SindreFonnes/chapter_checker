pub async fn get_site_as_string(site: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(site).await?.error_for_status()?;
    response.text().await
}
