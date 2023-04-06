extern crate rustserver;

use chrono::prelude::Local;

use rustserver::utils::parser::{
    library_parse, meal_parse, notice_parse, people_parse, weather_parse,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn library_test() {
        let library = library_parse().await.unwrap();
        println!("{:#?}", library);
    }

    #[actix_rt::test]
    async fn notice_test() {
        let notices = notice_parse("ajou", Some(5)).await.unwrap();
        println!("{:#?}", notices);
    }

    #[actix_rt::test]
    async fn people_test() {
        let people = people_parse("아주").await.unwrap();
        for person in &people.phone_number {
            match &person.kor_nm {
                Some(name) => println!("{:?}", name),
                None => continue,
            }
        }
        // println!("{:#?}", people);
    }

    #[actix_rt::test]
    async fn weather_test() {
        let weather = weather_parse().await.unwrap();
        println!("{:#?}", weather);
    }

    #[actix_rt::test]
    async fn meal_test() {
        let today = Local::now().format("%Y%m%d").to_string(); // "20220910"
        println!("{today}");

        let meal = meal_parse(today).await.unwrap();
        // println!("{:#?}", meal);

        let text = format!(
            "점심: {}\n\n저녁: {}",
            meal.data.lunch.unwrap_or("없음".to_string()),
            meal.data.dinner.unwrap_or("없음".to_string())
        );
        println!("{text}");
    }
}
