use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSignatureStatusConfig;
use solana_sdk::signature::Signature;
use std::collections::HashSet;
use tokio::time::{sleep, Duration};

pub async fn monitor_pending_transactions(client: &RpcClient) {
    let mut seen_signatures: HashSet<Signature> = HashSet::new();

    loop {
        // Fetch recent signatures
        match client.get_recent_blockhash().await {
            Ok((blockhash, _)) => {
                // Get signatures for recent transactions
                let signatures = client.get_signatures_for_address(&blockhash).await;

                match signatures {
                    Ok(sig_info) => {
                        for sig in sig_info {
                            if !seen_signatures.contains(&sig.signature) {
                                println!("New transaction signature: {:?}", sig.signature);
                                seen_signatures.insert(sig.signature.clone());

                                // Check the status of the transaction
                                let status = client.get_signature_status_with_config(
                                    &sig.signature,
                                    RpcSignatureStatusConfig {
                                        commitment: Some(solana_client::rpc_config::CommitmentConfig::confirmed()),
                                        ..RpcSignatureStatusConfig::default()
                                    }
                                ).await;

                                match status {
                                    Ok(Some(status)) => {
                                        println!("Transaction status: {:?}", status);
                                        // Implement your logic for MEV extraction here
                                    },
                                    Ok(None) => {
                                        println!("Transaction not found or not processed yet.");
                                    },
                                    Err(e) => {
                                        eprintln!("Error fetching signature status: {:?}", e);
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error fetching signatures: {:?}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error fetching recent blockhash: {:?}", e);
            }
        }

        sleep(Duration::from_secs(5)).await; // Adjust the interval as needed
    }
}
