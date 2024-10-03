// use futures_util::{SinkExt, StreamExt};
// use serde_json::{json, Value};
// use std::time::Duration;
// use tokio::time::sleep;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
// use url::Url;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

// helius websocket method
// pub async fn program_subscribe(
//     url: &str,
//     program_id: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let url = Url::parse(url)?;
//     let mut retry_count = 0;
//     const MAX_RETRIES: u32 = 5;

//     loop {
//         match connect_and_subscribe(&url, program_id).await {
//             Ok(_) => {
//                 println!("Connection closed normally");
//                 break;
//             }
//             Err(e) => {
//                 eprintln!("Connection error: {}", e);
//                 retry_count += 1;
//                 if retry_count >= MAX_RETRIES {
//                     return Err("Maximum retry count reached".into());
//                 }
//                 println!("Attempting to reconnect ({}/{})", retry_count, MAX_RETRIES);
//                 sleep(Duration::from_secs(5)).await;
//             }
//         }
//     }

//     Ok(())
// }

// async fn connect_and_subscribe(
//     url: &Url,
//     program_id: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let (ws_stream, _) = connect_async(url.to_string()).await?;
//     println!("WebSocket connection established");

//     let (mut write, mut read) = ws_stream.split();

//     let subscription_message = json!({
//         "jsonrpc": "2.0",
//         "id": 420,
//         "method": "transactionSubscribe",
//         "params": [
//             {
//                 "vote": false,
//                 "failed": false,
//                 "accountInclude": [program_id]
//             },
//             {
//                 "commitment": "processed",
//                 "encoding": "base64",
//                 "transactionDetails": "full",
//                 "showRewards": true,
//                 "maxSupportedTransactionVersion": 0
//             }
//         ]
//     });

//     write
//         .send(Message::Text(subscription_message.to_string()))
//         .await?;

//     let mut last_message_time = std::time::Instant::now();

//     loop {
//         tokio::select! {
//             msg = read.next() => {
//                 match msg {
//                     Some(Ok(Message::Text(response))) => {
//                         let json_response: Value = serde_json::from_str(&response)?;
//                         println!("Received: {}", json_response);
//                         last_message_time = std::time::Instant::now();
//                     }
//                     Some(Ok(Message::Binary(bin))) => {
//                         println!("Received binary message: {:?}", bin);
//                         last_message_time = std::time::Instant::now();
//                     }
//                     Some(Err(e)) => {
//                         eprintln!("Error receiving message: {}", e);
//                         return Err(e.into());
//                     }
//                     None => {
//                         println!("WebSocket connection closed");
//                         return Ok(());
//                     }
//                     _ => {}
//                 }
//             }
//             _ = sleep(Duration::from_secs(30)) => {
//                 if last_message_time.elapsed() > Duration::from_secs(60) {
//                     println!("No message received for a long time, sending heartbeat");
//                     write.send(Message::Ping(vec![])).await?;
//                 }
//             }
//         }
//     }
// }


// rpc method
pub fn get_signatures_for_address(rpc_url: &str, address: &str) -> Result<(), Box<dyn std::error::Error>> {

    let client = RpcClient::new(rpc_url.to_string());

    let pubkey = solana_program::pubkey::Pubkey::from_str(address)?;

    let signatures = client.get_signatures_for_address(&pubkey)?;

    for signature in signatures {
        println!("signature: {}", signature.signature);
        println!("block: {}", signature.slot);
        println!("------------------------");
    }

    Ok(())
}

// 添加新的公共函数
pub async fn program_subscribe(url: &str, program_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    get_signatures_for_address(url, program_id)?;
    sleep(Duration::from_secs(5)).await;
    Ok(())
}
