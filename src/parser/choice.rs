#[derive(Default)]
pub(crate) struct ChoiceFlag {
    pub(crate) short: char,
    pub(crate) long: String,
}

/// ``Choice::Switch`` have the capability to be enabled/disabled
pub(crate) enum Choice {
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
    /// Convert `Choice` to `ChoiceFlag`
    pub(crate) fn _to_flag(&self) -> ChoiceFlag {
        match self {
            Choice::Switch { short, long, .. } => ChoiceFlag {
                short: *short,
                long: long.to_string(),
            },
            Choice::Name { short, long } => ChoiceFlag {
                short: *short,
                long: long.to_string(),
            },
            // _ =>
        }
    }

    /// Set the state of choice to true
    ///
    /// The state is true if `enabled` or false if `disabled`
    ///
    /// # Examples
    ///
    /// ```
    /// let choice: choice::ChoiceGlossary = choice::ChoiceGlossary::new();
    /// choice.verbose.enable()
    /// assert_eq!(choice.verbose.get_state().unwrap, true)
    /// ```
    pub(crate) fn enable(&mut self) -> Result<(), String> {
        match self {
            Self::Switch { enable, .. } => {
                *enable = true;
                Ok(())
            }
            _ => Err(String::from(
                "Choice does not have a switch. Try defining choice type as `Choice::Switch`",
            )),
        }
    }

    /// Get the state the choice is in.
    ///
    /// The state is true if `enabled` or false if `disabled`
    pub(crate) fn get_state(&self) -> Result<bool, String> {
        match self {
            Self::Switch { enable, .. } => Ok(*enable),
            _ => Err(String::from(
                "Choice does not have a switch. Try defining choice type as `Choice::Switch`",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_state() {
        let choice = Choice::Name {
            short: 'h',
            long: String::from("help"),
        };
        assert_eq!(
            choice.get_state().unwrap_err(),
            "Choice does not have a switch. Try defining choice type as `Choice::Switch`"
        );
    }

    #[test]
    fn test_enable() {
        let mut choice = Choice::Switch {
            enable: false,
            short: 'h',
            long: String::from("help"),
        };
        _ = choice.enable();
        assert_eq!(choice.get_state().unwrap(), true);

        choice = Choice::Name {
            short: 'h',
            long: String::from("help"),
        };
        assert_eq!(
            choice.enable().unwrap_err(),
            "Choice does not have a switch. Try defining choice type as `Choice::Switch`"
        );
    }
}
