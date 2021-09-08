use ajou_notice::{AJOU_LINK, MY_USER_AGENT};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, options::FindOptions, Client};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

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
        .connect_timeout(Duration::from_secs(5))
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
    for id_element in &mut id_elements {
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
    'main: loop {
        // db connection check?
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        // println!("Seoul: {}", seoul_now);
        match seoul_now.weekday() {
            Weekday::Sat | Weekday::Sun => {
                let monday = Seoul
                    .ymd(seoul_now.year(), seoul_now.month(), seoul_now.day())
                    .and_hms(9, 0, 0)
                    + chrono::Duration::days(7 - seoul_now.weekday().num_days_from_monday() as i64);

                let difference = (monday - seoul_now).num_seconds();
                println!(
                    "Weekend...resting until next KST monday 9am: {} seconds",
                    difference
                );
                sleep(Duration::from_secs(difference as u64)).await;
            }
            _ => {}
        }
        if seoul_now.hour() >= 19 || seoul_now.hour() <= 8 {
            let mut next_morning = Seoul
                .ymd(seoul_now.year(), seoul_now.month(), seoul_now.day())
                .and_hms(9, 30, 0);

            if seoul_now.hour() >= 19 {
                next_morning = next_morning + chrono::Duration::days(1);
            }

            let difference = (next_morning - seoul_now).num_seconds();

            println!(
                "Night time...resting until next KST 9am: {} seconds",
                difference
            );
            sleep(Duration::from_secs(difference as u64)).await;
            continue 'main;
        }

        println!("Parsing notices now...");
        let notices: Vec<Notice> = match notice_parse("ajou", Some(30)).await {
            Ok(n) => n,
            _ => {
                sleep(Duration::from_secs(300)).await;
                continue 'main;
            }
        };

        if notices.is_empty() {
            println!("No notices, resting...");
            sleep(Duration::from_secs(300)).await;
            continue 'main;
        }

        for notice in notices.iter() {
            let post_id = notice.id;
            let mut db_notice = notice_collection
                .find(doc! { "id": post_id }, None)
                .await
                .unwrap();

            match db_notice.try_next().await {
                Ok(Some(_)) => continue, // duplicated
                Ok(None) => notice_collection.insert_one(notice, None).await.unwrap(),
                Err(_) => continue 'main,
            };

            // notice_collection.insert_one(notice, None).await.unwrap();
        }
        println!("Updated!, resting 30mins...");

        sleep(Duration::from_secs(1800)).await;
    }
}
