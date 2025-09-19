use iora::modules::cli;
use iora::modules::cli_toolset::{CliParser, CliExecutor};
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

    // Check if we're using the new CLI toolset commands
    let args: Vec<String> = std::env::args().collect();
    let is_toolset_command = args.len() > 1 && matches!(args[1].as_str(),
        "init" | "setup" | "features" | "apis" | "ai" | "blockchain" |
        "mcp" | "deploy" | "infra" | "plugins" | "profile" | "template"
    );

    if is_toolset_command {
        // Use the new CLI toolset
        let cli_app = CliParser::build_cli();
        let matches = match cli_app.try_get_matches_from(args) {
            Ok(matches) => matches,
            Err(e) => {
                eprintln!("❌ CLI parsing error: {}", e);
                process::exit(1);
            }
        };

        let command = match CliParser::parse_command(&matches) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("❌ Command parsing error: {}", e);
                process::exit(1);
            }
        };

        let mut executor = match CliExecutor::new() {
            Ok(executor) => executor,
            Err(e) => {
                eprintln!("❌ CLI executor initialization failed: {}", e);
                process::exit(1);
            }
        };

        if let Err(e) = executor.execute(command).await {
            eprintln!("❌ Command execution failed: {}", e);
            process::exit(1);
        }

        // Exit early for toolset commands - no default status needed
        return;
    } else {
        // Use the legacy CLI
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
}
