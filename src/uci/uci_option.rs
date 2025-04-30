pub enum OptionType {
    Spin,
    Button,
}

impl std::fmt::Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spin => write!(f, "spin"),
            Self::Button => write!(f, "button"),
        }
    }
}

pub struct UCIOption {
    pub name: &'static str,
    pub option_type: OptionType,
    pub min: Option<String>,
    pub max: Option<String>,
    pub default: Option<String>,
}

impl UCIOption {
    pub const fn new(
        name: &'static str,
        option_type: OptionType,
        min: Option<String>,
        max: Option<String>,
        default: Option<String>,
    ) -> Self {
        Self {
            name,
            option_type,
            min,
            max,
            default,
        }
    }
}

impl std::fmt::Display for UCIOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_str = self
            .default
            .as_ref()
            .map_or_else(String::new, |default| format!(" default {default}"));

        let min_str = self
            .min
            .as_ref()
            .map_or_else(String::new, |min| format!(" min {min}"));

        let max_str = self
            .max
            .as_ref()
            .map_or_else(String::new, |max| format!(" max {max}"));

        write!(
            f,
            "option name {} type {}{default_str}{min_str}{max_str}",
            self.name, self.option_type
        )
    }
}
