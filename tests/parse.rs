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
            println!("sched: {}", sched["title"].to_string());
        }
        // for sched in json["template"]["outputs"] as Vec<Notice> {}

        Ok(())
    }

    #[tokio::test]
    async fn html_parse() -> Result<(), reqwest::Error> {
        use arr_macro::arr;
        use reqwest::header::USER_AGENT;
        use scraper::{Html, Selector};

        #[derive(Debug)]
        struct Notice {
            id: u64,
            title: String,
            date: String,
            link: String,
            writer: String,
        }

        impl Default for Notice {
            fn default() -> Notice {
                Notice {
                    id: 0,
                    title: "".to_string(),
                    date: "".to_string(),
                    link: "".to_string(),
                    writer: "".to_string(),
                }
            }
        }

        let mut ajou =
            "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=list&article.offset=0&articleLimit="
                .to_string();

        let nums = "10".to_string();

        ajou.push_str(&nums);

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        // header 없이 보내면 404
        let res = client.get(ajou).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().await?;
        let body = res.text().await?;

        // println!("Body:\n\n{:?}", body);

        // HTML Parse
        let document = Html::parse_document(&body);
        let a_selector = Selector::parse("a").unwrap();

        // Notice has id, title, date, link, writer
        let ids = Selector::parse("td.b-num-box").unwrap();
        let titles = Selector::parse("div.b-title-box").unwrap(); // includes links
        let dates = Selector::parse("span.b-date").unwrap();
        let writers = Selector::parse("span.b-writer").unwrap();

        let mut notices: [Notice; 10] = arr![Notice::default(); 10];

        let mut id_elements = document.select(&ids);
        let mut title_elements = document.select(&titles);
        let mut date_elements = document.select(&dates);
        let mut writer_elements = document.select(&writers);

        // struct Notice
        for index in 0..10 {
            let id_element = id_elements.next().unwrap();
            let id = id_element.text().collect::<Vec<_>>()[0]
                .trim() // " 12345 "
                .parse::<u64>()
                .unwrap();

            let date_element = date_elements.next().unwrap();
            let date = date_element.text().collect::<Vec<_>>()[0]
                .trim()
                .to_string(); // "2021-07-15"

            let writer_element = writer_elements.next().unwrap();
            let writer = writer_element.text().collect::<Vec<_>>()[0]
                .trim()
                .to_string(); // "가나다라마"

            let title_element = title_elements.next().unwrap();
            let inner_a = title_element.select(&a_selector).next().unwrap();

            let mut title = inner_a.value().attr("title").unwrap().to_string();
            let link = inner_a.value().attr("href").unwrap().to_string();
            // Check duplication. title: [writer] blah -> title: [blah]
            let dup = "[".to_string() + &writer + "]";
            if title.contains(&dup) {
                title.replace_range(0..dup.len(), "");
                title = title.trim().to_string();
            }
            // println!("id: {}, title: {}, link: {}, date: {}, writer: {}", id, title, link, date, writer);

            notices[index].id = id;
            notices[index].title = title;
            notices[index].link = link;
            notices[index].date = date;
            notices[index].writer = writer;
        }

        println!("notices: {:?}", notices);

        Ok(())
    }
}
