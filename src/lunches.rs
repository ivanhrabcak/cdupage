use crate::edupage::Edupage;
use crate::edupage::RequestType;
use reqwest::Response;
use serde::Deserializer;
// TODO: Do docs on this
#[derive(Default)]
pub struct Lunches<Menu: 'static> {
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
impl<'de, Menu: Deserializer<'de>> Lunches<Menu> {
    pub(crate) fn make_choice(self, epage: Edupage, choice_str: &str)  {
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
        epage.request(request_url, RequestType::GET, None, Some(data.to_string())).unwrap();
    }
    pub fn choose(self, epage: Edupage) {
        todo!()
    }
    pub fn sign_off(self, epage: Edupage) {}
}
