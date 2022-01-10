use node_bindgen::derive::node_bindgen;

use crate::{edupage::{Edupage as RealEdupage, EdupageError}, edupage_traits::{Login, DBI}, edupage_types::Teacher};

mod trait_implementations;
mod external_wrapped;

struct Edupage {
    edupage: RealEdupage
}

#[node_bindgen]
impl Edupage {
    #[node_bindgen(constructor)]
    fn new() -> Self {
        Self { edupage: RealEdupage::new() }
    }

    #[node_bindgen]
    async fn login(&mut self, subdomain: String, username: String, password: String) -> Result<(), EdupageError> {
        self.edupage.login(&subdomain, &username, &password)
    }
    
    #[node_bindgen]
    fn is_logged_in(&self) -> bool {
        self.edupage.logged_in()
    }

    #[node_bindgen]
    fn get_teachers(&self) -> Result<Vec<Teacher>, EdupageError> {
        self.edupage.get_teachers()
    }
}