use crate::db::models::{Library, Meal, Notice, People, Weather};
use crate::MY_USER_AGENT;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";
pub const NAVER_WEATHER: &str = "https://m.search.naver.com/search.naver?sm=tab_hty.top&where=nexearch&query=%EB%82%A0%EC%94%A8+%EB%A7%A4%ED%83%843%EB%8F%99&oquery=%EB%82%A0%EC%94%A8"; // 아주대 지역 날씨
pub const NAVER_WEATHER_ICON: &str = "https://weather.naver.com/today/02117530?cpName=ACCUWEATHER"; // 아주대 지역 날씨는
pub const AJOU_LIBRARY: &str = env!("AJOU_LIBRARY"); // 아주대 중앙 도서관
pub const AJOU_PEOPLE: &str = env!("AJOU_PEOPLE"); // 아주대 인물 검색
pub const AJOU_MEAL: &str = env!("AJOU_MEAL"); // 아주대 인물 검색
                                               // pub const AJOU_MEAL: &str = env!("AJOU_MEAL"); // 아주대 학식

pub async fn notice_parse(
    query_option: &str,
    _nums: Option<usize>,
) -> Result<Vec<Notice>, reqwest::Error> {
    // let query = "?mode=list&article.offset=0&articleLimit=";

    // query = ?mode=list&srSearchKey=&srSearchVal=키워드&article.offset=0&articleLimit=

    let string;
    let query = match query_option {
        "ajou" => "?mode=list&article.offset=0&articleLimit=",
        "category" => "?mode=list&articleLimit=5&srCategoryId=",
        _ => {
            string = format!(
                "?mode=list&srSearchKey=&srSearchVal={}&article.offset=0&articleLimit=",
                query_option
            ); // format! has dropped so gotta save it to temp var
            &string
        }
    };

    let nums_int = _nums.unwrap_or(5);
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
    for id_element in &mut id_elements {
        let id = match id_element.text().collect::<Vec<_>>()[0]
            .trim() // " 12345 "
            .parse::<i32>()
        {
            Ok(some) => some,
            Err(_) => {
                date_elements.next().unwrap();
                writer_elements.next().unwrap();
                cate_elements.next().unwrap();
                title_elements.next().unwrap();
                continue; // 번호가 "공지"
            }
        };
        // .unwrap();

        let date_element = date_elements.next().unwrap();
        let date = date_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // "2021-07-15"

        let writer_element = writer_elements.next().unwrap();
        let writer = writer_element.text().collect::<Vec<_>>();
        let writer = if writer.is_empty() {
            "알 수 없음".to_string()
        } else {
            writer[0].trim().to_string() // "가나다라마"
        };

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
    }
    // println!("{:?}", notices);

    // notices.reverse();
    Ok(notices)
}

pub async fn weather_parse() -> Result<Weather, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .user_agent(MY_USER_AGENT)
        .build()?;

    let res = client.get(NAVER_WEATHER).send().await?;
    let res2 = client.get(NAVER_WEATHER_ICON).send().await?;

    let body = res.text().await?;
    let body2 = res2.text().await?;

    // println!("Body:\n{}", body);

    // HTML Parse
    let document = Html::parse_document(&body);
    let document2 = Html::parse_document(&body2);

    // 현재 온도
    let current_temp = Selector::parse("div.temperature_text").unwrap();
    let current_temp_element = document.select(&current_temp).next().unwrap();
    let current_temp = current_temp_element.text().collect::<Vec<_>>()[2]
        .trim()
        .to_string()
        + "도"; // "28도"

    // 최저 온도
    let min_temp = Selector::parse("span.lowest").unwrap();
    let min_temp_element = document.select(&min_temp).next().unwrap();
    let min_temp = min_temp_element.text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    let min_temp = min_temp.replace('°', "") + "도";

    // 최고 온도
    let max_temp = Selector::parse("span.highest").unwrap();
    let max_temp_element = document.select(&max_temp).next().unwrap();
    let max_temp = max_temp_element.text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    let max_temp = max_temp.replace('°', "") + "도";

    // 현재 날씨
    let current_status = Selector::parse("span.weather.before_slash").unwrap();
    let current_status_element = document.select(&current_status).next().unwrap();
    let current_status = current_status_element.text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 맑음

    // 일몰
    let sunset = Selector::parse("li.item_today.type_sun > a > span").unwrap();
    let sunset_element = document.select(&sunset).next().unwrap();
    let sunset = sunset_element.text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 19:22

    // 오전/오후 강수확률
    let rain_drops = Selector::parse("span.rainfall").unwrap();
    let mut rain_elements = document.select(&rain_drops);

    let rain_day = rain_elements.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 13%
    let rain_night = rain_elements.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 0%

    // 미세/초미세
    let four_stats_selector = Selector::parse("span.txt").unwrap();
    let mut four_elements = document.select(&four_stats_selector);

    let fine_dust = four_elements.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 나쁨
    let ultra_dust = four_elements.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 좋음

    // 자외선
    let uv = four_elements.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // 매우높음

    // 날씨 아이콘
    let icon = Selector::parse("div.summary_img > i").unwrap();
    let icon_element = document2.select(&icon).next().unwrap();
    let mut icon = icon_element.value().attr("data-ico").unwrap().to_string();
    let icon_classes = icon_element.value().attr("class").unwrap();
    if icon_classes.contains("night") {
        icon += "_night";
    }

    // struct Weather init
    let weather = Weather {
        current_temp,
        min_temp,
        max_temp,
        current_status,
        sunset,
        rain_day,
        rain_night,
        fine_dust,
        ultra_dust,
        uv,
        icon,
    };
    // let mut weather: Weather = Default::default();
    // weather.current_temp = current_temp;
    // weather.min_temp = min_temp;
    // weather.max_temp = max_temp;
    // weather.current_status = current_status;
    // weather.sunset = sunset;
    // weather.rain_day = rain_day;
    // weather.rain_night = rain_night;
    // weather.fine_dust = fine_dust;
    // weather.ultra_dust = ultra_dust;
    // weather.uv = uv;
    // // weather.windchill = windchill;  // 체감온도 추가됨 (2022.05.04)
    // weather.icon = format!(
    //     "https://raw.githubusercontent.com/Alfex4936/KakaoChatBot-Golang/main/imgs/{}.png?raw=true",
    //     icon
    // );

    println!("{:?}", weather);
    Ok(weather)
}

pub async fn library_parse() -> Result<Library, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(2))
        .build()?;

    // header 없이 보내면 404
    let res = client
        .get(AJOU_LIBRARY)
        .header(USER_AGENT, MY_USER_AGENT)
        .send()
        .await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    let library: Library = serde_json::from_str(&body).unwrap();
    Ok(library)
}

pub async fn people_parse(keyword: &str) -> Result<People, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(2))
        .build()?;

    let mut map = HashMap::new();
    map.insert("keyword", keyword);

    let res = client
        .post(AJOU_PEOPLE)
        .header(USER_AGENT, MY_USER_AGENT)
        .json(&map)
        .send()
        .await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    let people: People = serde_json::from_str(&body).unwrap();
    Ok(people)
}

pub async fn meal_parse(date: String) -> Result<Meal, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(2))
        .build()?;

    let mut map = HashMap::new();
    map.insert("categoryId", "221"); // 221: 교직원, 교직원밖에 정보 없음
    map.insert("yyyymmdd", &date);

    // header 없이 보내면 404
    let res = client
        .post(AJOU_MEAL)
        .header(USER_AGENT, MY_USER_AGENT)
        .json(&map)
        .send()
        .await?;

    let body = res.text().await?;

    // println!("Body:\n{}", body);

    let mut meal: Meal = serde_json::from_str(&body).unwrap();

    if meal.data.breakfast.is_none() && meal.data.lunch.is_none() && meal.data.dinner.is_none() {
        meal.msg_code = "empty".to_string();
    }

    if let Some(ref mut meal) = meal.data.breakfast {
        if meal.is_empty() {
            *meal = "없음".to_string();
        }
    }
    if let Some(ref mut meal) = meal.data.lunch {
        if meal.is_empty() {
            *meal = "없음".to_string();
        } else {
            *meal = str::replace(meal, "<br>", "\n");
        }
    }
    if let Some(ref mut meal) = meal.data.dinner {
        if meal.is_empty() {
            *meal = "없음".to_string();
        } else {
            *meal = str::replace(meal, "<br>", "\n");
        }
    }

    Ok(meal)
}
