#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Mutex;
use tera::{Context, Tera};

use crate::{GlobalState, ALL_MENU};

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
