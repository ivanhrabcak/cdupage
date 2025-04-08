#![doc = "If you don't know where to start, you can look at all the available methods in the [`edupage::Edupage`] struct documentation."]
#![doc = include_str!("../README.md")]

pub mod cdupage;
pub(crate) mod deserializers;
pub mod edupage;
pub(crate) mod macro_aliases;
pub mod traits;
pub mod types;

#[cfg(feature = "node")]
mod node;

#[macro_use]
extern crate macro_rules_attribute;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(not(feature = "node-types"))]
#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::traits::{DBI, Login, Timeline, Timetable};

    fn get_env_var(name: &'static str) -> Option<String> {
        use std::env;

        match env::var(name) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    #[test]
    fn login_test() {
        dotenv::dotenv().ok();

        use crate::edupage::Edupage;

        let subdomain = get_env_var("SUBDOMAIN");
        let username = get_env_var("USERNAME");
        let password = get_env_var("PASSWORD");

        if [&subdomain, &username, &password].contains(&&None) {
            debug_assert_ne!(subdomain, None);
            debug_assert_ne!(username, None);
            debug_assert_ne!(password, None);
        }

        let mut edupage = Edupage::new();

        let subdomain = subdomain.unwrap();
        let username = username.unwrap();
        let password = password.unwrap();

        let login_result = edupage.login(&subdomain, &username, &password);

        assert_matches!(login_result, Ok(_));

        assert!(edupage.logged_in());
    }

    #[test]
    fn dbi_test() {
        dotenv::dotenv().ok();

        use crate::edupage::Edupage;

        let subdomain = get_env_var("SUBDOMAIN");
        let username = get_env_var("USERNAME");
        let password = get_env_var("PASSWORD");

        if [&subdomain, &username, &password].contains(&&None) {
            debug_assert_ne!(subdomain, None);
            debug_assert_ne!(username, None);
            debug_assert_ne!(password, None);
        }

        let mut edupage = Edupage::new();

        let subdomain = subdomain.unwrap();
        let username = username.unwrap();
        let password = password.unwrap();

        let login_result = edupage.login(&subdomain, &username, &password);
        assert_matches!(login_result, Ok(_));

        let homework = edupage
            .filter_timeline_by_item_type(crate::types::timeline::TimelineItemType::Homework);
        assert_matches!(homework, Ok(_));

        let teachers = edupage.get_teachers();
        assert_matches!(teachers, Ok(_));

        let students = edupage.get_students();
        assert_matches!(students, Ok(_));

        let subjects = edupage.get_subjects();
        assert_matches!(subjects, Ok(_));

        let classrooms = edupage.get_classrooms();
        assert_matches!(classrooms, Ok(_));
    }

    #[test]
    fn timetable_test() {
        dotenv::dotenv().ok();

        use crate::edupage::Edupage;

        let subdomain = get_env_var("SUBDOMAIN");
        let username = get_env_var("USERNAME");
        let password = get_env_var("PASSWORD");

        if [&subdomain, &username, &password].contains(&&None) {
            debug_assert_ne!(subdomain, None);
            debug_assert_ne!(username, None);
            debug_assert_ne!(password, None);
        }

        let mut edupage = Edupage::new();

        let subdomain = subdomain.unwrap();
        let username = username.unwrap();
        let password = password.unwrap();

        let login_result = edupage.login(&subdomain, &username, &password);
        assert_matches!(login_result, Ok(_));

        let today = Utc::now().naive_local();
        let result = edupage.get_timetable(today.date());
        assert_matches!(result, Ok(_));
    }
}
