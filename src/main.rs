#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use std::sync::Mutex;

use backend::{redirect_to, GlobalState, ALL_MENU, FOOD_KOREAN, REVIEW_NAMES};
use fake::Fake;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

// lazy_static! {
//     static ref ALL_MENU: HashMap<&'static str, &'static str> = {
//         let mut m: HashMap<&'static str, &'static str> = HashMap::new();
//         m.insert("햄버거", "burger");
//         m.insert("마약계란덮밥", "mayak");
//         m.insert("냉모밀", "momil");
//         m.insert("라면", "ramen");
//         m.insert("떡볶이", "tteokbokki");
//         m.insert("우동", "udon");
//         m
//     };
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Food {
    name: String,
    desc: String,
    picture_url: String,
    review_count: usize,
    reivew_rate: u8,
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
pub struct SignupParam {
    name: String,
    password: String,
    email: String,
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

#[post("/user/signup")]
async fn post_signup(
    data: web::Data<Mutex<GlobalState>>,
    params: web::Form<SignupParam>,
) -> impl Responder {
    println!("=> User name: {}", params.name);
    let mut data = data.lock().unwrap();

    if data.get_name(&params.name) {
        return redirect_to("/sign_favorite.html");
    }

    data.name = params.name.to_owned();
    data.password = params.password.to_owned();
    data.email = params.email.to_owned();

    redirect_to("/sign_favorite.html")
}

#[get("/")]
async fn main_html(
    data: web::Data<Mutex<GlobalState>>,
    tera: web::Data<Mutex<Tera>>,
) -> impl Responder {
    let data = data.lock().unwrap();

    println!("=> User name: {}", data.name);
    let mut ctx = Context::new();
    ctx.insert("name", &data.name.to_owned()); // 이전 상태

    let tera = tera.lock().unwrap();

    // let index_html = Asset::get("mainpage.html").unwrap();
    // let index_html = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let rendered = tera.render("mainpage.html", &ctx).unwrap();
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

#[get("/menu_new.html")]
async fn menu_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let data = data.lock().unwrap();
    let tera = tera.lock().unwrap();

    let mut ctx = Context::new();

    ctx.insert("name", data.name.clone().as_str());
    for (food_name_kor, food_name_eng) in ALL_MENU.iter() {
        ctx.insert(
            format!("{}_review", food_name_eng),
            &data.get_review_rate(food_name_kor.to_string()),
        );
        ctx.insert(
            format!("total_review_{}", food_name_eng),
            &data.get_total_review_counts(food_name_kor.to_string()),
        );
    }

    let rendered = tera.render("menu_new.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/signup.html")]
async fn signup_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let data = data.lock().unwrap();
    let tera = tera.lock().unwrap();

    let mut ctx = Context::new();

    ctx.insert("name_cache", data.name.clone().as_str());
    ctx.insert("password_cache", data.password.clone().as_str());
    ctx.insert("email_cache", data.email.clone().as_str());

    let rendered = tera.render("signup.html", &ctx).unwrap();
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
        name: "덮밥".to_string(),
        desc: "밥 위에 고기, 야채, 소스 등을 넣고 같이 섞어 먹는 요리의 일종이다.".to_string(),
        picture_url: "images/dupbap.png".to_string(),
        ..Default::default()
    });
    foods.push(Food {
        name: "우동".to_string(),
        desc: "소바와 함께 일본의 전통적이고 가장 대중적인 면 요리.\n소바는 간토, 우동은 간사이를 대표하는 면요리다.".to_string(),
        picture_url: "images/udon.png".to_string(),
        ..Default::default()
    });
    // foods.push(Food {
    //     name: "햄버거".to_string(),
    //     desc: "패티를 구운 후 다양한 부재료와 함께 빵 사이에 끼워 먹는 음식이다.".to_string(),
    //     picture_url: "images/hamburger.png".to_string(),
    //     ..Default::default()
    // });
    // foods.push(Food {
    //     name: "햄버거".to_string(),
    //     desc: "패티를 구운 후 다양한 부재료와 함께 빵 사이에 끼워 먹는 음식이다.".to_string(),
    //     picture_url: "images/hamburger.png".to_string(),
    //     ..Default::default()
    // });

    // let sample: Vec<_> = foods
    //     .choose_multiple(&mut rand::thread_rng(), 5)
    //     .collect();

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
            .service(main_html)
            .service(sign_fav_html)
            .service(signup_html)
            .service(menu_html)
            .service(recommend_html)
            .service(history_html)
            .service(backend::order::my_order_html)
            .service(backend::order::get_order_food)
            .service(backend::order::update_food_order_status)
            .service(backend::review::review_html)
            .service(backend::review::review_write_html)
            .service(backend::review::post_review)
            .service(post_login)
            .service(post_signup)
            .service(ResourceFiles::new("/", generated).do_not_resolve_defaults())
        // 센서 POST
        // .service(pi_kakao::sensor::led_control)
    })
    .bind(backend::SERVER)?
    .run()
    .await
}
