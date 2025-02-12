// @generated automatically by Diesel CLI.

diesel::table! {
    article_tags (article_id, tag_id) {
        article_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    articles (id) {
        id -> Nullable<Integer>,
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
    cache (id) {
        id -> Nullable<Integer>,
        src_file_name -> Text,
        hash -> Text,
        html -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
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

diesel::joinable!(article_tags -> articles (article_id));
diesel::joinable!(article_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_tags,
    articles,
    cache,
    tags,
    users,
);
