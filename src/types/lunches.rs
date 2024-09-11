use chrono::{NaiveDate, NaiveTime};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::edupage::Edupage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lunch {
    served_from: Option<NaiveTime>,
    served_to: Option<NaiveTime>,
    amount_of_foods: Option<u32>,
    chooseable_menus: Vec<String>,
    can_be_changed_until: Option<String>,
    title: Option<String>,
    menus: Vec<Menu>,
    date: NaiveDate,
    boarder_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Menu {
    name: String,
    allergens: Option<String>,
    weight: Option<String>,
    number: Option<String>,
    rating: Option<Rating>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rating {
    date: String,
    boarder_id: String,
    quality_average: Option<f32>,
    quantity_average: Option<f32>,
    quality_ratings: Option<u32>,
    quantity_ratings: Option<u32>,
}

pub struct Lunches {
    edupage: Edupage,
}
impl Lunch {
    pub(crate) fn make_choice(self, epage: Edupage, choice_str: &str) {
        let sub = &epage.subdomain;
        let request_url = format!("{sub:#?}.edupage.org/menu");
        let boarder_menu = serde_json::json!({"stravnikid": self.boarder_id,
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
    /// Choose the food
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
}
impl Lunches {
    /// Retrieves lunch information for a given date
    ///
    /// # Arguments
    ///
    /// * `date` - The date for which to retrieve lunch information
    ///
    /// # Returns
    ///
    /// * `Result<Option<Lunch>>` - The lunch information if available, or None if not cooking
    pub fn get_lunch(&self, date: NaiveDate) -> Result<Option<Lunch>, Box<dyn Error>> {
        let date_strftime = date.format("%Y%m%d").to_string();
        let request_url = format!(
            "https://{}.edupage.org/menu/?date={}",
            self.edupage.subdomain.clone().unwrap(),
            date_strftime
        );

        let response = self.edupage.client.get(&request_url).send()?.text()?;
        let lunch_data: HashMap<String, serde_json::Value> = serde_json::from_str(
            response
                .split("edupageData: ")
                .nth(1)
                .unwrap()
                .split(",\r\n")
                .next()
                .unwrap(),
        )?;

        let lunches_data = lunch_data
            .get(&self.edupage.subdomain.clone().unwrap())
            .ok_or("Missing lunch data")?;

        let boarder_id = lunches_data
            .get("novyListok")
            .and_then(|nl| nl.get("addInfo").and_then(|ai| ai.get("stravnikid")))
            .ok_or("Missing boarder id")?
            .as_str()
            .unwrap()
            .to_string();

        let lunch = lunches_data
            .get("novyListok")
            .and_then(|nl| nl.get(&date.format("%Y-%m-%d").to_string()));

        if lunch.is_none() {
            return Ok(None);
        }

        let lunch = lunch.unwrap().get("2").unwrap();

        if lunch
            .get("isCooking")
            .unwrap_or(&serde_json::Value::Bool(false))
            .as_bool()
            .unwrap()
            == false
        {
            return Ok(Some(Lunch {
                served_from: None,
                served_to: None,
                amount_of_foods: None,
                chooseable_menus: vec![],
                can_be_changed_until: None,
                title: None,
                menus: vec![],
                date,
                boarder_id,
            }));
        }

        let served_from_str = lunch.get("vydaj_od").and_then(|v| v.as_str());
        let served_to_str = lunch.get("vydaj_do").and_then(|v| v.as_str());

        let served_from = served_from_str
            .map(|s| NaiveTime::parse_from_str(s, "%H:%M").ok())
            .flatten();
        let served_to = served_to_str
            .map(|s| NaiveTime::parse_from_str(s, "%H:%M").ok())
            .flatten();

        let title = lunch
            .get("nazov")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let amount_of_foods = lunch
            .get("druhov_jedal")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);
        let chooseable_menus = lunch
            .get("choosableMenus")
            .and_then(|v| v.as_object())
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default();

        let can_be_changed_until = lunch
            .get("zmen_do")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut menus = Vec::new();

        if let Some(rows) = lunch.get("rows").and_then(|v| v.as_array()) {
            for food in rows {
                if food.is_null() {
                    continue;
                }

                let name = food
                    .get("nazov")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let allergens = food
                    .get("alergenyStr")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let weight = food
                    .get("hmotnostiStr")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let number = food
                    .get("menusStr")
                    .and_then(|v| v.as_str())
                    .map(|s| s.replace(": ", ""));
                let mut rating = None;

                if let Some(num) = &number {
                    if let Some(rating_data) = lunch
                        .get("hodnotenia")
                        .and_then(|v| v.as_object())
                        .and_then(|obj| obj.get(num))
                    {
                        let quality = rating_data
                            .get("priemer")
                            .and_then(|v| v.as_f64())
                            .map(|v| v as f32);
                        rating = Some(Rating {
                            date: date.format("%Y-%m-%d").to_string(),
                            boarder_id: boarder_id.clone(),
                            quality_average: quality,
                            quantity_average: rating_data
                                .get("pocet")
                                .and_then(|v| v.as_u64())
                                .map(|v| v as f32),
                            quality_ratings: rating_data
                                .get("pocet")
                                .and_then(|v| v.as_u64())
                                .map(|v| v as u32),
                            quantity_ratings: rating_data
                                .get("pocet")
                                .and_then(|v| v.as_u64())
                                .map(|v| v as u32),
                        });
                    }
                }

                menus.push(Menu {
                    name,
                    allergens,
                    weight,
                    number,
                    rating,
                });
            }
        }

        Ok(Some(Lunch {
            served_from,
            served_to,
            amount_of_foods,
            chooseable_menus,
            can_be_changed_until,
            title,
            menus,
            date,
            boarder_id,
        }))
    }
}
