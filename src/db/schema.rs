// @generated automatically by Diesel CLI.

diesel::table! {
    articles_cache (id) {
        id -> Nullable<Integer>,
        hash -> Text,
        html -> Text,
    }
}

diesel::table! {
    articles (id) {
        id -> Integer,
        src_file_name -> Text,
        dst_file_name -> Text,
        title -> Nullable<Text>,
        modification_date -> Nullable<Timestamp>,
        summary -> Nullable<Text>,
        series -> Nullable<Text>,
        draft -> Nullable<Bool>,
        special_page -> Nullable<Bool>,
        timeline -> Nullable<Bool>,
        anchorjs -> Nullable<Bool>,
        tocify -> Nullable<Bool>,
        live_updates -> Nullable<Bool>,
    }
}

diesel::table! {
    series (id) {
        id -> Nullable<Integer>,
        name -> Text,
        article_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
        article_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        level -> Text,
    }
}

diesel::joinable!(series -> articles (article_id));
diesel::joinable!(tags -> articles (article_id));

diesel::allow_tables_to_appear_in_same_query!(articles_cache, articles, series, tags, users,);
