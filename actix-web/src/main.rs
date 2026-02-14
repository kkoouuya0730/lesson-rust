use std::fmt::format;

use actix_web::{
    error, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::Deserialize;
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);
    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

// #[get("/users/{user_id}/{friend}")]
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!("Welcome {}, user_id {}", info.friend, info.user_id))
// }

// #[get("/users/{user_id}/{friend}")]
// async fn index(req: HttpRequest) -> Result<String> {
//     let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
//     let userid: i32 = req.match_info().query("user_id").parse().unwrap();

//     Ok(format!("Welcome {}, user_id {}!", name, userid))
// }

async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.friend)
}

#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}", info.friend))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // use actix_web::{App, HttpServer};

    // HttpServer::new(|| {
    //     let json_config = web::JsonConfig::default()
    //         .limit(4096)
    //         .error_handler(|err, _req| {
    //             error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    //         });

    //     App::new().service(
    //         web::resource("/")
    //             .app_data(json_config)
    //             .route(web::post().to(index)),
    //     )
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
    let data = AppState {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(show_count)
            .service(add_one)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
