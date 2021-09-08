use ajou_notice::{AJOU_LINK, MY_USER_AGENT};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, options::FindOptions, Client};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notice {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

pub async fn notice_parse(
    query_option: &str,
    _nums: Option<usize>,
) -> Result<Vec<Notice>, reqwest::Error> {
    // let query = "?mode=list&article.offset=0&articleLimit=";

    // query = ?mode=list&srSearchKey=&srSearchVal=키워드&article.offset=0&articleLimit=

    let string;
    let query = match query_option {
        "ajou" => "?mode=list&article.offset=0&articleLimit=",
        _ => {
            string = format!(
                "?mode=list&srSearchKey=&srSearchVal={}&article.offset=0&articleLimit=",
                query_option
            ); // format! has dropped so gotta save it to temp var
            &string
        }
    };

    let nums_int = _nums.unwrap_or(7);
    // ajou.push_str(&nums_str);

    let url = [AJOU_LINK, query, &nums_int.to_string()].concat();

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(2))
        .build()?;

    // header 없이 보내면 404
    let res = client
        .get(url)
        .header(USER_AGENT, MY_USER_AGENT)
        .send()
        .await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    // HTML Parse
    let document = Html::parse_document(&body);
    let a_selector = Selector::parse("a").unwrap();

    // Notice has id, title, date, link, writer
    let ids = Selector::parse("td.b-num-box").unwrap();
    let cates = Selector::parse("span.b-cate").unwrap(); // 카테고리
    let titles = Selector::parse("div.b-title-box").unwrap(); // includes links
    let dates = Selector::parse("span.b-date").unwrap();
    let writers = Selector::parse("span.b-writer").unwrap();

    // let mut notices = vec![Notice::default(); nums_int];
    let mut notices: Vec<Notice> = vec![];

    let mut id_elements = document.select(&ids);
    let mut cate_elements = document.select(&cates);
    let mut title_elements = document.select(&titles);
    let mut date_elements = document.select(&dates);
    let mut writer_elements = document.select(&writers);

    // struct Notice
    while let Some(id_element) = id_elements.next() {
        let id = id_element.text().collect::<Vec<_>>()[0]
            .trim() // " 12345 "
            .parse::<i32>()
            .unwrap();

        let date_element = date_elements.next().unwrap();
        let date = date_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // "2021-07-15"

        let writer_element = writer_elements.next().unwrap();
        let writer = writer_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // "가나다라마"

        let cate_element = cate_elements.next().unwrap();
        let category = cate_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // " 학사 "

        let title_element = title_elements.next().unwrap();
        let inner_a = title_element.select(&a_selector).next().unwrap();

        let mut title = inner_a.value().attr("title").unwrap().to_string();
        let link = AJOU_LINK.to_string() + inner_a.value().attr("href").unwrap();

        // Check duplication. title: [writer] blah -> title: [blah]
        let dup = "[".to_string() + &writer + "]";

        if title.contains(&dup) {
            title = title.replace(&dup, "");
            // title.replace_range(0..dup.len(), "");
        }
        // println!("id: {}, title: {}, link: {}, date: {}, writer: {}", id, title, link, date, writer);

        let useless = " 자세히 보기".to_string();
        if title.contains(&useless) {
            title = title.replace(&useless, "");
        }

        let useless = "(재공지)".to_string();
        if title.contains(&useless) {
            title = title.replace(&useless, "");
        }

        title = title.trim().to_string();

        // title.retain(|c| !r#"~「」"#.contains(c));

        let notice = Notice {
            id,
            category,
            title,
            link,
            date,
            writer,
        };

        notices.push(notice);

        // (*notice).id = id;
        // (*notice).title = title;
        // (*notice).link = link;
        // (*notice).date = date;
        // (*notice).writer = writer;
    }
    // println!("{:?}", notices);

    notices.reverse();
    Ok(notices)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Connecting mongo-db...");
    let client_options = ClientOptions::parse(ajou_notice::MONGO_URL).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let notice_collection = client.database("ajou").collection::<Notice>("notice");

    println!("Connected!");

    let mut rust_notices: Vec<Notice> = Vec::new();

    let find_options = FindOptions::builder()
        .sort(doc! { "id": -1})
        .limit(1)
        .build();

    let mut db_notices = notice_collection.find(doc! {}, find_options).await.unwrap();

    while let Some(notice) = db_notices.try_next().await.unwrap() {
        rust_notices.push(notice);
    }
    let parsed_notice = notice_parse("ajou", Some(1)).await.unwrap();

    let last_db_notice_id = rust_notices.first().unwrap().id;
    let last_parsed_notice_id = parsed_notice.first().unwrap().id;

    let num_missing_notices: usize = (last_parsed_notice_id - last_db_notice_id) as usize;

    let parsed_notices = if num_missing_notices != 0 {
        notice_parse("ajou", Some(num_missing_notices))
            .await
            .unwrap()
    } else {
        vec![]
    };

    println!(
        "DB: {}, PARSED: {}",
        last_db_notice_id, last_parsed_notice_id
    );

    println!("Loaded: {:#?}", parsed_notices);
    if !parsed_notices.is_empty() {
        notice_collection
            .insert_many(parsed_notices, None)
            .await
            .unwrap();
    }

    Ok(())
}
