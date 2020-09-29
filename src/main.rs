use structopt::StructOpt;

use mass_http::*;

#[derive(StructOpt)]
pub struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    address_file: std::path::PathBuf,
    /// How long until the connection drops
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    time_out: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let paths: Vec<String> = read_lines(args.address_file)?;
    fetch(paths, args.time_out.into()).await?;
    Ok(())
}
