// use jup_perps_indexer::websocket::program_subscribe::program_subscribe;

// #[tokio::main]
// async fn main() {
//     // Solana WebSocket URL
//     let url = "wss://mainnet.helius-rpc.com/?api-key=2e453ba4-6a97-4aaf-9651-a61949aca79b";

//     // Program ID
//     let program_id = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";

//     // Call program_subscribe function from websocket module
//     match program_subscribe(url, program_id).await {
//         Ok(_) => println!("Program subscription completed successfully"),
//         Err(e) => eprintln!("Error occurred during program subscription: {}", e),
//     }
// }

mod websocket;

use jup_perps_indexer::websocket::program_subscribe::program_subscribe;

#[tokio::main]
async fn main() {
    let url = "https://mainnet.helius-rpc.com/?api-key=f46e7c57-a4d4-43b0-b65b-1f287e2380cb";
    let program_id = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";
    
    loop {
        match program_subscribe(url, program_id).await {
            Ok(_) => println!("程序订阅成功，继续监听..."),
            Err(e) => {
                eprintln!("程序订阅失败: {}", e);
                println!("5秒后重试...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
