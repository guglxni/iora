use iora::modules::cli;
use iora::modules::config;
use std::process;

#[tokio::main]
async fn main() {
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

    // Parse CLI arguments
    let cli_app = cli::build_cli();
    let matches = cli_app.get_matches();

    // Handle CLI commands
    if let Err(e) = cli::handle_cli_command(&matches).await {
        eprintln!("❌ Command execution failed: {}", e);
        process::exit(1);
    }

    // If no subcommand was used, show the default status
    if matches.subcommand().is_none() {
        match config::get_config() {
            Ok(cfg) => {
                println!("\n🚀 I.O.R.A. Intelligent Oracle Rust Assistant");
                println!("📍 Solana RPC: {}", cfg.solana_rpc_url());
                println!("👛 Wallet Path: {}", cfg.solana_wallet_path().display());
                println!("🤖 Gemini AI: Configured");
                println!("🔍 Typesense: {}", cfg.typesense_url());
                println!("🗄️  Intelligent Cache: Enabled");
                println!("🎯 Ready for multi-API crypto data fetching!");
                println!("\n💡 Quick commands:");
                println!("   iora config status              # Check API configuration");
                println!("   iora cache status               # Check cache status");
                println!("   iora process price -s BTC       # Get normalized price data");
                println!("   iora historical fetch -s BTC    # Fetch historical data");
                println!("   iora analytics dashboard        # View analytics dashboard");
                println!("   iora resilience status          # Check API resilience");
                println!("   iora query -s BTC               # Query Bitcoin price");
                println!("   iora oracle -s BTC              # Run complete AI oracle pipeline");
                println!("   iora oracle -s ETH --skip-feed  # Run analysis without Solana feed");
                println!("   iora cache warm symbols         # Warm cache with popular symbols");
                println!("   iora analytics usage            # View API usage metrics");
                println!("   iora analytics recommend        # Get optimization recommendations");
                println!("   iora health status              # Check API health status");
                println!("   iora health monitor             # Start continuous health monitoring");
            }
            Err(e) => {
                eprintln!("❌ Failed to access configuration: {}", e);
                process::exit(1);
            }
        }
    }
}
