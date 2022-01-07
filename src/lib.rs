#![feature(assert_matches)]

pub mod edupage_types;
pub mod edupage;
pub mod edupage_deserializers;
pub mod edupage_traits;
pub mod trait_implementations;

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::edupage_traits::Login;

    fn get_env_var(name: &'static str) -> Option<String> {
        use std::env;

        match env::var(name) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    #[test]
    fn login_test() {
        dotenv::dotenv().ok();

        use crate::{edupage::Edupage};

        let subdomain = get_env_var("SUBDOMAIN");
        let username = get_env_var("USERNAME");
        let password = get_env_var("PASSWORD");

        if vec![&subdomain, &username, &password].contains(&&None) {
            debug_assert_ne!(subdomain, None);
            debug_assert_ne!(username, None);
            debug_assert_ne!(password, None);
        }

        let mut edupage = Edupage::new();
        
        let subdmain = subdomain.unwrap();
        let username = username.unwrap();
        let password = password.unwrap();

        let login_result = edupage.login(&subdmain, &username, &password);

        assert_matches!(login_result, Ok(_));

        assert_eq!(edupage.logged_in(), true);
    }
}
