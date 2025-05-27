// @generated automatically by Diesel CLI.

diesel::table! {
    mesaje (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        content -> Nullable<Text>,
        date -> Nullable<Date>,
    }
}
