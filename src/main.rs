use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use std::sync::Mutex;

use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug)]
pub struct GlobalState {
    name: String,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            name: "".to_string(),
        }
    }

    pub fn change_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

#[derive(Serialize, Deserialize)]
pub struct UserParam {
    name: String,
    password: String,
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}

#[post("/user/login")]
async fn post_login(
    data: web::Data<Mutex<GlobalState>>,
    params: web::Form<UserParam>,
) -> impl Responder {
    println!("=> User name: {}", params.name);
    let mut data = data.lock().unwrap();

    data.name = params.name.to_owned();

    redirect_to("/sign_favorite.html")
}

fn handle_embedded_file(path: &str) -> impl Responder {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/")]
async fn main_html(data: web::Data<Mutex<Tera>>) -> impl Responder {
    let ctx = Context::new();
    // ctx.insert("name", "test");

    let data = data.lock().unwrap();

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = data.render("mainpage.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/sign_favorite.html")]
async fn sign_fav_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut ctx = Context::new();
    // ctx.insert("name", "test");

    let data = data.lock().unwrap();
    ctx.insert("name", data.name.clone().as_str());

    let tera = tera.lock().unwrap();

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = tera.render("sign_favorite.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/menu.html")]
async fn menu_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut ctx = Context::new();
    // ctx.insert("name", "test");

    let data = data.lock().unwrap();
    ctx.insert("name", data.name.clone().as_str());

    let tera = tera.lock().unwrap();

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = tera.render("menu.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/recommend.html")]
async fn recommend_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut ctx = Context::new();
    // ctx.insert("name", "test");

    let data = data.lock().unwrap();
    ctx.insert("name", data.name.clone().as_str());

    let tera = tera.lock().unwrap();

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = tera.render("recommend.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("===== 오늘의 메뉴 =====");

    let state = web::Data::new(Mutex::new(GlobalState::new()));

    let tera = web::Data::new(Mutex::new(
        Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap(),
    ));

    // 서버 실행
    HttpServer::new(move || {
        let generated = generate();

        App::new()
            .app_data(state.clone())
            .app_data(tera.clone())
            // 미들웨어
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .wrap(Logger::default())
            // 홈페이지
            .service(main_html)
            .service(sign_fav_html)
            .service(menu_html)
            .service(recommend_html)
            .service(post_login)
            .service(ResourceFiles::new("/", generated).do_not_resolve_defaults())
        // 센서 POST
        // .service(pi_kakao::sensor::led_control)
    })
    .bind(backend::SERVER)?
    .run()
    .await
}
