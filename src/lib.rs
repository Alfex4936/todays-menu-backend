#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_derive;
extern crate serde_json;

pub const SERVER: &str = "0.0.0.0:8010";

mod routes;

use review::Review;
pub use routes::review;

// Global
use actix_web::{http::header::LOCATION, HttpResponse};
use std::cell::RefCell;
use std::collections::HashMap;

use fake::{Fake, Faker};

#[derive(Debug)]
pub struct GlobalState {
    pub name: String, // 유저 네임
    pub reviews: HashMap<String, RefCell<Vec<Review>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut m: HashMap<String, RefCell<Vec<Review>>> = HashMap::new();
        m.insert("스시".to_string(), RefCell::new(Vec::new()));
        m.insert("햄버거".to_string(), RefCell::new(Vec::new()));

        GlobalState {
            name: "".to_string(),
            reviews: m,
        }
    }

    pub fn fake(self) -> Self {
        use fake::faker::lorem::en::*;
        use fake::faker::name::raw::*;
        use fake::locales::EN;

        for _ in 0..(100..500).fake::<i32>() {
            // 스시 랜덤 데이터
            let review_txt: Vec<String> = Words(5..30).fake();

            let review = Review {
                writer: Name(EN).fake(),
                review_txt: review_txt.join(" "),
                rate: (5..=10).fake::<u8>(),
            };
            self.add("스시".to_string(), review);
        }
        for _ in 0..(100..500).fake::<i32>() {
            let review_txt: Vec<String> = Words(5..30).fake();

            let review = Review {
                writer: Name(EN).fake(),
                review_txt: review_txt.join(" "),
                rate: (5..=10).fake::<u8>(),
            };
            self.add("햄버거".to_string(), review);
        }

        self
    }

    pub fn change_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn add(&self, food: String, review: Review) {
        // let mut review = Review::new();
        // review.writer = user_name.to_owned();
        // review.review_txt = review_txt.to_owned();
        // review.rate = rate;

        self.reviews.get(&food).unwrap().borrow_mut().push(review);
    }

    pub fn get_review(&self, food: String) -> Vec<Review> {
        let array = self.reviews.get(&food).unwrap().borrow();
        let mut reviews = vec![];
        for review in array.iter() {
            reviews.push(Review {
                writer: review.writer.to_owned(),
                review_txt: review.review_txt.to_owned(),
                rate: review.rate,
            });
        }

        reviews
    }

    pub fn get_review_rate(&self, food: String) -> u8 {
        let array = self.reviews.get(&food).unwrap().borrow();
        let mut total_rate: u32 = 1;

        for review in array.iter() {
            total_rate += review.rate as u32;
            // println!("rate: {}", review.rate);
        }

        // println!("Total_rate: {}, {}", total_rate, array.len());

        ((total_rate as usize / array.len()) * (10 as usize)) as u8
    }

    pub fn get_total_review_counts(&self, food: String) -> usize {
        self.reviews.get(&food).unwrap().borrow().len()
    }
}

pub fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((LOCATION, location))
        .finish()
}
