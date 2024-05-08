pub struct ChoiceName {
    pub short: char,
    pub long: String,
}

// Holds the state of a choice that can be on/off
pub struct ChoiceSwitch {
    pub enable: bool,
    pub short: char,
    pub long: String,
}

/// All available choices to use within cli
#[repr(C)]
pub struct ChoiceGlossary {
    pub help: ChoiceSwitch,
    pub nocache: ChoiceSwitch,
    pub offline: ChoiceSwitch,
    pub quiet: ChoiceSwitch,
    pub verbose: ChoiceSwitch,
    pub version: ChoiceName,
}

impl ChoiceGlossary {
    pub fn new() -> Self {
        Self {
            help: ChoiceSwitch {
                enable: false,
                short: 'h',
                long: String::from("help"),
            },
            nocache: ChoiceSwitch {
                enable: false,
                short: 'n',
                long: String::from("nocache"),
            },
            offline: ChoiceSwitch {
                enable: false,
                short: 'o',
                long: String::from("offline"),
            },
            quiet: ChoiceSwitch {
                enable: false,
                short: 'q',
                long: String::from("quiet"),
            },
            verbose: ChoiceSwitch {
                enable: false,
                short: 'v',
                long: String::from("verbose"),
            },
            version: ChoiceName {
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
