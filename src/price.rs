/// Open-High-Low-Close (OHLC)
#[derive(Default)]
pub struct Price {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

/// Popular trends in price
///
/// TODO: may contain array of prices or current price
///
///
/// `Custom` field allows for
/// endless trends to be created based on
/// needs
pub struct Trends {
    pub hourly: Price,
    pub daily: Price,
    pub weekly: Price,
    pub monthly: Price,
    pub yearly: Price,
    pub custom: Price,
}

impl Price {
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
    pub fn show_ohlc(&self, precision: usize) {
        print!(" [Open] - {1:.0$}", precision, self.open);
        print!("  [High] - {1:.0$}", precision, self.high);
        print!("  [Low] - {1:.0$}", precision, self.low);
        println!("  [Close] - {1:.0$}", precision, self.close);
        println!("-------------------------------------------------------------");
    }

    /// Print the direction and strength of a candle
    pub fn show_traits(&self) {
        let direction = self.to_direction();
        let direction = match direction {
            'U' => "⬆️",
            'D' => "⬇️",
            _ => "⬅️➡️",
        };
        println!("Direction: {}", direction);

        let strong_indicator = if self.is_strong(None) { "💪" } else { "❌" };
        println!("Strong: {}", strong_indicator);

        println!();
    }

    /// Returns the percentage of movement in price
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: Price = Price {
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
    /// Returns the raw point of movement in price
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: Price = Price {
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

    /// Returns the total range of movement in price
    ///
    /// # Examples
    ///
    /// ```
    /// let bar: Price = Price {
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

    /// Returns the direction of price movement
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
    /// let bar: Price = Price {
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
    /// let bar: Price = Price {
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
    /// let bar: Price = Price {
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
    /// let bar: Price = Price {
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
    /// let bar: Price = Price {
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

//////// PRICE //////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_weak() {
        let bar1: Price = Price {
            open: 7.0,
            close: 5.0,
            low: 2.5,
            high: 13.0,
        };
        let bar2: Price = Price {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: Price = Price {
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
        let bar1: Price = Price {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: Price = Price {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: Price = Price {
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
        let bar1: Price = Price {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: Price = Price {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: Price = Price {
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
        let bar1: Price = Price {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        assert_eq!(bar1.to_range(), 12.5);
    }

    #[test]
    fn test_to_point() {
        let bar1: Price = Price {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: Price = Price {
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
        let bar1: Price = Price {
            open: 5.0,
            close: 1.0,
            low: 0.5,
            high: 13.0,
        };
        let bar2: Price = Price {
            open: 1.0,
            close: 5.0,
            low: 5.5,
            high: 6.0,
        };
        let bar3: Price = Price {
            open: 1.0,
            close: 1.0,
            low: 0.6,
            high: 1.5454,
        };
        assert_eq!(bar1.to_percent(), -0.8);
        assert_eq!(bar2.to_percent(), 4.0);
        assert_eq!(bar3.to_percent(), 0.0);
    }
}
