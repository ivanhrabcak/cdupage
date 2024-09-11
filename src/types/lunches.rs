use crate::edupage::{Edupage, RequestType};
use reqwest::{Body, Method};
use serde::Deserialize;
// TODO: Do docs on this
#[derive(Default, Clone)]
pub struct Lunch<Menu> {
    amount_of_foods: i32,
    can_be_changed_until: chrono::NaiveDate,
    choosable_menus: Vec<String>,
    date: chrono::NaiveDate,
    menus: Vec<Menu>,
    served_from: Option<chrono::NaiveDate>,
    served_to: Option<chrono::NaiveDate>,
    title: String,
    __boarder_id: String,
    __epage: Edupage,
}
#[derive(Default)]
pub(crate) struct ALACarte<Rating> {
    allergens: String,
    name: String,
    number: String,
    rating: Option<Rating>,
    weight: String,
}
impl<Menu: Clone> Lunch<Menu> {
    pub(crate) fn make_choice(self, epage: Edupage, choice_str: &str) {
        let sub = &epage.subdomain;
        let request_url = format!("{sub:#?}.edupage.org/menu");
        let boarder_menu = serde_json::json!({"stravnikid": self.__boarder_id,
            "mysqlDate": self.date,
            "jids": {"2": choice_str},
            "view": "pc_listok",
            "pravo": "Student",
        });
        let data = serde_json::json!({
            "akcia": "ulozJedlaStravnika",
            "jedlaStravnika": boarder_menu,
        });
        epage
            .client
            .request(Method::GET, request_url)
            .body(data.to_string())
            .build()
            .unwrap();
    }
    pub fn choose(self, epage: Edupage, number: i32) {
        let letters: Option<[&str; 8]> = Some(["A", "B", "C", "D", "E", "F", "G", "H"]);
        let letter = letters.iter().nth((number - 1) as usize);
        for i in letter.unwrap() {
            self.clone().make_choice(epage.clone(), i)
        }
    }
    pub fn sign_off(self, epage: Edupage) {
        self.make_choice(epage, "AX")
    }
    pub fn get_lunch(self, date: chrono::NaiveDate) {
        let date_strftime = date.format("%Y%m%d");
        let request_url = format!("{}.edupage.org/menu", Edupage::new().subdomain.unwrap());
        let response = self
            .__epage
            .client
            .request(Method::GET, request_url)
            .build()
            .unwrap();
        todo!("Somehow figure out JSON splitting without known schema")
    }
}
pub trait Menu<R: Rating> {
    const ALLERGENS: String;
    const NAME: String;
    const NUMBER: String;
    const RATING: Option<R>;
    const WEIGHT: String;
}
#[derive(Default, Deserialize)]
pub struct Lunches(Edupage);

impl Lunches {
    #[deprecated = "Not finished yet."]
    async fn get_lunch(self, date: chrono::NaiveDate) {
        let date_strftime = date.format("%Y%m%d");
        let request_url = format!(
            "{}.edupage.org/menu/?date={date_strftime}",
            Edupage::new().subdomain.unwrap()
        );
        // let response = reqwest::get(request_url)
        //     .await.unwrap()
        //     .json()
        //     .await
        //     .unwrap();

        // let lunch_data: Self = serde_json::from_str(response).unwrap();
    }
}
pub trait Rating {
    type RatingDate;
    type RatingBoarderId;
    const QUALITY_AVERAGE: f32;
    const QUALITY_RATINGS: f32;
    const QUANTITY_AVERAGE: f32;
    const QUANTITY_RATINGS: f32;
    fn rate(self, epage: Edupage, quantity: i32, quality: i32);
}
