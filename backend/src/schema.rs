table! {
    devs (id) {
        id -> Int4,
        name -> Varchar,
        github -> Varchar,
        bio -> Text,
        avatar_url -> Varchar,
        techs -> Array<Text>,
    }
}
