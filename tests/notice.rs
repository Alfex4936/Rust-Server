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
}
