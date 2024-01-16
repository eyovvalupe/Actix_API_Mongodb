use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    email: String,
}

async fn create_db(uri: &str) -> Collection<User> {
    let client_options = ClientOptions::parse(uri).await.unwrap();
    Client::with_options(client_options)
        .unwrap()
        .database("rustDB")
        .collection("User")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_addr = env::var("HOST_URL").expect("Host url not working");
    let uri = env::var("DATABASE_URL").expect("Database url not working");

    let db = create_db(&uri).await;
    println!("Server Running at {} ....", host_addr);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::resource("/").route(web::get().to(greet)))
            .service(web::resource("/add_user").route(web::post().to(add_user)))
            .route("/get_user/{email}", web::get().to(get_user))
            .route("/update_user/{email}", web::put().to(update_user))
            .route("/delete_user/{email}", web::delete().to(delete_user))
    })
    .bind(host_addr)?
    .run()
    .await
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Jai Mata Di")
}

async fn add_user(user: web::Json<User>, col: web::Data<Collection<User>>) -> impl Responder {
    let user = user.into_inner();
    let email = user.email.to_lowercase();

    if let Ok(Some(_)) = col
        .find_one(doc! { "email": email.to_lowercase() }, None)
        .await
    {
        return HttpResponse::Ok().body("User already available");
    }
    let data = User {
        username: user.username,
        email,
    };
    match col.insert_one(data, None).await {
        Ok(_) => HttpResponse::Ok().body("User added successfully"),
        Err(e) => {
            eprintln!("Error inserting user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
        }
    }
}

async fn get_user(path: web::Path<String>, col: web::Data<Collection<User>>) -> impl Responder {
    let email = path.into_inner();
    // println!("{}",email);
    match col
        .find_one(doc! { "email": &email.to_lowercase() }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Error finding user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn update_user(
    path: web::Path<String>,
    user: web::Json<User>,
    col: web::Data<Collection<User>>,
) -> impl Responder {
    let updated_user = user.into_inner();
    let filter = doc! { "email": &path.into_inner()};
    let email = updated_user.email.to_lowercase();
    let data = User {
        username: updated_user.username,
        email,
    };
    match col.replace_one(filter, data, None).await {
        Ok(result) => {
            if result.modified_count > 0 {
                HttpResponse::Ok().body("User updated successfully")
            } else {
                HttpResponse::NotFound().body("User not found in the database")
            }
        }
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
        }
    }
}

async fn delete_user(path: web::Path<String>, col: web::Data<Collection<User>>) -> impl Responder {
    let email = path.into_inner();

    let filter = doc! { "email": &email };

    match col.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().body("User deleted successfully")
            } else {
                HttpResponse::NotFound().body("User not found in the database")
            }
        }
        Err(e) => {
            eprintln!("Error deleting user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
        }
    }
}
