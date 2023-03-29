use clap::Parser;

#[derive(Debug, Parser, Default)]
#[command(about, version, no_binary_name(true))]
struct Cli {
    #[arg(long, short, default_value_t = String::from("Default endpoint"))]
    /// RPC endpoint of the node that this wallet will connect to
    endpoint: String,

    #[arg(long, short)]
    refresh_rate: Option<u32>,
}

fn main() {
    let input = vec!["--endpoint", "localhost:8000"];

    let c = Cli::parse_from(input);

    println!("{:?}", c);
}
