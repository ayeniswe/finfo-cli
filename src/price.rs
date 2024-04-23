// All trends of data points now and (future) will need
// to track OHLC
struct Price {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}
impl Price {
    fn to_percent(&self) -> f32 {
        (self.close - self.open) / self.open
    }
    fn to_point(&self) -> f32 {
        self.close - self.open
    }
}

// Popular price trends
struct Trends {
    daily: Price,
    hourly: Price,
    weekly: Price,
    monthly: Price,
    yearly: Price,
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
