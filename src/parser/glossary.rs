use regex::Regex;
pub trait Glossary {
    fn new() -> Self;

    /// Ordered list of choices
    ///
    /// TODO: macro this behavior when a new
    /// choice is added to
    /// ```
    /// struct ChoiceGlossary
    /// ```
    fn _glossary(&self) -> Vec<&super::choice::Choice>;

    /// Search for the index of a choice in the glossary
    ///
    /// Specify `right` as `false` or `true` does
    /// leftmost or rightmost search respectively
    fn _bisect_search(&self, choice: &super::choice::ChoiceFlag, right: bool) -> i32 {
        let glossary = self._glossary();

        let mut l = 0i32;
        let mut r = glossary.len() as i32 - 1i32;
        let mut index = -1;

        while l <= r {
            let mid: usize = ((l + r) / 2) as usize;
            let mid_i32 = mid as i32;
            if choice.short != char::default() && glossary[mid]._to_flag().short == choice.short
                || choice.long != String::default()
                    && glossary[mid]._to_flag().long.starts_with(&choice.long)
            {
                index = mid_i32;
                if right {
                    l = mid_i32 + 1
                } else {
                    r = mid_i32 - 1
                }
            } else if choice.short != char::default()
                && glossary[mid]._to_flag().short < choice.short
                || choice.long != String::default() && glossary[mid]._to_flag().long < choice.long
            {
                l = mid_i32 + 1
            } else {
                r = mid_i32 - 1;
            }
        }

        index
    }

    /// Search glossary for ambigious options (if any)
    fn search(&self, mut arg: &str) -> Vec<String> {
        let mut find = vec![];
        let glossary = self._glossary();
        let mut choice = super::choice::ChoiceFlag::default();

        // Check args to distinguish long/short form and
        // set choice flag to compare against glossary
        if !arg.starts_with("--") && arg.starts_with("-") && arg.len() == 2 {
            choice.short = arg.chars().nth(1).unwrap_or_default();
        } else if arg.starts_with("--") && arg.len() >= 3 {
            // clear flag
            arg = arg.trim_start_matches("-");
            choice.long = arg.to_string();
        } else {
            return find;
        }

        // Search range if multiple options
        let left_index;
        let right_index;
        left_index = self._bisect_search(&choice, false);
        right_index = self._bisect_search(&choice, true);

        if left_index != -1 && right_index != -1 {
            for index in left_index..right_index + 1 {
                find.push(glossary[index as usize]._to_flag().long);
            }
        }
        find
    }

    /// Verify if an argument is a match with specified choice
    ///
    /// if ambigous then will print out possible choices
    /// # Examples
    ///
    /// ```
    /// let choice: choice::ChoiceGlossary = choice::ChoiceGlossary::new();
    /// assert_eq!(choice.is_choice("--verb", choice.verbose), true)
    /// ```
    fn is_choice(&self, arg: &str, choice: &super::choice::Choice) -> bool {
        let option_rule = Regex::new(r#"^(-.{1}|^--.*)$"#).unwrap(); // short options - ; long options --
        let args_nodash = arg.trim_start_matches("-");
        return choice._to_flag().long.starts_with(args_nodash) && option_rule.is_match(arg);
    }
}

/// All available choices to use within cli
pub(crate) struct ChoiceGlossary {
    pub(crate) help: super::choice::Choice,
    pub(crate) nocache: super::choice::Choice,
    pub(crate) offline: super::choice::Choice,
    pub(crate) quiet: super::choice::Choice,
    pub(crate) verbose: super::choice::Choice,
    pub(crate) version: super::choice::Choice,
}

impl Glossary for ChoiceGlossary {
    /// Ordered list of choices
    ///
    /// TODO: macro this behavior when a new
    /// choice is added to
    /// ```
    /// struct ChoiceGlossary
    /// ```
    fn _glossary(&self) -> Vec<&super::choice::Choice> {
        vec![
            &self.help,
            &self.nocache,
            &self.offline,
            &self.quiet,
            &self.verbose,
            &self.version,
        ]
    }

    fn new() -> Self {
        Self {
            help: super::choice::Choice::Switch {
                enable: false,
                short: 'h',
                long: String::from("help"),
            },
            nocache: super::choice::Choice::Switch {
                enable: false,
                short: 'n',
                long: String::from("nocache"),
            },
            offline: super::choice::Choice::Switch {
                enable: false,
                short: 'o',
                long: String::from("offline"),
            },
            quiet: super::choice::Choice::Switch {
                enable: false,
                short: 'q',
                long: String::from("quiet"),
            },
            verbose: super::choice::Choice::Switch {
                enable: false,
                short: 'v',
                long: String::from("verbose"),
            },
            version: super::choice::Choice::Name {
                short: 'V',
                long: String::from("version"),
            },
        }
    }
}

struct MockGlossary {
    help: super::choice::Choice,
    verbose: super::choice::Choice,
    version: super::choice::Choice,
}

impl Glossary for MockGlossary {
    fn _glossary(&self) -> Vec<&super::choice::Choice> {
        vec![&self.help, &self.verbose, &self.version]
    }
    fn new() -> Self {
        Self {
            help: super::choice::Choice::Name {
                short: 'h',
                long: String::from("help"),
            },
            verbose: super::choice::Choice::Name {
                short: 'v',
                long: String::from("verbose"),
            },
            version: super::choice::Choice::Name {
                short: 'V',
                long: String::from("version"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_choice() {
        let glossary = MockGlossary::new();
        assert_eq!(glossary.is_choice("--hlpe", &glossary.help), false);
        assert_eq!(glossary.is_choice("--hel", &glossary.help), true);
        assert_eq!(glossary.is_choice("-h", &glossary.help), true);
        assert_eq!(glossary.is_choice("-he", &glossary.help), false);
    }

    #[test]
    fn test_search() {
        let glossary = MockGlossary::new();
        assert_eq!(glossary.search("--ver").len(), 2);
        assert_eq!(glossary.search("--h").len(), 1);
        assert_eq!(glossary.search("--n").len(), 0);
    }
}
