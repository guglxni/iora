use iora::modules::config;
use std::process;

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration
    match config::init_config() {
        Ok(_) => {
            println!("✅ I.O.R.A. configuration loaded successfully!");
        }
        Err(e) => {
            eprintln!("❌ Configuration error: {}", e);
            eprintln!("Please check your .env file and ensure all required environment variables are set.");
            process::exit(1);
        }
    }

    // Get configuration for demonstration
    match config::get_config() {
        Ok(cfg) => {
            println!("🚀 I.O.R.A. Intelligent Oracle Rust Assistant");
            println!("📍 Solana RPC: {}", cfg.solana_rpc_url());
            println!("👛 Wallet Path: {}", cfg.solana_wallet_path().display());
                                println!("🤖 Gemini AI: Configured");
                    println!("🔍 Typesense: {}", cfg.typesense_url());
                    println!("🎯 Ready for oracle operations!");
        }
        Err(e) => {
            eprintln!("❌ Failed to access configuration: {}", e);
            process::exit(1);
        }
    }
}
