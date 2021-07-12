#[derive(Debug, Queryable)]
#[table_name = "ajou_sched"]
pub struct NewPost {
    pub content: String,
    pub start_date: String,
    pub end_date: String,
}
