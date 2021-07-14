mod common;

use common::*;

#[cfg(test)]
mod test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn return_json() {
        use rocket_contrib::json::Json;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        struct Notice {
            id: u64,
            title: String,
            date: String,
            link: String,
            writer: String,
        }

        let notice = Notice {
            id: 12345,
            title: "공지1".to_string(),
            date: "2021-07-09".to_string(),
            link: "https://".to_string(),
            writer: "CSW".to_string(),
        };

        println!("{:?}", Json(notice));
    }

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

        let nums = "2".to_string();

        server.push_str(&query);
        server.push_str(&nums);

        let res = reqwest::get(server).await?;
        println!("Status: {}", res.status());
        let body = res.text().await?;

        let json: Vec<Notice> = serde_json::from_str(&body).expect("JSON was not well-formatted");

        println!("JSON:\n\n{:?}", json);

        assert_eq!(json.len(), nums.trim().parse::<usize>().unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn kakao_json() -> Result<(), reqwest::Error> {
        use std::env;

        let mut server = env::var("AWS_SERVER")
            .unwrap_or("localhost:8000".to_string())
            .to_string();

        let query = "/v1/schedule".to_string();

        server.push_str(&query);

        let client = reqwest::Client::new();
        let post = r#"{
            "action": {
                "clientExtra": {},
                "detailParams": {
                    "sys_text": {
                        "groupName": "",
                        "origin": "2021",
                        "value": "2021"
                    }
                },
                "id": "id",
                "name": "스킬 이름",
                "params": {
                    "sys_text": "2021"
                }
            },
            "bot": {
                "id": "id",
                "name": "botName"
            },
            "contexts": [],
            "intent": {
                "extra": {
                    "reason": {
                        "code": 1,
                        "message": "OK"
                    }
                },
                "id": "id",
                "name": "intentName"
            },
            "userRequest": {
                "block": {
                    "id": "id",
                    "name": "userRequestName"
                },
                "lang": "kr",
                "params": {
                    "ignoreMe": "true",
                    "surface": "BuilderBotTest"
                },
                "timezone": "Asia/Seoul",
                "user": {
                    "id": "id",
                    "properties": {
                        "botUserKey": "key",
                        "bot_user_key": "key"
                    },
                    "type": "botUserKey"
                },
                "utterance": "발화문\n"
            }
        }"#;
        let res = client.post(server).json(post).send().await?;
        // println!("Status: {}", res.status());
        let body = res.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body).expect("Error");

        let sub_values = json["template"]["outputs"][0]["carousel"]["items"]
            .as_array()
            .unwrap();

        for sched in sub_values.iter() {
            println!("title: {}", sched["title"].to_string());
        }
        // for sched in json["template"]["outputs"] as Vec<Notice> {}

        Ok(())
    }
}
