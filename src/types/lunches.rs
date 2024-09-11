use crate::edupage::{Edupage, RequestType};
use reqwest::Method;
use serde::Deserialize;
// TODO: Do docs on this
#[derive(Default, Clone, Deserialize)]
pub struct Lunches<Menu> {
    amount_of_foods: i32,
    can_be_changed_until: chrono::NaiveDate,
    choosable_menus: Vec<String>,
    date: chrono::NaiveDate,
    menus: Vec<Menu>,
    served_from: Option<chrono::NaiveDate>,
    served_to: Option<chrono::NaiveDate>,
    title: String,
    __boarder_id: String,
}
#[derive(Default)]
pub(crate) struct ALACarte<Rating> {
    allergens: String,
    name: String,
    number: String,
    rating: Option<Rating>,
    weight: String,
}
impl<Menu: Clone> Lunches<Menu> {
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
            self.clone().make_choice(epage.clone(), i);
        }
    }
    pub fn sign_off(self, epage: Edupage) {}
}
