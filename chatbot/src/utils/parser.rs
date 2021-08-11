use crate::db::models::{Library, Notice, People, Weather};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";
pub const NAVER_WEATHER: &str = "https://weather.naver.com/today/02117530?cpName=ACCUWEATHER"; // 아주대 지역 날씨
pub const AJOU_LIBRARY: &str = env!("AJOU_LIBRARY"); // 아주대 중앙 도서관
pub const AJOU_PEOPLE: &str = env!("AJOU_PEOPLE"); // 아주대 인물 검색

pub async fn notice_parse(_nums: Option<usize>) -> Result<Vec<Notice>, reqwest::Error> {
    let mut ajou =
        "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=list&article.offset=0&articleLimit="
            .to_string();

    let nums_int = _nums.unwrap_or(5);
    let nums_str = nums_int.to_string();

    ajou.push_str(&nums_str);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // header 없이 보내면 404
    let res = client.get(ajou).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    // HTML Parse
    let document = Html::parse_document(&body);
    let a_selector = Selector::parse("a").unwrap();

    // Notice has id, title, date, link, writer
    let ids = Selector::parse("td.b-num-box").unwrap();
    let titles = Selector::parse("div.b-title-box").unwrap(); // includes links
    let dates = Selector::parse("span.b-date").unwrap();
    let writers = Selector::parse("span.b-writer").unwrap();

    let mut notices = vec![Notice::default(); nums_int];

    let mut id_elements = document.select(&ids);
    let mut title_elements = document.select(&titles);
    let mut date_elements = document.select(&dates);
    let mut writer_elements = document.select(&writers);

    // struct Notice
    for notice in notices.iter_mut() {
        let id_element = id_elements.next().unwrap();
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

        (*notice).id = id;
        (*notice).title = title;
        (*notice).link = link;
        (*notice).date = date;
        (*notice).writer = writer;
    }
    // println!("{:?}", notices);
    Ok(notices)
}

pub async fn weather_parse() -> Result<Weather, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // header 없이 보내면 404
    let res = client.get(NAVER_WEATHER).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    // HTML Parse
    let mut weather: Weather = Default::default();

    let document = Html::parse_document(&body);

    // Notice has id, title, date, link, writer
    let current_temp = Selector::parse("strong.current").unwrap();
    let current_stat = Selector::parse("span.weather").unwrap();
    let temps = Selector::parse("span.data").unwrap();
    let rains = Selector::parse("span.rainfall").unwrap();
    let stats = Selector::parse("em.level_text").unwrap();
    let icon = Selector::parse("div.today_weather > i").unwrap();
    let wind_chill = Selector::parse("div.weather_area > dl > dd:nth-child(6)").unwrap();

    let current_temp_element = document.select(&current_temp).next().unwrap();
    let current_stat_element = document.select(&current_stat).next().unwrap();
    let mut temps_element = document.select(&temps);
    let mut stats_element = document.select(&stats);
    let mut rains_element = document.select(&rains);
    let icon_element = document.select(&icon).next().unwrap();
    let wind_chill_element = document.select(&wind_chill).next().unwrap();

    let current_temp = current_temp_element.text().collect::<Vec<_>>()[1]
        .trim()
        .to_string()
        + "도"; // "28도"

    let current_stat = current_stat_element.text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // "구름조금"

    let day_temp = temps_element.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string()
        + "도"; // "최고 온도"
    let night_temp = temps_element.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string()
        + "도"; // "최저 온도"

    let fine_dust = stats_element.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // "보통"
    let ultra_dust = stats_element.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // "나쁨"
    let uv = stats_element.next().unwrap().text().collect::<Vec<_>>()[0]
        .trim()
        .to_string(); // "높음"

    let day_rain = rains_element.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string(); // "55%"
    let night_rain = rains_element.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string(); // "8%"

    let wind_chill = wind_chill_element.text().collect::<Vec<_>>()[0].trim(); // "33"
    let wind_chill = wind_chill.replace("°", "") + "도";

    let mut icon = icon_element.value().attr("data-ico").unwrap().to_string();
    let icon_classes = icon_element.value().attr("class").unwrap();

    // struct Weather init
    weather.current_temp = current_temp;
    weather.wind_chill = wind_chill;
    weather.current_status = current_stat;
    weather.max_temp = day_temp;
    weather.min_temp = night_temp;
    weather.rain_day = day_rain;
    weather.rain_night = night_rain;
    weather.fine_dust = fine_dust;
    weather.ultra_dust = ultra_dust;
    weather.uv = uv;

    if icon_classes.contains("night") {
        icon += "_night";
    }

    weather.icon = format!(
        "https://raw.githubusercontent.com/Alfex4936/KakaoChatBot-Golang/main/imgs/{}.png?raw=true",
        icon
    );

    Ok(weather)
}

pub async fn library_parse() -> Result<Library, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // header 없이 보내면 404
    let res = client.get(AJOU_LIBRARY).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    let library: Library = serde_json::from_str(&body).unwrap();
    Ok(library)
}

pub async fn people_parse(keyword: &String) -> Result<People, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut map = HashMap::new();
    map.insert("keyword", keyword);

    // header 없이 보내면 404
    let res = client.post(AJOU_PEOPLE).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").json(&map).send().await?;
    let body = res.text().await?;

    // println!("Body:\n{}", body);

    let people: People = serde_json::from_str(&body).unwrap();
    Ok(people)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn weather_test() {
        let weather = weather_parse().await.unwrap();
        println!("{:#?}", weather);
    }

    #[actix_rt::test]
    async fn library_test() {
        let library = library_parse().await.unwrap();
        println!("{:#?}", library);
    }
}