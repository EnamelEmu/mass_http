use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use futures::stream::StreamExt;
use reqwest::Client;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path_file: std::path::PathBuf,
    /// How long until the connection drops
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    time_out: u32
}


fn read_lines(path_file: std::path::PathBuf) -> std::io::Result<Vec<String>> {
    let file = File::open(&path_file)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let paths: Vec<String> = read_lines(args.path_file)?;
    let client = Client::builder().timeout(Duration::from_secs(args.time_out.into()))
	.build()?;
    let fetches = futures::stream::iter(
	paths.into_iter().map(|path| {
	    let send_fut = client.head(&path).send();
            async move {
		match send_fut.await {
                    Ok(resp) => {
			if resp.status().is_success() {
			    println!("SUCCESS: from {}", path);
			}
			else {
			    eprintln!("UNAUTHORIZED/REDIRECTED: from {}", path);
			}
		    }
		    Err(_) => eprintln!("DOWN {}", path),
		}
	    }
	})
    ).buffer_unordered(100).collect::<Vec<()>>();
    fetches.await;
    Ok(())
}