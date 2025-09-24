use dotenv;

mod modules {
    pub mod cli;
    pub mod fetcher;
    pub mod rag;
    pub mod analyzer;
    pub mod solana;
}

fn main() {
    dotenv::dotenv().ok();
    println!("Hello, world!");
}
