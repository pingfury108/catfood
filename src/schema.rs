// @generated automatically by Diesel CLI.

diesel::table! {
    cat_food (gid) {
        gid -> Varchar,
        title -> Varchar,
        describe -> Text,
    }
}
