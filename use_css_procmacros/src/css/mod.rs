use proc_macro2::TokenStream;
use quote::quote;

use self::ruleset::RuleSet;

mod ruleset;

pub struct Css {
    rulesets: Vec<RuleSet>,
}
impl Css {
    /// Generate rust functions from the parsed css code.
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
        // Find Rulesets in the given css_code
        let mut rulesets = vec![];
        let mut current_ruleset: Vec<&str> = vec![];
        // we need this counter to allow the use of nested styles
        let mut parsing_level = 0;
        for line in value.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            current_ruleset.push(line);

            if line.contains('{') {
                parsing_level += 1;
                continue;
            } else if line.contains('}') {
                parsing_level -= 1;
            }

            if parsing_level == 0 {
                rulesets.push(RuleSet::from(current_ruleset.clone()));
                current_ruleset.clear();
            }
        }

        Self { rulesets }
    }
}
