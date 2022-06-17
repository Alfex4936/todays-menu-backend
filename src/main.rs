#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use std::sync::Mutex;

use backend::{redirect_to, GlobalState, ALL_MENU, FOOD_KOREAN, REVIEW_NAMES};
use fake::Fake;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Food {
    name: String,
    desc: String,
    picture_url: String,
    review_count: usize,
    review_rate: u8,
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

#[get("/history.html")]
async fn history_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let data = data.lock().unwrap();

    let mut ctx = Context::new();
    ctx.insert("name", data.name.clone().as_str());
    ctx.insert("histories", &data.get_all_history());

    let tera = tera.lock().unwrap();
    let rendered = tera.render("history.html", &ctx).unwrap();

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
        ..Default::default()
    });
    foods.push(Food {
        name: "떡볶이".to_string(),
        desc: "떡과 부재료를 양념에 볶거나 끓여서 먹는 한식".to_string(),
        picture_url: "images/tteokbokki.png".to_string(),
        ..Default::default()
    });
    foods.push(Food {
        name: "라면".to_string(),
        desc: "라면은 국수를 증기로 익힌 뒤 기름에 튀겨 말린 것에 분말 스프를 별도로 첨부한 즉석 식품, 또는 그것을 물에 넣고 끓인 요리를 말한다.".to_string(),
        picture_url: "images/ramyeon.png".to_string(),
        ..Default::default()
    });
    foods.push(Food {
        name: "마약계란덮밥".to_string(),
        desc: "밥 위에 고기, 야채, 소스 등을 넣고 같이 섞어 먹는 요리의 일종이다.".to_string(),
        picture_url: "images/마약계란덮밥.png".to_string(),
        ..Default::default()
    });
    foods.push(Food {
        name: "우동".to_string(),
        desc: "소바와 함께 일본의 전통적이고 가장 대중적인 면 요리.\n소바는 간토, 우동은 간사이를 대표하는 면요리다.".to_string(),
        picture_url: "images/udon.png".to_string(),
        ..Default::default()
    });
    foods.push(Food {
        name: "냉모밀".to_string(),
        desc: "익힌 메밀국수를 차갑게 헹궈서 물기를 뺀 다음 가쓰오부시 장국에 찍어 먹는 음식. 우리나라에서는 '모밀'이라고도 부른다.".to_string(),
        picture_url: "images/냉모밀+새우튀김.png".to_string(),
        ..Default::default()
    });

    let mut rng = thread_rng();
    foods.shuffle(&mut rng);

    let sample: Vec<_> = foods.choose_multiple(&mut rand::thread_rng(), 5).collect();

    let tera = tera.lock().unwrap();

    let mut context = Context::new();
    context.insert("name", data.name.clone().as_str());
    context.insert("foods", &sample);

    let rendered = tera.render("recommend.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("===== 오늘의 메뉴 =====");
    println!("주소창에 입력: http://localhost:8010/");

    let state = web::Data::new(Mutex::new(GlobalState::new().fake()));

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
            .service(backend::user::main_html)
            .service(backend::user::sign_fav_html)
            .service(backend::user::signup_html)
            .service(recommend_html)
            .service(history_html)
            .service(backend::menu::menu_html)
            .service(backend::order::my_order_html)
            .service(backend::order::get_order_food)
            .service(backend::order::update_food_order_status)
            .service(backend::review::review_html)
            .service(backend::review::review_write_html)
            .service(backend::review::post_review)
            .service(backend::user::post_login)
            .service(backend::user::post_signup)
            .service(ResourceFiles::new("/", generated).do_not_resolve_defaults())
        // 센서 POST
        // .service(pi_kakao::sensor::led_control)
    })
    .bind(backend::SERVER)?
    .run()
    .await
}
