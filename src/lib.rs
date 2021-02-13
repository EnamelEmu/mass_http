use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use futures::stream::StreamExt;
use reqwest::Client;
use std::time::Duration;
use colored::*;

pub fn read_lines(address_file: std::path::PathBuf) -> std::io::Result<Vec<String>> {
    let file = File::open(&address_file)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

pub async fn fetch(paths: Vec<String>, t_out: u64) -> Result<(), Box<dyn std::error::Error>>
{
    let client = Client::builder()
	.timeout(Duration::from_secs(t_out))
	.user_agent("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0")
	.build()?;
    let fetches = futures::stream::iter(
	paths.into_iter().map(|path| {
	    let send_fut = client.get(&path).send();
            async move {
		match send_fut.await {
                    Ok(resp) => {
			if resp.status().is_success() {
			    println!("{}: from {}", resp.status().as_str().green(), path);
			}
			else {
			    eprintln!("{}: from {}", resp.status().as_str().red(), path);
			}
		    }
		    Err(err) => {
			if err.is_connect() {
			    eprintln!("{}: {}", "REFUSED".red(), path);
			}
			else {
			    eprintln!("{} {}", "TIMEOUT".red(), path);
			}
		    }
		}
	    }
	})
    ).buffer_unordered(100).collect::<Vec<()>>();
    fetches.await;
    Ok(())
}
