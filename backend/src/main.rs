#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod auth;
mod ws;

use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest, Error};
use actix_cors::Cors;
use actix_web_actors::ws as actix_ws;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use uuid::Uuid;
use crate::models::{Organization, NewUser, User, NewVote};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn register(pool: web::Data<DbPool>, form: web::Json<NewUser>) -> impl Responder {
    use schema::users::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    if !form.email.ends_with("@students.uonbi.ac.ke") {
        return HttpResponse::BadRequest().body("Only @students.uonbi.ac.ke emails allowed");
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(form.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let new_user = models::NewUser {
        email: form.email.clone(),
        password: password_hash,
        role: "student".to_string(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json("User registered")
}

async fn login(pool: web::Data<DbPool>, form: web::Json<models::NewUser>) -> impl Responder {
    use schema::users::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = users
        .filter(email.eq(&form.email))
        .first::<User>(&mut conn);

    match user {
        Ok(u) => {
            let parsed_hash = PasswordHash::new(&u.password).expect("Failed to parse hash");
            if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
                let token = auth::create_jwt(u.id, &u.role).expect("Token creation failed");
                HttpResponse::Ok().json(serde_json::json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

async fn get_organizations(pool: web::Data<DbPool>) -> impl Responder {
    use schema::organizations::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let orgs = organizations
        .load::<Organization>(&mut conn)
        .expect("Error loading organizations");

    HttpResponse::Ok().json(orgs)
}

async fn vote(pool: web::Data<DbPool>, req: HttpRequest, vote_data: web::Json<NewVote>) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() { return HttpResponse::Unauthorized().finish(); }
    
    let token = auth_header.unwrap().to_str().unwrap().replace("Bearer ", "");
    let claims = match auth::decode_jwt(&token) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    use schema::votes::dsl::*;
    use schema::candidates::dsl::{candidates, id as cand_id, votes_count};
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_vote = NewVote {
        user_id: claims.sub,
        candidate_id: vote_data.candidate_id,
        position_id: vote_data.position_id,
    };

    let res = diesel::insert_into(votes)
        .values(&new_vote)
        .execute(&mut conn);

    match res {
        Ok(_) => {
            diesel::update(candidates.filter(cand_id.eq(vote_data.candidate_id)))
                .set(votes_count.eq(votes_count + 1))
                .execute(&mut conn)
                .expect("Failed to update vote count");
            HttpResponse::Ok().json("Vote recorded")
        }
        Err(_) => HttpResponse::BadRequest().body("Duplicate vote or invalid candidate"),
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    actix_ws::start(ws::MyWs::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    println!("Starting UON-EOMS API on 0.0.0.0:8080...");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/api/auth/register", web::post().to(register))
            .route("/api/auth/login", web::post().to(login))
            .route("/api/organizations", web::get().to(get_organizations))
            .route("/api/vote", web::post().to(vote))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
