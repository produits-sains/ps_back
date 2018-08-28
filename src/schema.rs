table! {
    picard_raw_products (id) {
        id -> Nullable<Integer>,
        name -> Text,
        ingr_txt -> Text,
        scraper_version -> Text,
        parsed -> Bool,
    }
}
