#[macro_use]
extern crate criterion;
extern crate pprof;

use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";
pub const MY_USER_AGENT: &str = "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36";

#[derive(Clone, Default)]
pub struct Notice {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

pub fn notice_parse(
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

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // header 없이 보내면 404
    let res = client.get(url).header(USER_AGENT, MY_USER_AGENT).send()?;
    let body = res.text()?;

    // println!("Body:\n{}", body);

    // HTML Parse
    let document = Html::parse_document(&body);
    let a_selector = Selector::parse("a").unwrap();

    // Notice has id, title, date, link, writer
    let ids = Selector::parse("td.b-num-box").unwrap();
    let titles = Selector::parse("div.b-title-box").unwrap(); // includes links
    let dates = Selector::parse("span.b-date").unwrap();
    let writers = Selector::parse("span.b-writer").unwrap();

    // let mut notices = vec![Notice::default(); nums_int];
    let mut notices: Vec<Notice> = vec![];

    let mut id_elements = document.select(&ids);
    let mut title_elements = document.select(&titles);
    let mut date_elements = document.select(&dates);
    let mut writer_elements = document.select(&writers);

    // struct Notice
    while let Some(id_element) = id_elements.next() {
        // let id_element = match id_elements.next() {
        //     Some(item) => item,
        //     None => break,
        // }; // cant get id_elements length...

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

        let notice = Notice {
            id,
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
    Ok(notices)
}

fn bench_parse_notice(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10 {
            notice_parse("ajou", Some(100)).unwrap();
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_notice", bench_parse_notice);
}

criterion_main!(benches);

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(
            PProfProfiler::new(100, Output::Flamegraph(None))
        );
    targets = criterion_benchmark
}
