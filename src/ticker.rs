use crate::price::History;
/// The central point of
/// all data for a financial symbol
struct Ticker {
    history: History, // 52 week range should be implement in the History struct
    description: String,
    sector: Vec<String>,
    exchange: Vec<String>,
    // financial: Financial,
    // charting: Charting,
    price: f32,
    volume: f32,
    // news: News,
    // alert: Alert,
    // event: Event,
}

impl Ticker {
    pub fn new(
        history: History,
        description: String,
        sector: Vec<String>,
        exchange: Vec<String>,
        // financial: Financial,
        // charting: Charting,
        price: f32,
        volume: f32,
        // news: News,
        // alert: Alert,
        // event: Event,
    ) -> Self {
        Self {
            history,
            description,
            sector,
            exchange,
            // financial,
            // charting,
            price,
            volume,
            // news,
            // alert,
            // event,
        }
    }
}
