
async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> { //
    let response: serde_json::Value = reqwest::get(url)
    .await?
    .json::<serde_json::Value>()
    .await?;
    Ok(response)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]

    async fn tests_calls_async_fn(){
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let my_result = my_async_call(api_url).await;
        match my_result{
            Ok(r) => {
                dbg!(r);
            },
            Err(_) => {
                dbg!("failed");
            }
        };
    }
}