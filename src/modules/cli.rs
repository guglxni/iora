use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("iora")
        .version("0.1.0")
        .author("IORA Dev Team <dev@iora.project>")
        .about("Intelligent Oracle Rust Assistant")
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("QUERY")
                .help("The data query to process")
                .required(true),
        )
        .arg(
            Arg::new("gemini_key")
                .short('g')
                .long("gemini-key")
                .value_name("KEY")
                .help("Gemini API key")
                .required(true),
        )
        .arg(
            Arg::new("wallet_path")
                .short('w')
                .long("wallet-path")
                .value_name("PATH")
                .help("Path to Solana wallet keypair")
                .required(true),
        )
}
