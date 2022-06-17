#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tera::{Context, Tera};

use crate::{redirect_to, GlobalState};

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
