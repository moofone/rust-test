// use dashmap::DashMap;
extern crate lazy_static;
use std::collections::VecDeque;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Candle {
    pub open_time: i64,
    pub close_time: i64,
    pub tf: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub symbol: String,
    pub last_update: i64,
    pub hlc3: f64,
}

pub(crate) struct CandlesVecDeque {
    candles: VecDeque<Candle>,
    symbol: String,
    tf: String,
}

impl CandlesVecDeque {
    pub fn new(symbol: String, tf: String) -> Self {
        CandlesVecDeque {
            candles: VecDeque::new(),
            symbol,
            tf,
        }
    }
    // pub fn new(key: Key) -> Self {
    //     CandlesVecDeque {
    //         candles: RwLock::new(VecDeque::new()),
    //         symbol: key.symbol.to_owned(),
    //         tf: key.tf,
    //     }
    // }

    fn len(&self) -> usize {
        // Acquire a read lock on the candles field
        //let candles = self.candles.read().unwrap();
        return self.candles.len();
    }

    fn insert(&mut self, candle: Candle) {
        // Acquire a write lock on the candles field
        // let mut candles = self.candles.unwrap();
        self.candles.push_back(candle);
    }
}

pub struct Key {
    pub(crate) symbol: String,
    pub(crate) tf: String,
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        self.tf.hash(state);
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.tf == other.tf
    }
}

impl Eq for Key {}

impl fmt::Debug for CandlesVecDeque {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_candle = if self.candles.is_empty() {
            "None".to_string()
        } else {
            format!("{:?}", self.candles.back().unwrap())
        };
        write!(
            f,
            "CandlesVecDeque {{ symbol: {}, tf: {}, len: {}, last_candle: {} }}",
            self.symbol,
            self.tf,
            self.candles.len(),
            last_candle
        )
    }
}

pub struct FuturesWebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(FuturesWebsocketEvent) -> Result<()> + 'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub open_time: i64,

    #[serde(rename = "T")]
    pub close_time: i64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_asset_volume: String,

    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,

    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,

    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}
