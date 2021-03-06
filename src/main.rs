extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate diesel;

use std::env;

use crate::schema::applications::dsl::*;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use dashmap::DashMap;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{insert_into, MysqlConnection};
use dotenv::dotenv;
use models::Application;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

static CACHE: Lazy<DashMap<String, UUIDResponse>> = Lazy::new(|| DashMap::new());

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UUIDResponse {
    id: String,
}

async fn application(pool: Data<DbPool>, application: Json<Application>) -> impl Responder {
    let database = pool.get().expect("couldn't get db connection from pool");
    if application_exists(application.linking_id, &database).await {
        println!(
            "Attempted submission already exists: {}",
            &application.minecraft_username
        );
        HttpResponse::Accepted().body("Application already exists!")
    } else {
        insert_submission(&application, &database).await
    }
}

async fn insert_submission(
    application: &Application,
    connection: &MysqlConnection,
) -> HttpResponse {
    println!(
        "Insertion of submission: {}",
        &application.minecraft_username
    );

    let result = insert_into(applications)
        .values(application)
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().body("Application inserted successfully."),
        Err(why) => {
            eprintln!("Application failed to insert!, {:#?}", why);
            HttpResponse::BadRequest().body("Application failed to insert.")
        }
    }
}

async fn validate(name: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let player_name = name.to_lowercase();

    // Favour cache
    if CACHE.get(&player_name).is_some() {
        let uuid_resp = UUIDResponse {
            id: CACHE.get(&player_name).unwrap().id.clone(),
        };
        return actix_web::Result::Ok(HttpResponse::Ok().json(uuid_resp));
    }

    let url = format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        &player_name
    );

    let mut uuid_response = UUIDResponse { id: "".to_string() };

    // Try get response from mojang
    if let Ok(response) = reqwest::get(&url).await {
        if let Ok(json) = response.json::<UUIDResponse>().await {
            uuid_response = json;
        }
    }

    // Insert to cache
    CACHE.insert(player_name, uuid_response.clone());

    actix_web::Result::Ok(HttpResponse::Ok().json(uuid_response))
}

async fn application_exists(discord_id: i64, database: &MysqlConnection) -> bool {
    let results = applications
        .filter(linking_id.eq(discord_id))
        .filter(status.eq(0))
        .limit(1)
        .load::<Application>(database)
        .expect("Error checking to see if application exists");

    !results.is_empty()
}

/// Default path, returns the index file
async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/dist/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Now up and running!");
    HttpServer::new(|| {
        let db_url =
            env::var("DATABASE_URL").expect("No DATABASE_URL environment variable defined!");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool)
            .service(web::resource("/validate/{name}").route(web::get().to(validate)))
            .service(web::resource("/application/submit").route(web::post().to(application)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/", "static/dist/").show_files_listing())
    })
    .bind("127.0.0.1:8003")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::Application;

    #[test]
    fn test_serialisation() {
        let json_string = r#"
          {
          "minecraftUsername": "hu_sk",
          "age": 22,
          "linkingId": 276519212175065088,
          "addOneThing": "animals",
          "projectsOnBiome": "nothing",
          "biggestProject": "Bigness",
          "showcase": "Not much"
          }"#;

        let application: Application = serde_json::from_str(json_string).unwrap();

        assert_eq!(application.status, 0);
        assert_eq!(application.minecraft_username, "hu_sk");

        let json_string = r#"
          {
          "minecraftUsername": "hu_sk",
          "age": 22,
          "linkingId": 276519212175065088,
          "addOneThing": "more animal",
          "projectsOnBiome": "nothing",
          "biggestProject": "Big",
          "showcase": "Not much",
          "status": 2
          }"#;

        let application: Application = serde_json::from_str(json_string).unwrap();

        assert_eq!(application.status, 0);
    }
}
