use macro_rules_attribute::derive_alias;

derive_alias! {
    #[derive(Serde!)] = #[derive(Deserialize, Clone, Debug)];
}
