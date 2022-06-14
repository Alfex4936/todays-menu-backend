#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use std::cell::RefCell;
use std::sync::Mutex;

use lazy_static::lazy_static;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::{Context, Tera};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

lazy_static! {
    static ref FOOD_KOREAN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("sushi", "스시");
        m.insert("burger", "햄버거");
        m
    };
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    name: String,
    desc: String,
    picture_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    writer: String,
    reivew_txt: String,
    rate: u8,
}

#[derive(Debug)]
pub struct GlobalState {
    name: String, // 유저 네임
    reviews: HashMap<String, RefCell<Vec<String>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut m: HashMap<String, RefCell<Vec<String>>> = HashMap::new();
        m.insert("스시".to_string(), RefCell::new(Vec::new()));
        m.insert("햄버거".to_string(), RefCell::new(Vec::new()));

        GlobalState {
            name: "".to_string(),
            reviews: m,
        }
    }

    pub fn change_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn add(&self, food: String, review: String) {
        self.reviews.get(&food).unwrap().borrow_mut().push(review);
    }

    pub fn get_review(&self, food: String) -> Vec<String> {
        let array = self.reviews.get(&food).unwrap().borrow();
        let mut reviews = vec![];
        for review in array.iter() {
            reviews.push(review.to_owned());
        }

        reviews
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

#[derive(Serialize, Deserialize)]
pub struct ReviewParam {
    review_txt: String,
    food: String,
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

#[post("/review_save")]
async fn post_review(
    data: web::Data<Mutex<GlobalState>>,
    params: web::Form<ReviewParam>,
) -> impl Responder {
    println!("=> POST review");
    let data = data.lock().unwrap();

    data.add(params.food.to_owned(), params.review_txt.to_owned());
    println!("{:?}", data.get_review(params.food.to_owned())); // 저장된 리뷰 보기

    redirect_to("/menu2.html")
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

#[get("/review/{food}")]
async fn review_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<String>,
) -> impl Responder {
    println!("=> 리뷰 메뉴");

    let food_name = food.into_inner();

    let data = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", &data.name.clone());
    ctx.insert("food_name", FOOD_KOREAN.get(food_name.as_str()).unwrap());

    let tera = tera.lock().unwrap();

    let rendered = tera.render("review.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/recommend.html")]
async fn recommend_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let data = data.lock().unwrap();

    let mut foods: Vec<Food> = vec![];
    foods.push(Food {
        name: "햄버거".to_string(),
        desc: "패티를 구운 후 다양한 부재료와 함께 빵 사이에 끼워 먹는 음식이다.".to_string(),
        picture_url: "images/hamburger.png".to_string(),
    });
    foods.push(Food {
        name: "떡볶이".to_string(),
        desc: "떡과 부재료를 양념에 볶거나 끓여서 먹는 한식".to_string(),
        picture_url: "images/tteokbokki.png".to_string(),
    });
    foods.push(Food {
        name: "라면".to_string(),
        desc: "라면은 국수를 증기로 익힌 뒤 기름에 튀겨 말린 것에 분말 스프를 별도로 첨부한 즉석 식품, 또는 그것을 물에 넣고 끓인 요리를 말한다.".to_string(),
        picture_url: "images/ramyeon.png".to_string(),
    });
    foods.push(Food {
        name: "덮밥".to_string(),
        desc: "밥 위에 고기, 야채, 소스 등을 넣고 같이 섞어 먹는 요리의 일종이다.".to_string(),
        picture_url: "images/dupbap.png".to_string(),
    });
    foods.push(Food {
        name: "우동".to_string(),
        desc: "소바와 함께 일본의 전통적이고 가장 대중적인 면 요리.\n소바는 간토, 우동은 간사이를 대표하는 면요리다.".to_string(),
        picture_url: "images/udon.png".to_string(),
    });

    let tera = tera.lock().unwrap();

    let mut context = Context::new();
    context.insert("name", data.name.clone().as_str());
    context.insert("foods", &foods);

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = tera.render("recommend.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("===== 오늘의 메뉴 =====");
    println!("Enter: http://localhost:8010/");

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
            .service(review_html)
            .service(post_review)
            .service(post_login)
            .service(ResourceFiles::new("/", generated).do_not_resolve_defaults())
        // 센서 POST
        // .service(pi_kakao::sensor::led_control)
    })
    .bind(backend::SERVER)?
    .run()
    .await
}
