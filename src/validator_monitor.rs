use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::time::{sleep, Duration};

pub async fn monitor_validator(client: &RpcClient, validator_pubkey: &str) {
    let pubkey = Pubkey::from_str(validator_pubkey).expect("Invalid validator public key");

    loop {
        match client.get_account_with_commitment(&pubkey, CommitmentConfig::confirmed()).await {
            Ok(account_info) => {
                println!("Validator account data: {:?}", account_info);
                // You can implement additional logic to analyze the validator's performance,
                // such as checking the stake, rewards, or any other relevant metrics.
            },
            Err(e) => {
                eprintln!("Error fetching validator account info: {:?}", e);
            }
        }

        sleep(Duration::from_secs(30)).await; // Adjust the interval as needed
    }
}
