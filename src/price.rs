// All trends of data points now and (future) will need
// to track OHLC
struct Price {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}

// Popular price trends
struct Trends {
    hourly: Price,
    daily: Price,
    weekly: Price,
    monthly: Price,
    yearly: Price,
    custom: Price // Custom trend will be based on minutes
}

impl Price {
    fn to_percent(&self) -> f32 {
        (self.close - self.open) / self.open
    }
    fn to_point(&self) -> f32 {
        self.close - self.open
    }
    fn to_range(&self) -> f32 {
        self.high - self.low
    }
    fn to_direction(&self) -> char {
        if self.close > self.open {
            return 'U'
        }
        else if self.close < self.open {
            return 'D'
        }
        else {
            return 'N'
        }
    }
}

//////// PRICE //////////
#[test]
fn test_to_direction() {
    let price_ex1: Price = Price {
        open: 5.0,
        close: 1.0,
        low: 0.5,
        high: 13.0,
    };
    let price_ex2: Price = Price {
        open: 1.0,
        close: 5.0,
        low: 5.5,
        high: 1.0,
    };
    let price_ex3: Price = Price {
        open: 5.0,
        close: 5.0,
        low: 5.5,
        high: 1.0,
    };
    assert_eq!(price_ex1.to_direction(), 'D');
    assert_eq!(price_ex2.to_direction(), 'U');
    assert_eq!(price_ex3.to_direction(), 'N');
}

#[test]
fn test_to_range() {
    let price_ex1: Price = Price {
        open: 5.0,
        close: 1.0,
        low: 0.5,
        high: 13.0,
    };
    let price_ex2: Price = Price {
        open: 1.0,
        close: 5.0,
        low: 5.5,
        high: 1.0,
    };
    assert_eq!(price_ex1.to_range(), 12.5);
    assert_eq!(price_ex2.to_range(), -4.5);
}

#[test]
fn test_to_point() {
    let price_ex1: Price = Price {
        open: 5.0,
        close: 1.0,
        low: 0.5,
        high: 13.0,
    };
    let price_ex2: Price = Price {
        open: 1.0,
        close: 5.0,
        low: 5.5,
        high: 1.0,
    };
    assert_eq!(price_ex1.to_point(), -4.0);
    assert_eq!(price_ex2.to_point(), 4.0);
}

#[test]
fn test_to_percent() {
    let price_ex1: Price = Price {
        open: 5.0,
        close: 1.0,
        low: 0.5,
        high: 13.0,
    };
    let price_ex2: Price = Price {
        open: 1.0,
        close: 5.0,
        low: 5.5,
        high: 1.0,
    };
    let price_ex3: Price = Price {
        open: 1.0,
        close: 1.0,
        low: 2.67,
        high: 1.5454,
    };
    assert_eq!(price_ex1.to_percent(), -0.8);
    assert_eq!(price_ex2.to_percent(), 4.0);
    assert_eq!(price_ex3.to_percent(), 0.0);
}
