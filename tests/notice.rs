mod common;

use common::*;

#[cfg(test)]
mod test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn kakao_post() {
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
        let json: serde_json::Value =
            serde_json::from_str(post).expect("JSON was not well-formatted");

        assert_eq!(json["userRequest"]["utterance"].as_str(), Some("발화문\n"));
    }

    #[test]
    fn kakao_post_struct() {
        use serde::{Deserialize, Serialize};
        use serde_json::{Map, Value};

        #[allow(non_snake_case)]
        #[derive(Serialize, Deserialize)]
        struct Kakao {
            action: Map<String, Value>,
            bot: Map<String, Value>,
            contexts: Vec<String>,
            intent: Map<String, Value>,
            userRequest: Map<String, Value>,
        }

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
        let json: Kakao = serde_json::from_str(post).expect("JSON was not well-formatted");

        assert_eq!(json.userRequest["utterance"].as_str(), Some("발화문\n"));
    }

    #[tokio::test]
    async fn get_notices_from_ajou() -> Result<(), reqwest::Error> {
        use reqwest::header::USER_AGENT;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        struct Notice {
            id: u64,
            title: String,
            date: String,
            link: String,
            writer: String,
        }

        let mut ajou =
            "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=list&article.offset=0&articleLimit="
                .to_string();

        let nums = "5".to_string();

        ajou.push_str(&nums);

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let res = client.get(ajou).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().await?;
        // header 없이 보내면 404
        let body = res.text().await?;

        println!("Body:\n\n{:}", body);

        Ok(())
    }
}
