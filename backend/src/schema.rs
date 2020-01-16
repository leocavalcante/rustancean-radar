table! {
    devs (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        github -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Varchar>,
        techs -> Nullable<Array<Text>>,
    }
}
