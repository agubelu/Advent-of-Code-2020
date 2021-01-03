use std::collections::HashMap;
use Rule::*;

///////////////////////////////////////////////////////////////////////////////

pub type RuleIndex = u32;
pub type RuleCombo = Vec<RuleIndex>;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Rule {
    Terminal { val: String },
    Composite { options: Vec<RuleCombo>},
}

impl Rule {
    pub fn to_regex(&self, map: &HashMap<RuleIndex, Rule>) -> String {
        match self {
            Terminal{val} => val.clone(),
            Composite{options} => {
                if options.len() == 1 {
                    Self::create_combo_re(&options[0], &map)
                } else {
                    format!("({})", 
                        options.iter().map(|combo| Self::create_combo_re(combo, &map)).collect::<Vec<_>>().join("|")
                    )
                }
            },
        }
    }

    fn create_combo_re(combo: &[RuleIndex], map: &HashMap<RuleIndex, Rule>) -> String {
        return combo.iter().map(|i| map[i].to_regex(&map)).collect::<Vec<_>>().join("");
    }
}