use crate::price::History;
/// The central point of
/// all data for a financial symbol
struct Ticker<'a> {
    history: History, // 52 week range should be implement in the History struct
    description: &'a str,
    sector: Vec<&'a str>,
    exchange: Vec<&'a str>,
    // financial: Financial,
    // charting: Charting,
    price: f32,
    volume: f32,
    // news: News,
    // alert: Alert,
    // event: Event,
}

impl<'a> Ticker<'a> {
    pub fn new(
        history: History,
        description: &'a str,
        sector: Vec<&'a str>,
        exchange: Vec<&'a str>,
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
