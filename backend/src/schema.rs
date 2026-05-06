diesel::table! {
    candidates (id) {
        id -> Uuid,
        user_id -> Uuid,
        position_id -> Uuid,
        approved -> Bool,
        manifesto -> Nullable<Text>,
        votes_count -> Int4,
    }
}

diesel::table! {
    departments (id) {
        id -> Uuid,
        name -> Varchar,
        faculty_id -> Uuid,
    }
}

diesel::table! {
    faculties (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        slogan -> Nullable<Varchar>,
        mission -> Nullable<Text>,
        vision -> Nullable<Text>,
        faculty_id -> Nullable<Uuid>,
        department_id -> Nullable<Uuid>,
        logo_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    positions (id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        eligibility_rules -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        verified -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    votes (id) {
        id -> Uuid,
        user_id -> Uuid,
        candidate_id -> Uuid,
        position_id -> Uuid,
        timestamp -> Timestamp,
    }
}

diesel::joinable!(candidates -> positions (position_id));
diesel::joinable!(candidates -> users (user_id));
diesel::joinable!(departments -> faculties (faculty_id));
diesel::joinable!(organizations -> departments (department_id));
diesel::joinable!(organizations -> faculties (faculty_id));
diesel::joinable!(positions -> organizations (organization_id));
diesel::joinable!(votes -> candidates (candidate_id));
diesel::joinable!(votes -> positions (position_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    candidates,
    departments,
    faculties,
    organizations,
    positions,
    users,
    votes,
);
