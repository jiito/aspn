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
    hosts (id) {
        id -> Int4,
        ip_address -> Inet,
        user_token -> Varchar,
    }
}

diesel::table! {
    hosts_functions (function_id, host_id) {
        function_id -> Int4,
        host_id -> Int4,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(hosts_functions -> functions (function_id));
diesel::joinable!(hosts_functions -> hosts (host_id));

diesel::allow_tables_to_appear_in_same_query!(
    developers,
    functions,
    hosts,
    hosts_functions,
    projects,
);
