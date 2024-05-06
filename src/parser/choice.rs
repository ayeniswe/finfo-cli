/// The choice states that are made
pub struct Choice {
    pub verbose: bool,
    pub offline: bool,
    pub nocache: bool,
    pub quiet: bool,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            verbose: false,
            offline: false,
            nocache: false,
            quiet: false,
        }
    }
}

impl Choice {
    /// Verify if an argument is a valid choice
    /// # Examples
    ///
    /// ```
    /// assert_eq!(is_choice("--hlpre", "h", "help"), false)
    /// ```
    pub fn is_choice(&self, arg: &str, short: &str, long: &str) -> bool {
        let choice: &str = arg.trim_start_matches("-");
        return arg.starts_with("--") && long.starts_with(choice)
            || arg.starts_with("-") && short == choice;
    }
}
