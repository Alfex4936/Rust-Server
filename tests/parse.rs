mod common;

use common::*;

#[cfg(test)]
mod test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn simple_html_parse() -> Result<(), reqwest::Error> {
        use serde::{Deserialize, Serialize};
        use std::env;

        #[derive(Serialize, Deserialize, Debug)]
        struct Notice {
            id: u64,
            title: String,
            date: String,
            link: String,
            writer: String,
        }

        let mut server = env::var("AWS_SERVER")
            .unwrap_or("localhost:8000".to_string())
            .to_string();

        let query = "/v1/notices/".to_string();

        let nums = "1".to_string();

        server.push_str(&query);
        server.push_str(&nums);

        let res = reqwest::get(server).await?;
        println!("Status: {}", res.status());
        let body = res.text().await?;

        let json: Vec<Notice> = serde_json::from_str(&body).expect("JSON was not well-formatted");

        println!("JSON:\n\n{:?}", json);
        Ok(())
    }
}
