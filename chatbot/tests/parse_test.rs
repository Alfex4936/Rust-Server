extern crate rustserver;

use chrono::prelude::Local;

use rustserver::utils::parser::{
    library_parse, meal_parse, notice_parse, people_parse,
    weather_parse,
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
        assert_eq!(notices.len(), 5);
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

        let meal = meal_parse(today.to_owned(), 63).await.unwrap();
        // println!("{:#?}", meal);

        let text = format!(
            "점심: {}\n\n저녁: {}",
            meal.data.lunch.unwrap_or("없음".to_string()),
            meal.data.dinner.unwrap_or("없음".to_string())
        );
        println!("{text}");

        let meal = meal_parse(today.to_owned(), 221).await.unwrap();
        // println!("{:#?}", meal);

        let text = format!(
            "점심: {}\n\n저녁: {}",
            meal.data.lunch.unwrap_or("없음".to_string()),
            meal.data.dinner.unwrap_or("없음".to_string())
        );
        println!("{text}");
    }

    #[actix_rt::test]
    async fn course_test() {
        // let course = course_parse("U0209001").await.unwrap();
        // insert_courses_to_mongodb("전공과목", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 전공과목 전체

        // let course = course_parse("U0209002").await.unwrap();
        // insert_courses_to_mongodb("교양과목", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 교양과목 전체

        // let course = course_parse("U0209003").await.unwrap();
        // insert_courses_to_mongodb("기초과목", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 기초과목 공통

        // let course = course_parse("U0209004").await.unwrap();
        // insert_courses_to_mongodb("공학기초", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 공학기초 전체

        // let course = course_parse("U0209005").await.unwrap();
        // insert_courses_to_mongodb("영역별교양", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 영역별교양 전체

        // let course = course_parse("U0209006").await.unwrap();
        // insert_courses_to_mongodb("학점교류", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 학점교류 전체

        // let course = course_parse("U0209029").await.unwrap();
        // insert_courses_to_mongodb("일선과목", course.data_list.ds_cour120)
        //     .await
        //     .unwrap(); // 일선과목 전체
    }
}
