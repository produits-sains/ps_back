table! {
    find_product_rules (id) {
        id -> Nullable<Integer>,
        vendor -> Text,
        xpath -> Text,
        regexp_rule -> Text,
        regexp_flags -> Text,
    }
}

table! {
    ingr (id) {
        id -> Nullable<Integer>,
        vendor -> Text,
        name -> Text,
        score -> Integer,
        last_update -> Integer,
    }
}

table! {
    raw_products (id) {
        id -> Nullable<Integer>,
        vendor -> Text,
        vendor_id -> Text,
        name -> Text,
        url -> Text,
        ingr_txt -> Text,
        scraper_version -> Text,
        parsed -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    find_product_rules,
    ingr,
    raw_products,
);
