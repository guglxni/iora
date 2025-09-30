use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::error::Error;
use std::fs;
use chrono::Utc;

#[allow(dead_code)]
pub struct SolanaOracle {
    client: RpcClient,
    wallet: Keypair,
    program_id: Pubkey,
}

impl SolanaOracle {
    pub fn new(rpc_url: &str, wallet_path: &str, program_id: &str) -> Result<Self, Box<dyn Error>> {
        let client =
            RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

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

    pub async fn feed_oracle(
        &self,
        analysis: &super::analyzer::Analysis,
    ) -> Result<String, Box<dyn Error>> {
        // Use the Anchor-generated instruction to update oracle data
        // For now, we'll create a simple instruction call
        // In a production setup, this would use the proper Anchor CPI

        let oracle_data_pda = self.find_oracle_data_pda()?;
        let current_time = Utc::now().timestamp();

        // For MVP, create a simple instruction that mimics the update_data call
        // In a real implementation, this would use anchor_lang::Instruction
        let instruction_data = self.build_update_instruction_data(
            &analysis.insight,
            analysis.processed_price,
            analysis.confidence,
            &analysis.recommendation,
            current_time,
        )?;

        let instruction = solana_sdk::instruction::Instruction {
            program_id: self.program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(oracle_data_pda, false),
                solana_sdk::instruction::AccountMeta::new_readonly(self.wallet.pubkey(), true),
            ],
            data: instruction_data,
        };

        // Build and send transaction
        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
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

    /// Find the PDA for oracle data storage
    pub fn find_oracle_data_pda(&self) -> Result<Pubkey, Box<dyn Error>> {
        let seeds: &[&[u8]] = &[b"oracle-data"];
        let (pda, _) = Pubkey::find_program_address(seeds, &self.program_id);
        Ok(pda)
    }

    /// Build instruction data for update_data call
    pub fn build_update_instruction_data(
        &self,
        insight: &str,
        price: f64,
        confidence: f32,
        recommendation: &str,
        timestamp: i64,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        // This is a simplified instruction data builder
        // In a real Anchor setup, this would be generated automatically

        let mut data = Vec::new();

        // Instruction discriminator (first 8 bytes)
        // For Anchor programs, this is typically a hash of the function name
        // For "update_data", we'll use a simple discriminator
        data.extend_from_slice(&[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);

        // Serialize parameters
        // symbol (we'll use a fixed symbol for simplicity)
        let symbol = "BTC";
        data.extend_from_slice(&(symbol.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol.as_bytes());

        // price
        data.extend_from_slice(&price.to_le_bytes());

        // insight
        let truncated_insight = if insight.len() > 500 { &insight[..500] } else { insight };
        data.extend_from_slice(&(truncated_insight.len() as u32).to_le_bytes());
        data.extend_from_slice(truncated_insight.as_bytes());

        // confidence
        data.extend_from_slice(&confidence.to_le_bytes());

        // recommendation
        let truncated_rec = if recommendation.len() > 10 { &recommendation[..10] } else { recommendation };
        data.extend_from_slice(&(truncated_rec.len() as u32).to_le_bytes());
        data.extend_from_slice(truncated_rec.as_bytes());

        // timestamp
        data.extend_from_slice(&timestamp.to_le_bytes());

        Ok(data)
    }

    /// Initialize the oracle account
    pub async fn initialize_oracle(&self) -> Result<String, Box<dyn Error>> {
        let oracle_data_pda = self.find_oracle_data_pda()?;

        // Create initialize instruction
        let instruction_data = vec![0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90]; // Initialize discriminator

        let instruction = solana_sdk::instruction::Instruction {
            program_id: self.program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(oracle_data_pda, false),
                solana_sdk::instruction::AccountMeta::new(self.wallet.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
            data: instruction_data,
        };

        // Build and send transaction
        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.wallet.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(signature.to_string())
    }
}
