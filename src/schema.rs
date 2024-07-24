// @generated automatically by Diesel CLI.

diesel::table! {
    cat_food (gid) {
        gid -> Text,
        title -> Text,
        describe -> Text,
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
