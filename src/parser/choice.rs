pub enum Choice {
    Switch {
        enable: bool,
        short: char,
        long: String,
    },
    Name {
        short: char,
        long: String,
    },
}

impl Choice {
    pub fn enable(&mut self) {
        match self {
            Self::Switch { enable, .. } => {
                *enable = true;
                Ok(())
            }
            _ => Err(String::from(
                "Choice does not have a switch. Try defining choice type as `Choice::Switch`",
            )),
        }
        .unwrap_or_default()
    }

    pub fn disable(&mut self) {
        match self {
            Self::Switch { enable, .. } => {
                *enable = false;
                Ok(())
            }
            _ => Err(String::from(
                "Choice does not have a switch. Try defining choice type as `Choice::Switch`",
            )),
        }
        .unwrap_or_default()
    }

    /// Get the state the choice is in.
    ///
    /// The state is true if `enabled` or false if `disabled`
    ///
    /// # Examples
    ///
    /// ```
    /// let choice: choice::ChoiceGlossary = choice::ChoiceGlossary::new();
    /// assert_eq!(choice.verbose.get_state(), false)
    /// ```
    pub fn get_state(&self) -> bool {
        match self {
            Self::Switch { enable, .. } => Ok(*enable),
            _ => Err(String::from(
                "Choice does not have a switch. Try defining choice type as `Choice::Switch`",
            )),
        }
        .unwrap_or_default()
    }
}
/// All available choices to use within cli
pub struct ChoiceGlossary {
    pub help: Choice,
    pub nocache: Choice,
    pub offline: Choice,
    pub quiet: Choice,
    pub verbose: Choice,
    pub version: Choice,
}

impl ChoiceGlossary {
    fn _glossary(&self) -> Vec<&Choice> {
        vec![
            &self.help,
            &self.nocache,
            &self.offline,
            &self.quiet,
            &self.verbose,
            &self.version,
        ]
    }

    fn _search(&self) {
        for choice in self._glossary() {
            println!("{}", choice.get_state())
        }
    }

    pub fn new() -> Self {
        Self {
            help: Choice::Switch {
                enable: false,
                short: 'h',
                long: String::from("help"),
            },
            nocache: Choice::Switch {
                enable: false,
                short: 'n',
                long: String::from("nocache"),
            },
            offline: Choice::Switch {
                enable: false,
                short: 'o',
                long: String::from("offline"),
            },
            quiet: Choice::Switch {
                enable: false,
                short: 'q',
                long: String::from("quiet"),
            },
            verbose: Choice::Switch {
                enable: false,
                short: 'v',
                long: String::from("verbose"),
            },
            version: Choice::Name {
                short: 'V',
                long: String::from("version"),
            },
        }
    }

    /// Verify if an argument is a valid choice
    /// # Examples
    ///
    /// ```
    /// let choice: Choice = Choice::default();
    /// assert_eq!(choice.is_choice("--hlpre", "h", "help"), false)
    /// ```
    pub fn is_choice(&self, arg: &str, short: &str, long: &str) -> bool {
        self._search();
        let choice: &str = arg.trim_start_matches("-");
        return arg.starts_with("--") && long.starts_with(choice)
            || arg.starts_with("-") && short == choice;
    }
}

#[test]
fn test_is_choice() {
    let choice: ChoiceGlossary;
    assert_eq!(choice.is_choice("--hlpre", "h", "help"), false);
}
