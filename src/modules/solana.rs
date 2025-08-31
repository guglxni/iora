use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::error::Error;
use std::fs;

pub struct SolanaOracle {
    client: RpcClient,
    wallet: Keypair,
    program_id: Pubkey,
}

impl SolanaOracle {
    pub fn new(rpc_url: &str, wallet_path: &str, program_id: &str) -> Result<Self, Box<dyn Error>> {
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

        // Load wallet keypair from file
        let wallet_data = fs::read(wallet_path)?;
        let wallet = Keypair::from_bytes(&wallet_data)?;

        let program_id = program_id.parse()?;

        Ok(Self {
            client,
            wallet,
            program_id,
        })
    }

    pub async fn feed_oracle(&self, _analysis: &super::analyzer::Analysis) -> Result<String, Box<dyn Error>> {
        // For MVP, we'll create a simple transaction to store data
        // In a full implementation, this would interact with a custom Solana program

        // Create a simple memo instruction to store the analysis
        let memo_instruction = system_instruction::create_account(
            &self.wallet.pubkey(),
            &Pubkey::new_unique(), // New account for data storage
            self.client.get_minimum_balance_for_rent_exemption(0)?,
            0,
            &Pubkey::default(), // System program
        );

        // Build and send transaction
        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[memo_instruction],
            Some(&self.wallet.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(signature.to_string())
    }

    pub fn get_balance(&self) -> Result<u64, Box<dyn Error>> {
        let balance = self.client.get_balance(&self.wallet.pubkey())?;
        Ok(balance)
    }
}
