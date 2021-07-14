diesel::table! {
    ajou_sched (id) {
        id -> Integer,
        start_date -> Varchar,
        end_date -> Varchar,
        content -> Varchar,
    }
}
