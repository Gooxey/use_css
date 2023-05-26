use proc_macro2::TokenStream;
use quote::quote;

use self::ruleset::RuleSet;

mod ruleset;

pub struct Css {
    rulesets: Vec<RuleSet>,
}
impl Css {
    pub fn to_rust(&self) -> TokenStream {
        let mut rust_code = quote!();
        for ruleset in &self.rulesets {
            let ruleset_rust_code = ruleset.to_rust();

            rust_code = quote!(
                #rust_code
                #ruleset_rust_code
            );
        }

        rust_code
    }
}
impl From<String> for Css {
    fn from(value: String) -> Self {
        let css_code = value.split('\n');

        // Find Rulesets in the given css_code
        // Then let these Rulesets parse their Rules
        let mut rulesets = vec![];
        let mut current_ruleset: Vec<&str> = vec![];
        // required for parsing nested rulesets
        let mut parsing_level = 0;
        for line in css_code {
            if line.contains('{') {
                parsing_level += 1;

                if parsing_level == 1 {
                    current_ruleset.clear();
                }
            } else if line.contains('}') {
                parsing_level -= 1;

                if parsing_level == 0 {
                    rulesets.push(RuleSet::from(current_ruleset.clone()));
                    continue;
                }
            }
            current_ruleset.push(line);
        }

        Self { rulesets }
    }
}
