mod transaction_monitor;
mod validator_monitor;

use solana_client::rpc_client::RpcClient;
use tokio;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url);

    // Start monitoring all pending transactions
    tokio::spawn(async move {
        transaction_monitor::monitor_pending_transactions(&client).await;
    });

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
