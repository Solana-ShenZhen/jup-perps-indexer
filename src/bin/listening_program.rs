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
use jup_perps_indexer::websocket::program_subscribe::program_subscribe;
use dotenv::dotenv;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let url = dotenv::var("RPC_URL").unwrap();
    let program_id = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";

    loop {
        match program_subscribe(&url, program_id).await {
            Ok(_) => println!("Program subscription successful, continuing to listen..."),
            Err(e) => {
                eprintln!("Program subscription failed: {}", e);
                println!("Retrying in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
