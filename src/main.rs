use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

// mod ipc {
//     use super::*;

//     pub type Req = Market;
//     pub type Resp = Vec<Event>;

//     pub type Responder = oneshot::Sender<Resp>;
//     pub type Tx = mpsc::UnboundedSender<(Req, Responder)>;
//     pub type Rx = mpsc::UnboundedReceiver<(Req, Responder)>;

//     pub async fn request(client: &Tx, req: Req) -> Resp {
//         let (tx, rx) = oneshot::channel();
//         client.unbounded_send((req, tx)).unwrap();
//         rx.await.expect("ipc oneshot")
//     }

//     pub async fn respond(tx: Responder, resp: Resp) -> std::result::Result<(), Resp> {
//         tx.send(resp)
//     }
// }

const SYMBOLS: [&'static str; 44] = [
    "ADAUSDT",
    "BTCUSDT",
    "ADAUSDT",
    "AAVEUSDT",
    "ATOMUSDT",
    "ALGOUSDT",
    "AVAXUSDT",
    "BCHUSDT",
    "BNBUSDT",
    "COMPUSDT",
    "C98USDT",
    "CHZUSDT",
    "CRVUSDT",
    "DOTUSDT",
    "DOGEUSDT",
    "EGLDUSDT",
    "EOSUSDT",
    "ETCUSDT",
    "ETHUSDT",
    "FILUSDT",
    "FTMUSDT",
    "GRTUSDT",
    "HBARUSDT",
    "IOTAUSDT",
    "LINKUSDT",
    "LTCUSDT",
    "THETAUSDT",
    "MANAUSDT",
    "MKRUSDT",
    "MATICUSDT",
    "NEARUSDT",
    "RVNUSDT",
    "ONEUSDT",
    "OMGUSDT",
    "SANDUSDT",
    "SOLUSDT",
    "SRMUSDT",
    "STORJUSDT",
    "UNIUSDT",
    "XLMUSDT",
    "XTZUSDT",
    "XRPUSDT",
    "ZECUSDT",
    "VETUSDT",
];
const TFS: [&'static str; 6] = ["1m", "5m", "15m", "30m", "1h", "4h"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let endpoints: Vec<String> = {
        let mut endpoints = Vec::new();
        for symbol in SYMBOLS.iter() {
            for tf in TFS.iter() {
                endpoints.push(format!("{}@kline_{}", symbol.to_lowercase(), tf));
            }
        }
        endpoints
    };

    println!("endpoints:{:?} len:{}", endpoints, endpoints.len());
    const MAX_BINANCE_STREAMS_PER_WEBSOCKET: usize = 100;

    // let (stdin_tx, stdin_rx) = futures::channel::mpsc::unbounded();
    // tokio::spawn(read_stdin(stdin_tx));

    for chunk in endpoints.chunks(MAX_BINANCE_STREAMS_PER_WEBSOCKET) {
        // let chunk = chunk.to_vec(); // Make a copy of the chunk
        let url_string = format!("wss://stream.binance.com/ws/{}", chunk.join("/"));
        let url = Url::parse(&url_string).unwrap();

        let (stdin_tx, stdin_rx) = futures::channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!(
            "WebSocket handshake has been successfully completed on chunk len {} {:?}",
            chunk.len(),
            chunk
        );

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    Ok(())
}

async fn read_stdin(tx: futures::channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
