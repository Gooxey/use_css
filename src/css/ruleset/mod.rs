use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};

use self::rule::Rule;

pub mod rule;

pub struct RuleSet {
    name: String,
    rules: Vec<Rule>,
}
impl RuleSet {
    pub fn to_rust(&self) -> TokenStream {
        let mut rules_rust_code = quote!();
        for rule in &self.rules {
            let rule_str = rule.to_rust();

            rules_rust_code = quote!(
                #rules_rust_code
                + #rule_str
            );
        }

        let name = format_ident!("{}", self.name.to_string());
        quote!(
            /// This function has been generated by the [`use_css`](use_css::use_css) macro. For more detail on this function see the [`use_css crate's documentation`](use_css).
            pub fn #name() -> stylist::StyleSource {
                stylist::StyleSource::from_str(
                    &(
                        String::default()
                        #rules_rust_code
                    )
                ).expect("The given code is not css!")
            }
        )
    }
}
impl From<Vec<&str>> for RuleSet {
    fn from(value: Vec<&str>) -> Self {
        let mut css_code = value.iter();

        // Get the name of this Ruleset
        let first_line = css_code
            .next()
            .expect("The Ruleset should consist of at least 1 line!");
        let name = first_line
            .split(' ')
            .next()
            .expect("The first line of the Ruleset should not be empty!")
            .to_string();

        // Parse the Rules
        let mut rules = vec![];
        for line in css_code {
            let line = line.trim().to_string();

            if line.contains('}') {
                break;
            } else if line.is_empty() {
                continue;
            }
            rules.push(Rule::from(line));
        }

        Self { name, rules }
    }
}
