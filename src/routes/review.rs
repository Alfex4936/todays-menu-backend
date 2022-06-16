#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tera::{Context, Tera};

use crate::{redirect_to, GlobalState, FOOD_KOREAN};

#[derive(Serialize, Deserialize)]
pub struct ReviewParam {
    pub review_txt: String,
    pub food: String,
    pub rate_param: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Review {
    pub writer: String,
    pub review_txt: String,
    pub rate: u8,
}

impl Review {
    pub fn new() -> Self {
        Default::default()
    }
}

#[get("/review/{food}")]
async fn review_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<String>,
) -> impl Responder {
    println!("=> 리뷰 페이지 메뉴");

    let food_name = food.into_inner();

    let data = data.lock().unwrap();

    let mut reviews: Vec<Review> =
        data.get_review(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string());
    reviews.reverse(); // 최신 리뷰순

    let mut ctx = Context::new();
    ctx.insert("name", &data.name.clone());
    ctx.insert("food_name", FOOD_KOREAN.get(food_name.as_str()).unwrap());
    ctx.insert("food_name_eng", food_name.as_str());
    ctx.insert("reviews", &reviews);

    ctx.insert(
        "total_review_count",
        &data.get_total_review_counts(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string()),
    );
    ctx.insert(
        "total_star",
        &data.get_review_rate_range(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string()),
    );

    // println!("{:?}", ctx);

    let tera = tera.lock().unwrap();

    let rendered = tera.render("reviewpage.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/review/write/{food}")]
async fn review_write_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<String>,
) -> impl Responder {
    println!("=> 리뷰 메뉴");

    let food_name = food.into_inner();

    let data = data.lock().unwrap();

    let mut reviews: Vec<Review> =
        data.get_review(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string());
    reviews.reverse(); // 최신 리뷰순

    let mut ctx = Context::new();
    ctx.insert("name", &data.name.clone());
    ctx.insert("food_name", FOOD_KOREAN.get(food_name.as_str()).unwrap());
    ctx.insert("food_name_eng", food_name.as_str());
    ctx.insert("reviews", &reviews);

    ctx.insert(
        "total_review_count",
        &data.get_total_review_counts(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string()),
    );
    ctx.insert(
        "total_star",
        &data.get_review_rate_range(FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string()),
    );

    // println!("{:?}", ctx);

    let tera = tera.lock().unwrap();

    let rendered = tera.render("review.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/review_save")]
async fn post_review(
    data: web::Data<Mutex<GlobalState>>,
    params: web::Form<ReviewParam>,
) -> impl Responder {
    println!("=> POST review");
    let data = data.lock().unwrap();

    let review = Review {
        writer: data.name.to_owned(),
        review_txt: params.review_txt.to_owned(),
        rate: params.rate_param,
    };

    data.add(params.food.to_owned(), review);
    // println!("{:?}", data.get_review(params.food.to_owned())); // 저장된 리뷰 보기

    // redirect_to("/menu_new.html")
    redirect_to("/menu_new.html")
}
