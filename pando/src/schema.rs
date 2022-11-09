// @generated automatically by Diesel CLI.

diesel::table! {
    developers (id) {
        id -> Int4,
        name -> Varchar,
        ip_address -> Inet,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    functions (id) {
        id -> Int4,
        ref_name -> Varchar,
        route -> Varchar,
        project_id -> Int4,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    developers,
    functions,
    projects,
);
