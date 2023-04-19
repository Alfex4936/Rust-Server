use crate::db::models::{CourseResp, Library, Meal, Notice, People, Weather};
use crate::{MONGO_URL, MY_USER_AGENT};
use anyhow::{anyhow, Result};
use cached::proc_macro::cached;
use cached::SizedCache;
use csv::ReaderBuilder;
use lazy_static::lazy_static;
use mongodb::{
    bson::{doc, to_bson},
    options::{ClientOptions, UpdateOptions},
    Client,
};
use once_cell::sync::OnceCell;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE, USER_AGENT};
use scraper::{Html, Selector};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";
pub const NAVER_WEATHER: &str = "https://m.search.naver.com/search.naver?sm=tab_hty.top&where=nexearch&query=%EB%82%A0%EC%94%A8+%EB%A7%A4%ED%83%843%EB%8F%99&oquery=%EB%82%A0%EC%94%A8"; // 아주대 지역 날씨
pub const NAVER_WEATHER_ICON: &str = "https://weather.naver.com/today/02117530?cpName=ACCUWEATHER"; // 아주대 지역 날씨는
pub const AJOU_LIBRARY: &str = env!("AJOU_LIBRARY"); // 아주대 중앙 도서관
pub const AJOU_PEOPLE: &str = env!("AJOU_PEOPLE"); // 아주대 인물 검색
pub const AJOU_MEAL: &str = env!("AJOU_MEAL"); // 아주대 학식
pub const AJOU_COURSE: &str = env!("AJOU_COURSE");
const DEFAULT_NUM_ARTICLES: usize = 7;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(5))
        .user_agent(MY_USER_AGENT)
        .build()
        .unwrap();
    static ref INIT: OnceCell<Mutex<()>> = OnceCell::new();
    static ref GLOBAL_INDEX: Mutex<Arc<BTreeMap<String, Vec<Course>>>> =
        Mutex::new(Arc::new(BTreeMap::new()));
}

fn get_query(query_option: &str) -> Cow<'_, str> {
    match query_option {
        "ajou" => "?mode=list&article.offset=0&articleLimit=".into(),
        "category" => "?mode=list&articleLimit=5&srCategoryId=".into(),
        _ => format!(
            "?mode=list&srSearchKey=&srSearchVal={}&article.offset=0&articleLimit=",
            query_option
        )
        .into(),
    }
}

pub async fn notice_parse(
    query_option: &str,
    _nums: Option<usize>,
) -> Result<Vec<Notice>, reqwest::Error> {
    let query = get_query(query_option);
    let nums_int = _nums.unwrap_or(DEFAULT_NUM_ARTICLES);

    let url = [AJOU_LINK, &query, &nums_int.to_string()].concat();

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

    // HTML Parse
    let document = Html::parse_document(&body);
    let a_selector = Selector::parse("a").unwrap();

    let ids = Selector::parse("td.b-num-box").unwrap();
    let cates = Selector::parse("span.b-cate").unwrap();
    let titles = Selector::parse("div.b-title-box").unwrap();
    let dates = Selector::parse("span.b-date").unwrap();
    let writers = Selector::parse("span.b-writer").unwrap();

    let id_elements = document.select(&ids);
    let mut cate_elements = document.select(&cates);
    let mut title_elements = document.select(&titles);
    let mut date_elements = document.select(&dates);
    let mut writer_elements = document.select(&writers);

    let notices: Vec<Notice> = id_elements
        .filter_map(|id_element| {
            let date = date_elements.next()?.text().next()?.trim().to_string();
            let writer = writer_elements
                .next()?
                .text()
                .next()
                .unwrap_or("알 수 없음")
                .trim()
                .to_string();
            let category = cate_elements.next()?.text().next()?.trim().to_string();
            let inner_a = title_elements.next()?.select(&a_selector).next()?;
            let id = id_element.text().next()?.trim().parse::<i32>().ok()?;

            let mut title = inner_a.value().attr("title")?.to_string();
            let link = format!("{}{}", AJOU_LINK, inner_a.value().attr("href").unwrap());

            let dup = format!("[{}]", writer);
            if title.contains(&dup) {
                title = title.replace(&dup, "");
            }

            title = title
                .replace(" 자세히 보기", "")
                .replace("(재공지)", "")
                .trim()
                .to_string();

            Some(Notice {
                id,
                category,
                title,
                link,
                date,
                writer,
            })
        })
        .collect();

    Ok(notices)
}

pub async fn weather_parse() -> Result<Weather, reqwest::Error> {
    let res = CLIENT.get(NAVER_WEATHER).send().await?;
    let res2 = CLIENT.get(NAVER_WEATHER_ICON).send().await?;

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
    let mut min_temp_elements = document.select(&min_temp);
    let min_temp = min_temp_elements.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    let min_temp = min_temp.replace('°', "") + "도";

    // 최고 온도
    let max_temp = Selector::parse("span.highest").unwrap();
    let mut max_temp_elements = document.select(&max_temp);
    let max_temp = max_temp_elements.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    let max_temp = max_temp.replace('°', "") + "도";

    // Tmrw
    let tmrw_max_temp = max_temp_elements.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    let tmrw_min_temp = min_temp_elements.next().unwrap().text().collect::<Vec<_>>()[1]
        .trim()
        .to_string();

    let tmrw_max_temp = tmrw_max_temp.replace('°', "") + "도";
    let tmrw_min_temp = tmrw_min_temp.replace('°', "") + "도";

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

    icon = format!(
        "https://raw.githubusercontent.com/Alfex4936/KakaoChatBot-Golang/main/imgs/{}.png?raw=true",
        icon
    );

    // struct Weather init
    let weather = Weather {
        current_temp,
        min_temp,
        max_temp,
        tmrw_min_temp,
        tmrw_max_temp,
        current_status,
        sunset,
        rain_day,
        rain_night,
        fine_dust,
        ultra_dust,
        uv,
        icon,
    };
    Ok(weather)
}

pub async fn library_parse() -> Result<Library, reqwest::Error> {
    // header 없이 보내면 404
    let res = CLIENT
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
    let mut map = HashMap::new();
    map.insert("keyword", keyword);

    let res = CLIENT
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

pub async fn meal_parse(date: String, res: u8) -> Result<Meal, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("categoryId", res.to_string()); // 221: 교직원, 63, 기숙사 식당
    map.insert("yyyymmdd", date);

    // header 없이 보내면 404
    let res = CLIENT
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Course {
    duration: Option<f32>,

    course_type: Option<String>,

    class_time: String,

    class_number: String,

    recommended_year: Option<String>,

    course_category: String,

    main_lecturer_employee_number: String,

    abeek_practical_points: f32,

    file_name: Option<String>,

    pub main_lecturer_name: String,

    course_category_english: String,

    major_code_english: Option<String>,

    department_code: String,

    plan_input_status: String,

    file_path: Option<String>,

    abeek_theoretical_points: f32,

    classroom_english: String,

    employee_number: String,

    department_english: Option<String>,

    course_type_korean: String,

    subject_code: String,

    main_open_course_number: String,

    major_code: String,

    major_name: Option<String>,

    classroom: String,

    abee_point: Option<f32>,

    semester_code: String,

    main_lecturer_english_name: Option<String>,

    course_category_code: String,

    open_course_number: String,

    original_language_course: Option<String>,

    cqi_status: String,

    course_evaluation: String,

    semester_name: String,

    year: String,

    department_name: Option<String>,

    english_grade_type: Option<String>,

    abeek_design_points: Option<f32>,

    abeek_status: String,

    file_status: String,

    pub subject_korean_name: String,

    lesson_document_management_class: Option<String>,

    pub class_time_korean: String,

    recommended_year_code: Option<String>,

    lesson_number: String,

    approved_unadapted: String,

    credit_points: Option<f32>,

    pub subject_id: String,

    pub subject_english_name: String,

    cooperative_open_course: String,

    cooperative_course: Option<String>,

    row_status: i32,

    lecture_type_name: Option<String>,

    pub unique_id: String,
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id.eq(&other.unique_id)
    }
}

impl Eq for Course {}

impl Hash for Course {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_id.hash(state);
    }
}

async fn load_csv_data(file_path: &str) -> Result<Vec<Course>> {
    let mut buf = Vec::new();
    let mut file = File::open(file_path).await?;
    file.read_to_end(&mut buf).await?;

    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .flexible(true)
        .from_reader(buf.as_slice());

    let mut courses: Vec<Course> = vec![];
    for result in reader.deserialize() {
        let record: Course = result?;
        courses.push(record);
    }

    Ok(courses)
}

async fn index_from_courses(courses: &[Course]) {
    let mut index = GLOBAL_INDEX.lock().await;
    let index_inner = Arc::make_mut(&mut index);
    let mut added_ids: HashSet<String> = HashSet::new();

    for course in courses {
        if added_ids.contains(&course.subject_id) {
            continue;
        }

        let keys = vec![
            course.subject_korean_name.to_lowercase(),
            course.subject_english_name.to_lowercase(),
            course.main_lecturer_name.to_lowercase(),
        ];

        for key in keys {
            index_inner
                .entry(key)
                .or_insert_with(Vec::new)
                .push(course.clone());
        }

        added_ids.insert(course.unique_id.clone());
    }
}

#[cached(
    type = "SizedCache<String, Vec<Course>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ query.to_lowercase() }"#
)]
async fn search(query: &str) -> Vec<Course> {
    let query = query.to_lowercase();
    let index = GLOBAL_INDEX.lock().await;
    let index_clone = Arc::clone(&index);

    let mut results: Vec<Course> = Vec::new();
    let mut added_ids: HashSet<String> = HashSet::new();

    for courses_list in index_clone.values() {
        for course in courses_list {
            if !added_ids.contains(&course.unique_id)
                && (course.subject_korean_name.to_lowercase().contains(&query)
                    || course.subject_english_name.to_lowercase().contains(&query)
                    || course.main_lecturer_name.to_lowercase().contains(&query))
            {
                results.push(course.clone());
                added_ids.insert(course.unique_id.clone());
            }
        }
    }

    results
}

pub async fn load_courses(query: &str) -> Result<Vec<Course>> {
    let init_lock = INIT.get_or_init(|| Mutex::new(()));
    {
        let _guard = init_lock.lock().await;
        if GLOBAL_INDEX.lock().await.is_empty() {
            let exe_path = env::current_exe()?;
            let exe_dir = exe_path.parent().unwrap();
            let csv_path = exe_dir.join("course.csv");
            let csv_path_str = csv_path.to_str().unwrap();

            let courses = load_csv_data(csv_path_str).await?;
            index_from_courses(&courses).await;

            // println!("{:?}", GLOBAL_INDEX.lock().await);
        }
    }

    let mut matching_courses = search(query).await;
    matching_courses.truncate(10);
    Ok(matching_courses)
}

pub async fn course_parse(str_submatt_fg: &str) -> Result<CourseResp, reqwest::Error> {
    let payload = serde_json::json!({
        "url": "uni/uni/cour/lssn/findCourLecturePlanDocumentReg.action",
        "param": {
            "strYy": "2023",
            "strShtmCd": "U0002001",
            "strSubmattFg": str_submatt_fg,
            "strSustcd": "",
            "strMjCd": "",
            "strSubmattFldFg": "",
            "strCoopOpenYn": "공동개설"
        }
    });

    // Create a HeaderMap and insert the necessary headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(COOKIE, HeaderValue::from_static(""));

    let res = CLIENT
        .post(AJOU_COURSE)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let body = res.text().await?;

    let courses: CourseResp = serde_json::from_str(&body).unwrap();
    Ok(courses)
}

fn get_collection_name(category: &str) -> String {
    format!("course_2023-1_{}", category)
}

pub async fn insert_courses_to_mongodb(category: &str, courses: Vec<Course>) -> Result<()> {
    let client_options = ClientOptions::parse(MONGO_URL).await?;
    let client = Client::with_options(client_options)?;

    let database_name = "ajou";
    let collection_name = get_collection_name(category);
    println!(
        "Inserting courses for {collection_name} (len: {})...",
        courses.len()
    );

    let db = client.database(database_name);
    let collection = db.collection::<Course>(&collection_name);

    for course in courses {
        // Assuming 'subject_code' is unique for each course
        let filter = doc! { "subject_code": &course.subject_id };

        let document = to_bson(&course)?
            .as_document()
            .ok_or_else(|| anyhow!("Error converting course to BSON document"))?
            .clone();

        let update = doc! { "$set": document };
        let options = UpdateOptions::builder().upsert(true).build();

        collection
            .update_one(filter, update, options)
            .await
            .map_err(|e| anyhow!("Error upserting course in MongoDB: {:?}", e))?;

        // println!("Updated documents: {:?}", result.upserted_id);
    }

    println!("Finished updating courses...");
    Ok(())
}
