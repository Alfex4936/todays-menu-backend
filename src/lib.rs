#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_derive;
extern crate serde_json;

pub const SERVER: &str = "0.0.0.0:8010";

mod routes;

use order::Order;
use review::Review;
pub use routes::order;
pub use routes::review;

// Global
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FOOD_KOREAN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("burger", "햄버거");
        m.insert("mayak", "마약계란덮밥");
        m.insert("momil", "냉모밀");
        m.insert("ramen", "라면");
        m.insert("tteokbokki", "떡볶이");
        m.insert("udon", "우동");
        m
    };

    pub static ref ALL_MENU: HashMap<&'static str, &'static str> = {
        let mut m: HashMap<&'static str, &'static str> = HashMap::new();
        m.insert("햄버거", "burger");
        m.insert("마약계란덮밥", "mayak");
        m.insert("냉모밀", "momil");
        m.insert("라면", "ramen");
        m.insert("떡볶이", "tteokbokki");
        m.insert("우동", "udon");
        m
    };

    pub static ref REVIEW_NAMES: Vec<&'static str> = {
        // https://nickname.hwanmoo.kr/?format=json&count=50
        let m: Vec<&'static str> = vec![
            "젓가락질이 서툰 미스터초밥왕",
            "돈이 다 떨어진 주사기",
            "상상도 못한 웰시코기",
            "겁이 많은 햄스터",
            "기차놀이하는 병뚜껑",
            "mbti 맹신하는 나무늘보",
            "무슨 말을 해도 안 웃긴 천칭자리",
            "신조어 1도 모르는 엑셀마스터",
            "머리를 긁적이는 사랑꾼",
            "여행 중인 족발",
            "나 빼고 다 결혼한 바밤바",
            "유연한 크리스티아누호날두",
            "당돌한 아따맘마",
            "고양이한테 물린 우산꽂이",
            "머리 빨리 자라나는 햄스터",
            "여름에 만세 못하는 리신",
            "사랑니빠진 토익만점자",
            "배고픈 등산러",
            "Fun cool sexy 거지",
            "상처받은 바비인형",
            "행복한 자물쇠",
            "스트릿출신 대학생",
            "실리를 추구하는 선인장",
            "고양이한테 물린 핫식스",
            "책 읽는 척 하는 언더아머 단속반",
            "개가 짖을까봐 조마조마한 푸시알림",
            "쏘맥타는 대학원생",
            "돈이 다 떨어진 산책용 목줄",
            "반사회적 노사연",
            "찬물에 샤워하는 김기사",
            "조깅하는 오메가3",
            "선물을 고민하는 사형수",
            "강아지 눈치 보는 핫식스",
            "자전거를 타고다니는 공기계",
            "경찰서에 잡혀 온 악당",
            "소원 비는 인현왕후",
            "부서진 거미",
            "당황한 샌님",
            "스트릿출신 셀레나고메즈",
            "경찰서에 잡혀 온 도라에몽",
            "코고는 새내기",
            "결혼한 공대생",
            "퇴사하고 싶은 살바도르 달리",
            "Fun cool sexy 종이학",
            "무슨 말을 해도 안 웃긴 빨대",
            "시대를 앞서간 음밥페",
            "과장님한테 혼난 골프공",
            "회의하는 사랑꾼",
            "시계 볼 줄 모르는 인싸",
            "고시공부하는 쾌걸근육MAN",
            "출튀하는 모닝커피",
            "반장 경험이 없는 실험맨",
            "상처받은 사장",
            "시력이 나빠지는 리버풀",
            "Fun cool sexy 피피티마스터",
            "꼰대를 싫어하는 자물쇠",
            "야식을 좋아하는 돈까스소스",
            "반사회적 엽기토끼",
            "주변을 경계하는 순두부",
            "숙취에 쩔은 새우깡",
            "김밥 마는 오장육부",
            "매일 셀카 찍는 종이학",
            "졸업사진찍는 영국사람",
            "집에 물이 새는 키오스크",
            "깍두기 담그는 이구아나",
            "어깨빵하는 궁예",
            "젓가락질이 서툰 곤약젤리",
            "mbti 맹신하는 리신",
            "쿨찐 만렙 찍은 해커",
            "학사경고 받은 세종대왕",
            "폰 잃어버린 딜리버리맨",
            "주먹이 작은 별에서온그대",
            "휴가 짤린 라운지",
            "매운 음식을 좋아하는 호갱",
            "마스크 찾고 있는 모닝커피",
            "고기굽는 에너지 드링크",
            "나 빼고 다 결혼한 치즈냥이",
            "랜덤으로 생성된 대학생",
            "회사에서 게임하는 마당딸린 집",
            "거북목의 성가대 소프라노",
            "한 여름밤의 도라에몽",
            "스쿼트 하는 푸시알림",
            "시대를 앞서간 암모나이트",
            "108배 하는 스폰지밥",
            "질풍노도의 오토바이",
            "잠꼬대하는 마리오",
            "사다리타는 오토바이",
            "뒤로 걷는 세젤귀",
            "웃음소리 이상한 대학생",
            "잔소리하는 알바 사장",
            "죽어가는 크리스티아누호날두",
            "108배 하는 시베리안허숙희",
            "퇴사하고 싶은 명함",
            "일 하기 싫은 베스트드라이버",
            "택배를 받은 치즈피자",
            "바짓가랑이 붙잡는 슬리퍼",
            "면도하는 맛집평론가",
            "음악 듣는 호박",
            "로또 당첨될 운명의 아메리카노",
            "앱을 출시해서 기쁜 사자",
        ];
        m
    };

    static ref REVIEW_TXT: Vec<&'static str> = {
        // https://nickname.hwanmoo.kr/?format=json&count=50
        let m: Vec<&'static str> = vec![
            "맛있네요",
            "맛있어ㅠㅠ",
            "맛없어ㅠㅠ",
            "맛없어요ㅠㅠ",
            "맛이ㅠㅠ",
            "가격이ㅠㅠ",
            "싸고 좋음",
            "싸고 맛남",
            "머이리 매워요",
            "머이리 짜요",
            "ㅋㅋㅋㅋㅋㅋㅋ",
            "이걸 왜먹어",
            "다신 안먹음",
            "오래 안걸렸네요",
            "15분 정도 걸린듯",
            "30분 정도 걸린듯",
            "한시간 정도 걸린듯",
            "머리카락 나옴",
            "아니 왜이리 오래 걸려",
            "최악임 먹지 마셈",
            "혀가 수치스러웠습니다",
            "우리 집 개도 안먹을듯",
            "너무 맛있게 잘 먹음",
            "먹고 A+ 나옴",
            "ㅁㄴㅇㄻㅇㄹㄴㅁㄴㅇㄹ",
        ];
        m
    };
}

use actix_web::{http::header::LOCATION, HttpResponse};
use std::cell::RefCell;
use std::collections::HashMap;

use fake::Fake;

#[derive(Debug, Default)]
pub struct GlobalState {
    pub name: String,     // 유저 네임
    pub password: String, // 유저 비번
    pub email: String,    // 이메일
    pub reviews: HashMap<String, RefCell<Vec<Review>>>,
    pub orders: HashMap<String, Order>,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut m: HashMap<String, RefCell<Vec<Review>>> = HashMap::new();
        m.insert("햄버거".to_string(), RefCell::new(Vec::new()));
        m.insert("냉모밀".to_string(), RefCell::new(Vec::new()));
        m.insert("마약계란덮밥".to_string(), RefCell::new(Vec::new()));
        m.insert("라면".to_string(), RefCell::new(Vec::new()));
        m.insert("떡볶이".to_string(), RefCell::new(Vec::new()));
        m.insert("우동".to_string(), RefCell::new(Vec::new()));

        let mut orders: HashMap<String, Order> = HashMap::new();
        orders.insert("햄버거".to_string(), Order::new());
        orders.insert("냉모밀".to_string(), Order::new());
        orders.insert("마약계란덮밥".to_string(), Order::new());
        orders.insert("라면".to_string(), Order::new());
        orders.insert("떡볶이".to_string(), Order::new());
        orders.insert("우동".to_string(), Order::new());

        GlobalState {
            name: "ajounice".to_string(),
            reviews: m,
            orders: orders,
            ..Default::default()
        }
    }

    pub fn fake(self) -> Self {
        use rand::seq::SliceRandom;

        for _ in 0..(100..200).fake::<i32>() {
            // 마약계란덮밥 랜덤 데이터
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            // let review_txt: Vec<String> = Words(5..15).fake();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (1..=10).fake::<u8>(),
            };
            self.add("마약계란덮밥".to_string(), review);
        }
        for _ in 0..(200..600).fake::<i32>() {
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (4..=10).fake::<u8>(),
            };
            self.add("햄버거".to_string(), review);
        }
        for _ in 0..(100..300).fake::<i32>() {
            // 냉모밀
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (5..=7).fake::<u8>(),
            };
            self.add("냉모밀".to_string(), review);
        }
        for _ in 0..(100..800).fake::<i32>() {
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (6..=10).fake::<u8>(),
            };
            self.add("라면".to_string(), review);
        }
        for _ in 0..(100..700).fake::<i32>() {
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (4..=10).fake::<u8>(),
            };
            self.add("떡볶이".to_string(), review);
        }
        for _ in 0..(100..600).fake::<i32>() {
            let sample: Vec<_> = REVIEW_NAMES
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample2: Vec<_> = REVIEW_TXT
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let review = Review {
                writer: sample[0].to_string(),
                review_txt: sample2[0].to_string(),
                rate: (5..=10).fake::<u8>(),
            };
            self.add("우동".to_string(), review);
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

    pub fn add_order(&mut self, food: String, number: i32) -> bool {
        // let mut review = Review::new();
        // review.writer = user_name.to_owned();
        // review.review_txt = review_txt.to_owned();
        // review.rate = rate;

        let order = self.orders.get_mut(&food).unwrap();
        if order.order_number != 0 {
            return false;
        }

        self.orders.get_mut(&food).unwrap().order_number = number;
        self.orders.get_mut(&food.to_owned()).unwrap().order_food = food.to_owned();
        // self.orders.get_mut(&food).unwrap().set_order_number(number);
        // self.orders.get_mut(&food).unwrap().set_order_food(food);

        true
    }

    pub fn get_order_number_by_food(&self, food: String) -> i32 {
        self.orders.get(&food).unwrap().order_number
    }

    pub fn get_name(&self, name: &str) -> bool {
        if self.name.eq(name) {
            true
        } else {
            false
        }
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
        // 10 ~ 100
        let array = self.reviews.get(&food).unwrap().borrow();
        let mut total_rate: u32 = 1;

        for review in array.iter() {
            total_rate += review.rate as u32;
            // println!("rate: {}", review.rate);
        }

        // println!("Total_rate: {}, {}", total_rate, array.len());

        ((total_rate as usize / array.len()) * (10 as usize)) as u8
    }

    pub fn get_review_rate_range(&self, food: String) -> f32 {
        // 0.0 ~ 5.0
        // convert 10~100 range to 0.0 ~ 5.0 with ratio
        // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
        let new_range_value = (self.get_review_rate(food) - 10) as f32 * 5.0 / 90 as f32 + 0.0;

        new_range_value // 12.123 -> 12.0
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
