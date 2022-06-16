#![allow(proc_macro_derive_resolution_fallback)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use std::sync::Mutex;

use crate::{redirect_to, GlobalState, ALL_MENU, FOOD_KOREAN, REVIEW_NAMES};
use fake::Fake;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Order {
    pub order_number: i32,
    pub order_food: String,
}

impl Order {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_order_number(&mut self, number: i32) {
        self.order_number = number;
    }

    pub fn set_order_food(&mut self, food: String) {
        self.order_food = food;
    }
}

#[get("/my_order/{food}")]
async fn my_order_html(
    tera: web::Data<Mutex<Tera>>,
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<String>,
) -> impl Responder {
    let data = data.lock().unwrap();
    let tera = tera.lock().unwrap();

    let food_name = food.into_inner();
    let food_name = FOOD_KOREAN.get(food_name.as_str()).unwrap();

    let mut ctx = Context::new();

    ctx.insert("name", data.name.clone().as_str());
    ctx.insert("food_name", food_name);

    ctx.insert(
        "order_number",
        &data.get_order_number_by_food(food_name.to_string()),
    );
    ctx.insert("order_status", "주문 완료");

    let rendered = tera.render("my_order.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/my_order/order/{food}")]
async fn get_order_food(
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<String>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let food_name = food.into_inner(); // eng

    if let _result = data.add_order(
        FOOD_KOREAN.get(food_name.as_str()).unwrap().to_string(),
        (100..999).fake::<i32>(),
    ) {
        // true: new order
    } else {
        // false: already exisiting
    }

    redirect_to(&format!("/my_order/{food_name}")) // /order/momil
}
