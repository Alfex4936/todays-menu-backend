#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::{get, post, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tera::{Context, Tera};

use crate::{redirect_to, GlobalState};

lazy_static! {
    static ref FOOD_KOREAN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("sushi", "스시");
        m.insert("burger", "햄버거");
        m.insert("dakgangjung", "햄버거");
        m.insert("ramen", "라면");
        m.insert("tteokbokki", "떡볶이");
        m.insert("udon", "우동");
        m
    };
}

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

    redirect_to("/menu2.html")
}
