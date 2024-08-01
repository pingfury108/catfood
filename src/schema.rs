// @generated automatically by Diesel CLI.

diesel::table! {
    cat_food (gid) {
        gid -> Text,
        title -> Text,
        describe -> Text,
        img -> Nullable<Text>,
        product_name -> Nullable<Text>,
        brand_owner -> Nullable<Text>,
        brand_owner_addr -> Nullable<Text>,
        manufacturer -> Nullable<Text>,
        manufacturer_addr -> Nullable<Text>,
        manufacturer_license_number -> Nullable<Text>,
        product_standard -> Nullable<Text>,
        net_content -> Nullable<Text>,
        reference_price -> Nullable<Text>,
        ingredients -> Nullable<Text>,
        production_method -> Nullable<Text>,
        additives -> Nullable<Text>,
        guaranteed_analysis -> Nullable<Text>,
    }
}

diesel::table! {
    users (uid) {
        uid -> Text,
        name -> Text,
        pwd -> Text,
        display_name -> Text,
        email -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(cat_food, users,);
