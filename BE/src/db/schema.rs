diesel::table! {
    my_schema.Ticket (pubkey) {
        pubkey -> Text,
        currency -> Text,
        entry_fee -> Integer,
        duration_type -> Text,
        duration -> Nullable<Text>,
        end_time -> Nullable<Timestamp>,
        main_bank_amount -> Double,
        col_bank_amount -> Double,
        total_players -> Integer,
        created_at -> Timestamp,
        current_state -> Text,
    }
}

diesel::table! {
    my_schema.User (pubkey) {
        pubkey -> Text,
        username -> Nullable<Text>,
        email -> Nullable<Text>,
        total_earnings -> Double,
        withdrawn_earnings -> Double,
        perfect_combinations_won -> Integer,
        created_at -> Timestamp,
        total_earnings_rank -> Nullable<Integer>,
        withdrawn_earnings_rank -> Nullable<Integer>,
        perfect_combinations_rank -> Nullable<Integer>,
    }
}