use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slogan: Option<String>,
    pub mission: Option<String>,
    pub vision: Option<String>,
    pub faculty_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub logo_url: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = positions)]
pub struct Position {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub eligibility_rules: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = candidates)]
pub struct Candidate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub position_id: Uuid,
    pub approved: bool,
    pub manifesto: Option<String>,
    pub votes_count: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = votes)]
pub struct NewVote {
    pub user_id: Uuid,
    pub candidate_id: Uuid,
    pub position_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: usize,
}
