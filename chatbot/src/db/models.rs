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
#[derive(Debug, Deserialize)]
pub struct VariableList {
    #[serde(rename = "ErrorMsg")]
    error_msg: String,
    #[serde(rename = "ErrorCode")]
    error_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Course {
    #[serde(rename = "tm")]
    duration: f32,

    #[serde(rename = "submattFgEngNm")]
    course_type: Option<String>,

    #[serde(rename = "ltTmEngNm")]
    class_time: String,

    #[serde(rename = "clssNo")]
    class_number: String,

    #[serde(rename = "rcomShyrCdNm")]
    recommended_year: Option<String>,

    #[serde(rename = "sustLsnFgNm")]
    course_category: String,

    #[serde(rename = "maLecturerEmplNo")]
    main_lecturer_employee_number: String,

    #[serde(rename = "abeekInspPracPntCnt")]
    abeek_practical_points: f32,

    #[serde(rename = "fileNm")]
    file_name: Option<String>,

    #[serde(rename = "maLecturerEmplNm")]
    main_lecturer_name: String,

    #[serde(rename = "sustLsnFgEngNm")]
    course_category_english: String,

    #[serde(rename = "mjCdEngNm")]
    major_code_english: Option<String>,

    #[serde(rename = "sustCd")]
    department_code: String,

    #[serde(rename = "planInputYn")]
    plan_input_status: String,

    #[serde(rename = "filePath")]
    file_path: Option<String>,

    #[serde(rename = "abeekTheoPntCnt")]
    abeek_theoretical_points: f32,

    #[serde(rename = "ltRoomEngNm")]
    classroom_english: String,

    #[serde(rename = "emplNo")]
    employee_number: String,

    #[serde(rename = "sustCdEngNm")]
    department_english: Option<String>,

    #[serde(rename = "submattFgNm")]
    course_type_korean: String,

    #[serde(rename = "sbjtCd")]
    subject_code: String,

    #[serde(rename = "mainOpenLtNo")]
    main_open_course_number: String,

    #[serde(rename = "mjCd")]
    major_code: String,

    #[serde(rename = "mjCdNm")]
    major_name: Option<String>,

    #[serde(rename = "ltRoomNm")]
    classroom: String,

    #[serde(rename = "abeePnt")]
    abee_point: Option<f32>,

    #[serde(rename = "shtmCd")]
    semester_code: String,

    #[serde(rename = "maLecturerEmplEngNm")]
    main_lecturer_english_name: Option<String>,

    #[serde(rename = "sustLsnFg")]
    course_category_code: String,

    #[serde(rename = "openLtNo")]
    open_course_number: String,

    #[serde(rename = "orgLangLtYn")]
    original_language_course: Option<String>,

    #[serde(rename = "cqiYn")]
    cqi_status: String,

    #[serde(rename = "lsnApprDetailPop")]
    course_evaluation: String,

    #[serde(rename = "shtmNm")]
    semester_name: String,

    #[serde(rename = "yy")]
    year: String,

    #[serde(rename = "sustCdNm")]
    department_name: Option<String>,

    #[serde(rename = "engGrdFgNm")]
    english_grade_type: Option<String>,

    #[serde(rename = "abeekDgnPntCnt")]
    abeek_design_points: f32,

    #[serde(rename = "abeekYn")]
    abeek_status: String,

    #[serde(rename = "fileFg")]
    file_status: String,

    #[serde(rename = "sbjtKorNm")]
    subject_korean_name: String,

    #[serde(rename = "lsnPdocMngtClssYn")]
    lesson_document_management_class: Option<String>,

    #[serde(rename = "ltTmNm")]
    class_time_korean: String,

    #[serde(rename = "rcomShyrCd")]
    recommended_year_code: Option<String>,

    #[serde(rename = "tlsnNo")]
    lesson_number: String,

    #[serde(rename = "apprUnAdptYn")]
    approved_unadapted: String,

    #[serde(rename = "pnt")]
    credit_points: f32,

    #[serde(rename = "sbjtId")]
    subject_id: String,

    #[serde(rename = "sbjtEngNm")]
    subject_english_name: String,

    #[serde(rename = "coopOpenLtYn")]
    cooperative_open_course: String,

    #[serde(rename = "coopLt")]
    cooperative_course: Option<String>,

    #[serde(rename = "rowStatus")]
    row_status: i32,

    #[serde(rename = "ltFgNm")]
    lecture_type_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatasetList {
    #[serde(rename = "DS_COUR120")]
    pub ds_cour120: Vec<Course>,
}

#[derive(Debug, Deserialize)]

pub struct CourseResp {
    #[serde(rename = "VariableList")]
    pub var_list: VariableList,
    #[serde(rename = "DatasetList")]
    pub data_list: DatasetList,
}
