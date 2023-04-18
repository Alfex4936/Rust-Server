#![allow(dead_code)]

// use crate::db::schema::ajou_notices;
// use crate::db::schema::ajou_sched;
// use diesel::prelude::*;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notice {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    // pub id: i32,
    pub start_date: String,
    pub end_date: String,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct Weather {
    pub max_temp: String,
    pub min_temp: String,
    pub tmrw_min_temp: String,
    pub tmrw_max_temp: String,
    pub current_temp: String,
    pub current_status: String,
    // pub wind_chill: String, // 체감온도
    pub rain_day: String,
    pub rain_night: String,
    pub fine_dust: String,
    pub ultra_dust: String,
    pub sunset: String,
    pub uv: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub msg_code: String,
    #[serde(rename = "p018Text")]
    pub data: MealContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 63 기숙사, 220 학생, 221 교직원
pub struct MealContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakfast: Option<String>, //아침
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lunch: Option<String>, // 점심
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dinner: Option<String>, // 저녁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snack_bar: Option<String>, // 분식
    #[serde(rename = "menuDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>, // 날짜
    #[serde(rename = "restaurantNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // 식당 이름 (교직원식당(생활관 2층))
}

#[derive(Debug, Deserialize)]
pub struct Library {
    code: String,
    pub data: LibraryData,
    message: String,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryData {
    pub list: Vec<LibraryList>,
    total_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryList {
    id: u32,
    pub active_total: u32,
    pub available: u32,
    branch_group: Value,
    disable_period: Value,
    is_active: bool,
    is_reservable: bool,
    pub name: String,
    note: Value,
    occupied: u32,
    room_type_id: u32,
    total: u32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct People {
    msg_code: String,
    pub phone_number: Vec<PeopleList>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeopleList {
    // 전부 None일 수도...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_no: Option<String>, // 전화번호: 031-219-"1234"
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_cd: Option<String>, // "DS01234657"
    #[serde(skip_serializing_if = "Option::is_none")]
    user_no: Option<String>, // "201900000"
    #[serde(skip_serializing_if = "Option::is_none")]
    buss_nm: Option<String>, // 업무명: "XXX학과(공학인증)"
    #[serde(skip_serializing_if = "Option::is_none")]
    mdf_line_no: Option<String>, // "289"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_nm: Option<String>, // 부서명: "정보통신대학교학팀(팔달관 777-1)"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, // 이메일: "example@ajou.ac.kr"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kor_nm: Option<String>, // 이름(신분): "이름1(직원)" | "이름2(교원)"
}

// Course
#[derive(Debug, Deserialize, Default)]
pub struct VariableList {
    #[serde(rename = "ErrorMsg")]
    error_msg: String,
    #[serde(rename = "ErrorCode")]
    error_code: String,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Course {
    #[serde(rename(deserialize = "tm", serialize = "duration"))]
    duration: Option<f32>,

    #[serde(rename(deserialize = "submattFgEngNm", serialize = "course_type"))]
    course_type: Option<String>,

    #[serde(rename(deserialize = "ltTmEngNm", serialize = "class_time"))]
    class_time: String,

    #[serde(rename(deserialize = "clssNo", serialize = "class_number"))]
    class_number: String,

    #[serde(
        rename(deserialize = "rcomShyrCdNm", serialize = "recommended_year"),
        default
    )]
    recommended_year: Option<String>,

    #[serde(rename(deserialize = "sustLsnFgNm", serialize = "course_category"))]
    course_category: String,

    #[serde(rename(
        deserialize = "maLecturerEmplNo",
        serialize = "main_lecturer_employee_number"
    ))]
    main_lecturer_employee_number: String,

    #[serde(rename(
        deserialize = "abeekInspPracPntCnt",
        serialize = "abeek_practical_points"
    ))]
    abeek_practical_points: Option<f32>,

    #[serde(rename(deserialize = "fileNm", serialize = "file_name"), default)]
    file_name: Option<String>,

    #[serde(rename(deserialize = "maLecturerEmplNm", serialize = "main_lecturer_name"))]
    main_lecturer_name: String,

    #[serde(rename(deserialize = "sustLsnFgEngNm", serialize = "course_category_english"))]
    course_category_english: String,

    #[serde(
        rename(deserialize = "mjCdEngNm", serialize = "major_code_english"),
        default
    )]
    major_code_english: Option<String>,

    #[serde(rename(deserialize = "sustCd", serialize = "department_code"))]
    department_code: String,

    #[serde(rename(deserialize = "planInputYn", serialize = "plan_input_status"))]
    plan_input_status: String,

    #[serde(rename(deserialize = "filePath", serialize = "file_path"), default)]
    file_path: Option<String>,

    #[serde(rename(
        deserialize = "abeekTheoPntCnt",
        serialize = "abeek_theoretical_points"
    ))]
    abeek_theoretical_points: Option<f32>,

    #[serde(rename(deserialize = "ltRoomEngNm", serialize = "classroom_english"))]
    classroom_english: String,

    #[serde(rename(deserialize = "emplNo", serialize = "employee_number"))]
    employee_number: String,

    #[serde(
        rename(deserialize = "sustCdEngNm", serialize = "department_english"),
        default
    )]
    department_english: Option<String>,

    #[serde(rename(deserialize = "submattFgNm", serialize = "course_type_korean"))]
    course_type_korean: String,

    #[serde(rename(deserialize = "sbjtCd", serialize = "subject_code"))]
    subject_code: String,

    #[serde(rename(deserialize = "mainOpenLtNo", serialize = "main_open_course_number"))]
    main_open_course_number: String,

    #[serde(rename(deserialize = "mjCd", serialize = "major_code"))]
    major_code: String,

    #[serde(rename(deserialize = "mjCdNm", serialize = "major_name"), default)]
    major_name: Option<String>,

    #[serde(rename(deserialize = "ltRoomNm", serialize = "classroom"))]
    classroom: String,

    #[serde(rename(deserialize = "abeePnt", serialize = "abee_point"))]
    abee_point: Option<f32>,

    #[serde(rename(deserialize = "shtmCd", serialize = "semester_code"))]
    semester_code: String,

    #[serde(rename(
        deserialize = "maLecturerEmplEngNm",
        serialize = "main_lecturer_english_name"
    ))]
    main_lecturer_english_name: Option<String>,

    #[serde(rename(deserialize = "sustLsnFg", serialize = "course_category_code"))]
    course_category_code: String,

    #[serde(rename(deserialize = "openLtNo", serialize = "open_course_number"))]
    open_course_number: String,

    #[serde(rename(deserialize = "orgLangLtYn", serialize = "original_language_course"))]
    original_language_course: Option<String>,

    #[serde(rename(deserialize = "cqiYn", serialize = "cqi_status"))]
    cqi_status: String,

    #[serde(rename(deserialize = "lsnApprDetailPop", serialize = "course_evaluation"))]
    course_evaluation: String,

    #[serde(rename(deserialize = "shtmNm", serialize = "semester_name"))]
    semester_name: String,

    #[serde(rename(deserialize = "yy", serialize = "year"))]
    year: String,

    #[serde(rename(deserialize = "sustCdNm", serialize = "department_name"))]
    department_name: Option<String>,

    #[serde(
        rename(deserialize = "engGrdFgNm", serialize = "english_grade_type"),
        default
    )]
    english_grade_type: Option<String>,

    #[serde(rename(deserialize = "abeekDgnPntCnt", serialize = "abeek_design_points"))]
    abeek_design_points: Option<f32>,

    #[serde(rename(deserialize = "abeekYn", serialize = "abeek_status"))]
    abeek_status: String,

    #[serde(rename(deserialize = "fileFg", serialize = "file_status"))]
    file_status: String,

    #[serde(rename(deserialize = "sbjtKorNm", serialize = "subject_korean_name"))]
    subject_korean_name: String,

    #[serde(
        rename(
            deserialize = "lsnPdocMngtClssYn",
            serialize = "lesson_document_management_class"
        ),
        default
    )]
    lesson_document_management_class: Option<String>,

    #[serde(rename(deserialize = "ltTmNm", serialize = "class_time_korean"))]
    class_time_korean: String,

    #[serde(
        rename(deserialize = "rcomShyrCd", serialize = "recommended_year_code"),
        default
    )]
    recommended_year_code: Option<String>,

    #[serde(rename(deserialize = "tlsnNo", serialize = "lesson_number"))]
    lesson_number: String,

    #[serde(rename(deserialize = "apprUnAdptYn", serialize = "approved_unadapted"))]
    approved_unadapted: String,

    #[serde(rename(deserialize = "pnt", serialize = "credit_points"))]
    credit_points: f32,

    #[serde(rename(deserialize = "sbjtId", serialize = "subject_id"))]
    pub subject_id: String,

    #[serde(rename(deserialize = "sbjtEngNm", serialize = "subject_english_name"))]
    subject_english_name: String,

    #[serde(rename(deserialize = "coopOpenLtYn", serialize = "cooperative_open_course"))]
    cooperative_open_course: String,

    #[serde(
        rename(deserialize = "coopLt", serialize = "cooperative_course"),
        default
    )]
    cooperative_course: Option<String>,

    #[serde(rename(deserialize = "rowStatus", serialize = "row_status"))]
    row_status: i32,

    #[serde(
        rename(deserialize = "ltFgNm", serialize = "lecture_type_name"),
        default
    )]
    lecture_type_name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct DatasetList {
    #[serde(rename = "DS_COUR120")]
    pub ds_cour120: Vec<Course>,
}

#[derive(Debug, Deserialize, Default)]

pub struct CourseResp {
    #[serde(rename = "VariableList")]
    pub var_list: VariableList,
    #[serde(rename = "DatasetList")]
    pub data_list: DatasetList,
}
