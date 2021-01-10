extern crate dotenv;
extern crate env_logger;

use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use database_connection::MySQLConnection;
use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;

mod database_connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct UUIDResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
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

async fn application(
    database: Data<MySQLConnection>,
    application: Json<Application>,
) -> impl Responder {
    if application_exists(&application.linking_id, &database).await {
        println!(
            "Attempted submission already exists: {}",
            &application.minecraft_username
        );
        HttpResponse::Accepted().body("Application already exists!")
    } else {
        let update = format!("INSERT INTO `applications` (`minecraft_username`, `age`, `linking_id`, `add_one_thing`, `projects_on_biome`, `biggest_project`, `showcase`, `status`) VALUES ('{}',{},{},'{}','{}','{}','{}', {});", application.minecraft_username, application.age, application.linking_id, application.add_one_thing, application.projects_on_biome, application.biggest_project, application.showcase, application.status);

        println!(
            "Insertion of submission: {}",
            &application.minecraft_username
        );

        match database.execute_update(&update).await {
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

async fn application_exists(discord_id: &str, database: &MySQLConnection) -> bool {
    let results = database
        .execute_query(&format!(
            "SELECT * FROM applications WHERE discord_id = {} AND status = 0;",
            discord_id,
        ))
        .await;

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
        let mysql = MySQLConnection::new();
        App::new()
            .wrap(middleware::Logger::default())
            .data(mysql)
            .service(web::resource("/validate/{name}").route(web::get().to(validate)))
            .service(web::resource("/application/submit").route(web::post().to(application)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/", "static/dist/").show_files_listing())
    })
    .bind("127.0.0.1:8003")?
    .run()
    .await
}
