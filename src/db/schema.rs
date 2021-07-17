diesel::table! {
    ajou_sched (id) {
        id -> Integer,
        start_date -> Varchar,
        end_date -> Varchar,
        content -> Varchar,
    }
}

diesel::table! {
    ajou_notices (id) {
        id -> Integer,
        title -> Varchar,
        date -> Varchar,
        link -> Varchar,
        writer -> Varchar,
    }
}
