extern crate dotenv;
extern crate env_logger;

use std::env;

use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use bson::Bson;
use dotenv::dotenv;
use futures::executor::block_on;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::{bson, Client, Database};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UUIDResponse {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    minecraft_username: String,
    age: String,
    linking_id: String,
    add_one_thing: String,
    projects_on_biome: String,
    biggest_project: String,
    showcase: String,
    #[serde(skip_deserializing)]
    status: i32,
}

async fn application(database: Data<Database>, application: Json<Application>) -> impl Responder {
    if get_application(&application.linking_id, &database)
        .await
        .is_some()
    {
        println!(
            "Attempted submission already exists: {}",
            &application.minecraft_username
        );
        HttpResponse::Accepted().body("Application already exists!")
    } else {
        let collection = database.collection("applications");
        let serialised = bson::to_bson(&application.0).unwrap();
        let document = serialised.as_document().unwrap().clone();

        println!(
            "Insertion of submission: {}",
            &application.minecraft_username
        );

        match collection.insert_one(document, None).await {
            Ok(_) => HttpResponse::Ok().body("Application inserted successfully."),
            Err(why) => {
                eprintln!("Application failed to insert!, {:#?}", why);
                HttpResponse::BadRequest().body("Application failed to insert.")
            }
        }
    }
}

async fn validate(name: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let player_name = name.to_lowercase();
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

    actix_web::Result::Ok(HttpResponse::Ok().json(uuid_response))
}

async fn get_application(discord_id: &str, database: &Database) -> Option<Application> {
    let collection = database.collection("applications");

    let document = doc! { "linkingId": &discord_id };

    if let Ok(mut cursor) = collection.find(document, None).await {
        while let Some(doc_result) = cursor.next().await {
            if let Ok(document) = doc_result {
                let app: Application = bson::from_bson(Bson::Document(document)).unwrap();
                return Some(app);
            }
        }
    }
    None
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
        let mongo_conn_str = env::var("DB_CONN_STR").expect("No DB_CONN_STR variable found!");

        let mongo_db = env::var("DB_NAME").expect("No DB_NAME variable found!");

        let client = block_on(Client::with_uri_str(&mongo_conn_str))
            .expect("Unable to connect to database provided!");

        let db = client.database(&mongo_db);

        App::new()
            .wrap(middleware::Logger::default())
            .data(db)
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
          "age": "22",
          "linkingId": "276519212175065088",
          "addOneThing": "drugsssss",
          "projectsOnBiome": "nothing im fat",
          "biggestProject": "Big cok n ball",
          "showcase": "Not much"
          }"#;

        let application: Application = serde_json::from_str(json_string).unwrap();

        assert_eq!(application.status, 0);
        assert_eq!(application.minecraft_username, "hu_sk");

        let json_string = r#"
          {
          "minecraftUsername": "hu_sk",
          "age": "22",
          "linkingId": "276519212175065088",
          "addOneThing": "drugsssss",
          "projectsOnBiome": "nothing im fat",
          "biggestProject": "Big cok n ball",
          "showcase": "Not much",
          "status": 2
          }"#;

        let application: Application = serde_json::from_str(json_string).unwrap();

        assert_eq!(application.status, 0);
    }
}
