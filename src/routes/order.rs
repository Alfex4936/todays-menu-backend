#![allow(proc_macro_derive_resolution_fallback)]
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::{redirect_to, GlobalState, FOOD_KOREAN};
use chrono::prelude::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

trait Korean {
    fn kweek(&self) -> String;
    fn kday(&self) -> String;
}

impl Korean for chrono::DateTime<chrono::Local> {
    fn kweek(&self) -> String {
        match self.weekday() {
            Weekday::Mon => return "월요일".to_string(),
            Weekday::Tue => return "화요일".to_string(),
            Weekday::Wed => return "수요일".to_string(),
            Weekday::Thu => return "목요일".to_string(),
            Weekday::Fri => return "금요일".to_string(),
            Weekday::Sat => return "토요일".to_string(),
            Weekday::Sun => return "일요일".to_string(),
        };
    }

    fn kday(&self) -> String {
        match self.hour() {
            h if h > 12 => return "오후".to_string(),
            _ => return "오전".to_string(),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Order {
    pub order_number: i32,
    pub order_food: String,
    pub status: u8, // -1 = 주문 실패, 0 = 주문 완료, 1 = 조리 시작, 2 = 조리 완료
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
    let food_name_kor = FOOD_KOREAN.get(food_name.as_str()).unwrap();

    let current = Local::now();

    let mut ctx = Context::new();

    ctx.insert("name", data.name.clone().as_str());
    ctx.insert("food_name", food_name_kor);
    ctx.insert(
        "time",
        &format!("2022-06-17 {}{}", current.kday(), current.format("%-I:%M")),
    );

    ctx.insert(
        "order_number",
        &data.get_order_number_by_food(food_name_kor.to_string()),
    );
    match data.get_order_status_by_food(food_name_kor.to_string()) {
        0 => {
            ctx.insert("order_status", "주문 완료");
            ctx.insert("order_status_icon", "🔔");
        }
        1 => {
            ctx.insert("order_status", "조리 시작");
            ctx.insert("order_status_icon", "🔥");
        }
        2 => {
            ctx.insert("order_status", "조리 완료");
            ctx.insert("order_status_icon", "😋");
        }
        _ => {
            ctx.insert("order_status", "주문 실패");
            ctx.insert("order_status_icon", "&#9888;");
        }
    }

    let rendered = tera.render("my_order.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/my_order/")]
async fn get_order_food() -> impl Responder {
    redirect_to(&format!("/my_order/")) // /order/momil
}

#[get("/my_order/order/{food}/{status}")]
async fn update_food_order_status(
    data: web::Data<Mutex<GlobalState>>,
    food: web::Path<(String, u8)>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let food_name_eng = food.0.to_string(); // eng
    let food_name_kor = FOOD_KOREAN.get(food_name_eng.as_str()).unwrap();

    data.add_order(food_name_kor.to_string(), (100..999).fake::<i32>());
    data.set_order_status(food_name_kor.to_string(), food.1);

    redirect_to(&format!("/my_order/{food_name_eng}")) // /order/momil
}
