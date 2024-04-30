use crate::color::{GREEN, RED, RESET, YELLOW};
use std::io::Write;

/// Open-High-Low-Close (OHLC)
#[derive(Default)]
pub struct OHLC {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

/// Popular trends in OHLC
///
/// `timeframe` field allows to store
/// specifed timeframe
pub struct History {
    pub hourly: Vec<OHLC>,
    pub daily: Vec<OHLC>,
    pub weekly: Vec<OHLC>,
    pub monthly: Vec<OHLC>,
    pub yearly: Vec<OHLC>,
    pub timeframe: Vec<OHLC>,
}

impl History {
    pub fn new(
        hourly: Vec<OHLC>,
        daily: Vec<OHLC>,
        weekly: Vec<OHLC>,
        monthly: Vec<OHLC>,
        yearly: Vec<OHLC>,
        timeframe: Vec<OHLC>,
    ) -> Self {
        return Self {
            hourly,
            daily,
            weekly,
            monthly,
            yearly,
            timeframe,
        };
    }
}

impl OHLC {
    pub fn new(open: f32, high: f32, low: f32, close: f32) -> Self {
        return Self {
            open,
            high,
            low,
            close,
        };
    }

    /// Print the OHLC
    ///
    /// To prevent losing precision specfiy
    /// the level of `precision` after decimal needed
    ///
    pub fn show_ohlc<T: Write>(&self, mut output: T, precision: usize) {
        let _ = write!(output, " [Open] - {1:.0$}", precision, self.open);
        let _ = write!(output, "  [High] - {1:.0$}", precision, self.high);
        let _ = write!(output, "  [Low] - {1:.0$}", precision, self.low);
        let _ = writeln!(output, "  [Close] - {1:.0$}", precision, self.close);
        let sep = "-".repeat(61);
        let _ = writeln!(output, "{}", sep);
    }

    /// Write the direction and strength of a candle
    pub fn show_direction<T: Write>(&self, mut output: T) {
        let _ = write!(output, "Direction: ");
        let _ = match self.to_direction() {
            'U' => writeln!(output, "{}⬆ Up{}", GREEN, RESET),
            'D' => writeln!(output, "{}⬇ Down{}", RED, RESET),
            _ => writeln!(output, "{}⬌ Side{}", YELLOW, RESET),
        };
    }

    /// Wri the strength of a candle
    pub fn show_strong<T: Write>(&self, mut output: T) {
        let strong_indicator = if self.is_strong(None) { '✅' } else { '❌' };
        let _ = writeln!(output, "Strong: {}", strong_indicator);
    }

    /// Returns the percentage of movement in OHLC
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_bot_wick(), -0.8)
    /// ```
    pub fn to_percent(&self) -> f32 {
        (self.close - self.open) / self.open
    }
    /// Returns the raw point of movement in OHLC
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_point(), -4.0)
    /// ```
    pub fn to_point(&self) -> f32 {
        self.close - self.open
    }

    /// Returns the total range of movement in OHLC
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_range(), 12.5)
    /// ```
    pub fn to_range(&self) -> f32 {
        self.high - self.low
    }

    /// Returns the direction of OHLC movement
    ///
    /// Movement can be one of the following:
    ///
    /// `'D'` = Downward or Downtrend
    ///
    /// `'U'` = Upward or Uptrend
    ///
    /// `'N'` = Neutral or Flat
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_direction(), 'D')
    /// ```
    pub fn to_direction(&self) -> char {
        if self.close > self.open {
            return 'U';
        } else if self.close < self.open {
            return 'D';
        } else {
            return 'N';
        }
    }

    /// Returns the range of the top wick of a candle bar
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_top_wick(), 8.0)
    /// ```
    pub fn to_top_wick(&self) -> f32 {
        let dir: char = self.to_direction();
        if dir == 'D' {
            return self.high - self.open;
        } else {
            return self.high - self.close;
        }
    }

    /// Returns the range of the bottom wick of a candle bar
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.to_bot_wick(), 0.5)
    /// ```
    pub fn to_bot_wick(&self) -> f32 {
        let dir: char = self.to_direction();
        if dir == 'D' {
            return self.close - self.low;
        } else {
            return self.open - self.low;
        }
    }

    /// Returns `true` if the candle is relatively weak based on
    /// the gauge level otherwise `false`
    ///
    /// Default level is: `1.0`
    ///
    /// Usually seen when candle wicks in current direction is
    /// relatively stronger than the body of a candle
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.is_weak(), false)
    /// ```
    pub fn is_weak(&self, gauge: Option<f32>) -> bool {
        let gauge: f32 = gauge.or(Some(1.0)).unwrap();
        let body: f32 = self.close - self.open;
        if body == 0.0 {
            return false;
        } else if body > 0.0 {
            return self.to_top_wick() / body > gauge;
        } else {
            return self.to_bot_wick() / body < -gauge;
        };
    }

    /// Returns `true` if the candle is relatively strong based on
    /// the gauge level otherwise `false`
    ///
    /// Default level is: `2.0`
    ///
    /// Usually seen when the body of
    /// a candle is strong compared to its opposing direction
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: OHLC = OHLC {
    ///     open: 5.0,
    ///     close: 1.0,
    ///     low: 0.5,
    ///     high: 13.0,
    ///  };
    /// assert!eq(bar.is_strong(), true)
    /// ```
    pub fn is_strong(&self, gauge: Option<f32>) -> bool {
        let gauge: f32 = gauge.or(Some(2.0)).unwrap();
        let body: f32 = self.close - self.open;
        if body == 0.0 {
            return false;
        } else if body > 0.0 {
            return body / self.to_top_wick() > gauge;
        } else {
            return body / self.to_bot_wick() < -gauge;
        };
    }
}

//////// OHLC //////////
#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_is_weak() {
        let bar1: OHLC = OHLC {
            open: 7.0,
            close: 5.0,
            low: 2.5,
            high: 13.0,
        };
        let bar2: OHLC = OHLC {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: OHLC = OHLC {
            open: 5.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        assert_eq!(bar1.is_weak(None), true);
        assert_eq!(bar2.is_weak(Some(1.0)), false);
        assert_eq!(bar3.is_weak(None), false);
    }

    #[test]
    fn test_is_strong() {
        let bar1: OHLC = OHLC {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: OHLC = OHLC {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: OHLC = OHLC {
            open: 5.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        assert_eq!(bar1.is_strong(None), true);
        assert_eq!(bar2.is_strong(Some(3.0)), true);
        assert_eq!(bar3.is_strong(None), false);
    }

    #[test]
    fn test_to_direction() {
        let bar1: OHLC = OHLC {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: OHLC = OHLC {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: OHLC = OHLC {
            open: 5.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        assert_eq!(bar1.to_direction(), 'D');
        assert_eq!(bar2.to_direction(), 'U');
        assert_eq!(bar3.to_direction(), 'N');
    }

    #[test]
    fn test_to_range() {
        let bar1: OHLC = OHLC {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        assert_eq!(bar1.to_range(), 12.5);
    }

    #[test]
    fn test_to_point() {
        let bar1: OHLC = OHLC {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: OHLC = OHLC {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        assert_eq!(bar1.to_point(), -4.0);
        assert_eq!(bar2.to_point(), 4.0);
    }

    #[test]
    fn test_to_percent() {
        let bar1: OHLC = OHLC {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: OHLC = OHLC {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: OHLC = OHLC {
            open: 1.0,
            close: 1.0,
            low: 0.6,
            high: 1.5454,
        };
        assert_eq!(bar1.to_percent(), -0.8);
        assert_eq!(bar2.to_percent(), 4.0);
        assert_eq!(bar3.to_percent(), 0.0);
    }

    #[test]
    fn test_show_direction() {
        // mising test for neutral and down
        let bar1: OHLC = OHLC {
            open: 3.0,
            close: 5.0,
            low: 1.6,
            high: 8.54,
        };
        let mut output: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::<u8>::new());
        bar1.show_direction(&mut output);
        assert_eq!(
            output.into_inner(),
            b"Direction: \x1b[32m\xE2\xAC\x86 Up\x1b[0m\n"
        );
    }

    #[test]
    fn test_show_ohlc() {
        let bar1: OHLC = OHLC::new(4.03, 10.0, 1.0, 6.0);
        let mut output: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::<u8>::new());
        bar1.show_ohlc(&mut output, 2);
        assert_eq!(
            output.into_inner(),
            b" [Open] - 4.03  [High] - 10.00  [Low] - 1.00  [Close] - 6.00\n-------------------------------------------------------------\n"
        );
    }

    #[test]
    fn test_show_strong() {
        // missing test for green check
        let bar1: OHLC = OHLC::new(4.03, 10.0, 1.0, 6.0);
        let mut output: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::<u8>::new());
        bar1.show_strong(&mut output);
        assert_eq!(output.into_inner(), b"Strong: \xE2\x9D\x8C\n");
    }
}
